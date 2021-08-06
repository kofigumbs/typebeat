#![feature(array_methods)]

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use anyhow::Result;
use crossbeam::atomic::AtomicCell;
use miniaudio::{Device, DeviceConfig, DeviceType, Format, Frames, FramesMut};
use serde::Deserialize;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcRequest, RpcResponse, WebViewBuilder};

use effects::{Bus, FaustDsp};
use samples::Sample;

mod effects;
mod samples;

const VOICE_COUNT: i32 = 5;
const TRACK_COUNT: i32 = 15;

#[derive(Clone, Copy)]
enum SampleType {
    File,
    Live,
    LiveRecord,
    LivePlay,
}
impl From<i32> for SampleType {
    fn from(value: i32) -> Self {
        match value {
            x if x <= 0 => SampleType::File,
            x if x == 1 => SampleType::Live,
            x if x == 2 => SampleType::LiveRecord,
            _ /* x > */ => SampleType::LivePlay,
        }
    }
}

struct Track {
    file_sample: Sample,
    sample_type: AtomicCell<SampleType>,
}

#[derive(Default)]
struct Parameter<const MIN: i32, const MAX: i32>(AtomicCell<i32>);
impl<const MIN: i32, const MAX: i32> Parameter<{ MIN }, { MAX }> {
    fn store(&self, value: i32) {
        self.0.store(value.clamp(MIN, MAX));
    }
    fn load(&self) -> i32 {
        self.0.load()
    }
    fn nudge(&self, value: i32, jump: i32) {
        match value {
            0 => self.store(self.load() - jump),
            1 => self.store(self.load() - 1),
            2 => self.store(self.load() + 1),
            3 => self.store(self.load() + jump),
            _ => {}
        }
    }
}

#[derive(Default)]
struct Controls {
    active_track_id: Parameter<0, { TRACK_COUNT - 1 }>,
    playing: AtomicCell<bool>,
    armed: AtomicCell<bool>,
    tempo: Parameter<1, 999>,
    root: Parameter<-12, 12>,
    scale: Parameter<0, 4>,
    tracks: Vec<Track>,
}
impl Controls {
    fn active_track(&self) -> &Track {
        &self.tracks[self.active_track_id.load() as usize]
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Getter {
    ActiveTrack,
    Playing,
    Armed,
    Tempo,
    Root,
    Scale,
    SampleType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Setter {
    ActiveTrack,
    AuditionDown,
    Play,
    Arm,
    Tempo,
    TempoTaps,
    Root,
    Scale,
    SampleType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "context")]
enum Message {
    Get { method: Getter },
    Set { method: Setter, data: i32 },
}

struct Ui {
    controls: Arc<Controls>,
    sender: Sender<(Setter, i32)>,
}
impl Ui {
    fn process(&self, request: RpcRequest) -> Option<RpcResponse> {
        let (message,) = serde_json::from_value(request.params?).ok()?;
        match message {
            Message::Set { method, data } => {
                let _ = self.sender.send((method, data));
                None
            }
            Message::Get { method } => Some(RpcResponse::new_result(
                request.id,
                Some(self.control(method).into()),
            )),
        }
    }

    fn control(&self, method: Getter) -> i32 {
        match method {
            Getter::ActiveTrack => self.controls.active_track_id.load().into(),
            Getter::Playing => self.controls.playing.load().into(),
            Getter::Armed => self.controls.armed.load().into(),
            Getter::Tempo => self.controls.tempo.load().into(),
            Getter::Root => self.controls.root.load().into(),
            Getter::Scale => self.controls.scale.load().into(),
            Getter::SampleType => self.controls.active_track().sample_type.load() as i32,
        }
    }
}

#[derive(Default)]
struct Cursor {
    age: usize,
    position: f32,
    increment: f32,
    track_id: Option<usize>,
}
impl Cursor {
    fn process(&mut self, controls: &Controls, input: &[f32], output: &mut [f32]) {
        let track = match self.track_id {
            None => return,
            Some(track_id) => &controls.tracks[track_id],
        };
        match track.sample_type.load() {
            SampleType::File => {
                let duration = track.file_sample.duration();
                let position = self.position.floor() as usize;
                let position_rem = self.position.fract();
                if position < duration && position_rem == 0. {
                    self.advance(output, |channel, sample| {
                        *sample += track.file_sample.at(position, channel);
                    });
                } else if position + 1 < duration {
                    self.advance(output, |channel, sample| {
                        let a = track.file_sample.at(position, channel);
                        let b = track.file_sample.at(position + 1, channel);
                        *sample += a + position_rem * (b - a);
                    });
                }
            }
            SampleType::Live | SampleType::LiveRecord | SampleType::LivePlay => {
                for sample in output.iter_mut() {
                    *sample += input.iter().sum::<f32>();
                }
            }
        }
    }

    fn advance<F: Fn(usize, &mut f32)>(&mut self, frame: &mut [f32], write: F) {
        for (channel, sample) in frame.iter_mut().enumerate() {
            write(channel, sample);
        }
        self.position += self.increment;
    }
}

struct Player {
    cursor: Cursor,
    insert_effect: effects::insert,
}
impl Player {
    fn process(&mut self, controls: &Controls, input: &[f32], output: &mut [f32]) {
        let mut pre_buffer = [0.; effects::insert::INPUTS];
        let mut post_buffer = [0.; effects::insert::OUTPUTS];
        self.cursor.process(controls, input, &mut pre_buffer);
        self.insert_effect.compute(
            1,
            &pre_buffer.each_ref().map(std::slice::from_ref),
            &mut post_buffer.each_mut().map(std::slice::from_mut),
        );
        for (output_frame, buffer_frame) in output.iter_mut().zip(post_buffer) {
            *output_frame += buffer_frame;
        }
    }
}

struct Voices {
    players: Vec<Player>,
    send_effects: Vec<Box<dyn Send + FaustDsp<T = f32>>>,
}
impl Voices {
    fn key_down(&mut self, track_id: i32) {
        for player in self.players.iter_mut() {
            player.cursor.age += 1;
        }
        let mut player = self
            .players
            .iter_mut()
            .max_by_key(|x| x.cursor.age)
            .unwrap();
        player.cursor = Cursor {
            age: 0,
            position: 0.0,
            increment: 1.0,
            track_id: Some(track_id as usize),
        };
        player.insert_effect.instance_clear();
    }

    fn process(&mut self, controls: &Controls, input: &[f32], output: &mut [f32]) {
        let mut pre_buffer = [0.; effects::insert::OUTPUTS];
        let mut post_buffer = [0.; 2];
        for player in self.players.iter_mut() {
            player.process(controls, input, &mut pre_buffer);
            for (output_frame, buffer_frame) in output.iter_mut().zip(pre_buffer) {
                *output_frame += buffer_frame;
            }
        }
        for (i, effect) in self.send_effects.iter_mut().enumerate() {
            let i = 2 * (i + 1);
            effect.compute(
                1,
                &pre_buffer.each_ref().map(std::slice::from_ref)[i..i + 2],
                &mut post_buffer.each_mut().map(std::slice::from_mut),
            );
            for (output_frame, buffer_frame) in output.iter_mut().zip(post_buffer) {
                *output_frame += buffer_frame;
            }
        }
    }
}

trait Consume: Sized {
    fn consume(self) {}
}
impl<T: Sized> Consume for T {}

struct Audio {
    controls: Arc<Controls>,
    voices: Voices,
    receiver: Receiver<(Setter, i32)>,
}
impl Audio {
    fn process(&mut self, input: &Frames, output: &mut FramesMut) {
        for (setter, data) in self.receiver.try_iter() {
            match setter {
                Setter::ActiveTrack => self.controls.active_track_id.store(data),
                Setter::AuditionDown => self.voices.key_down(data),
                Setter::Play => self.controls.playing.fetch_xor(true).consume(),
                Setter::Arm => self.controls.armed.fetch_xor(true).consume(),
                Setter::Tempo => self.controls.tempo.nudge(data, 10),
                Setter::TempoTaps => self.controls.tempo.store(data),
                Setter::Root => self.controls.root.nudge(data, 7),
                Setter::Scale => self.controls.scale.store(data),
                Setter::SampleType => self.controls.active_track().sample_type.store(data.into()),
            }
        }
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            self.voices.process(&self.controls, input, output);
        }
    }
}

fn main() -> Result<()> {
    let mut controls = Controls::default();
    controls.tempo.store(120);
    for i in 0..TRACK_COUNT {
        controls.tracks.push(Track {
            file_sample: Sample::read_file(i)?,
            sample_type: SampleType::File.into(),
        });
    }

    let mut players = Vec::new();
    players.resize_with(VOICE_COUNT as usize, || {
        let mut insert_effect = effects::insert::new();
        insert_effect.instance_init(44100);
        Player {
            cursor: Cursor::default(),
            insert_effect,
        }
    });

    let voices = Voices {
        players,
        send_effects: vec![
            Box::new(effects::reverb::new()),
            Box::new(effects::echo::new()),
            Box::new(effects::drive::new()),
        ],
    };
    for player in voices.players.iter() {
        assert_eq!(player.insert_effect.get_num_inputs(), 2);
        assert_eq!(
            voices.send_effects.len() + 1,
            player.insert_effect.get_num_outputs() as usize / 2
        );
    }
    for dsp in voices.send_effects.iter() {
        assert_eq!(dsp.get_num_inputs(), 2);
        assert_eq!(dsp.get_num_outputs(), 2);
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let ui = Ui {
        sender,
        controls: Arc::new(controls),
    };
    let mut audio = Audio {
        receiver,
        controls: Arc::clone(&ui.controls),
        voices,
    };

    let mut device_config = DeviceConfig::new(DeviceType::Duplex);
    device_config.capture_mut().set_channels(1);
    device_config.capture_mut().set_format(Format::F32);
    device_config.playback_mut().set_channels(2);
    device_config.playback_mut().set_format(Format::F32);
    device_config.set_sample_rate(44100);
    let mut device = Device::new(None, &device_config)?;
    device.set_data_callback(move |_, output, input| audio.process(input, output));
    device.start()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Typebeat")
        .build(&event_loop)?;
    let html = std::env::current_dir()?
        .join("src")
        .join("ui")
        .join("index.html");
    let _webview = WebViewBuilder::new(window)?
        .with_url(&format!("file://{}", html.display()))?
        .with_rpc_handler(move |_, request| ui.process(request))
        .build()?;
    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
