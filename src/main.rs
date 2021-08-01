#![feature(array_methods)]

use std::borrow::Borrow;
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
struct Controls {
    active_track_id: AtomicCell<i32>,
    playing: AtomicCell<bool>,
    armed: AtomicCell<bool>,
    tempo: AtomicCell<i32>,
    root: AtomicCell<i32>,
    scale: AtomicCell<i32>,
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

enum Keep {
    Below(i32),
    Between(i32, i32),
}
trait AtomicI32: Borrow<AtomicCell<i32>> {
    fn set(&self, value: i32, keep: Keep) {
        self.borrow().store(match keep {
            Keep::Below(max) => value.clamp(0, max - 1),
            Keep::Between(min, max) => value.clamp(min, max),
        });
    }

    fn nudge(&self, value: i32, jump: i32, keep: Keep) {
        match value {
            0 => self.set(self.borrow().load() - jump, keep),
            1 => self.set(self.borrow().load() - 1, keep),
            2 => self.set(self.borrow().load() + 1, keep),
            3 => self.set(self.borrow().load() + jump, keep),
            _ => {}
        }
    }
}
impl AtomicI32 for AtomicCell<i32> {}

trait AtomicBool: Borrow<AtomicCell<bool>> {
    fn toggle(&self) {
        self.borrow().fetch_xor(true);
    }
}
impl AtomicBool for AtomicCell<bool> {}

#[derive(Default)]
struct Cursor {
    active: bool,
    track_id: usize,
    position: f32,
    increment: f32,
}
impl Cursor {
    fn process(&mut self, track: &Track, input: &[f32], output: &mut [f32]) {
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
    fn process(&mut self, track: &Track, input: &[f32], output: &mut [f32]) {
        let mut pre_buffer = [0.; effects::insert::INPUTS];
        let mut post_buffer = [0.; effects::insert::OUTPUTS];
        self.cursor.process(track, input, &mut pre_buffer);
        self.insert_effect.compute(
            1,
            &pre_buffer.each_ref().map(std::slice::from_ref),
            &mut post_buffer.each_mut().map(std::slice::from_mut),
        );
        output.copy_from_slice(&post_buffer[..2]);
    }
}

struct Voices {
    players: Vec<Player>,
}
impl Voices {
    fn key_down(&mut self, track_id: i32) {
        let player = &mut self.players[0];
        player.cursor = Cursor {
            active: true,
            track_id: track_id as usize,
            position: 0.0,
            increment: 1.0,
        };
        player.insert_effect.instance_clear();
    }
}

struct Audio {
    controls: Arc<Controls>,
    voices: Voices,
    receiver: Receiver<(Setter, i32)>,
}
impl Audio {
    fn process(&mut self, input: &Frames, output: &mut FramesMut) {
        let controls = &mut self.controls;
        let voices = &mut self.voices;
        for (setter, data) in self.receiver.try_iter() {
            match setter {
                Setter::ActiveTrack => controls.active_track_id.set(data, Keep::Below(TRACK_COUNT)),
                Setter::AuditionDown => voices.key_down(data),
                Setter::Play => controls.playing.toggle(),
                Setter::Arm => controls.armed.toggle(),
                Setter::Tempo => controls.tempo.nudge(data, 10, Keep::Between(1, 999)),
                Setter::TempoTaps => controls.tempo.set(data, Keep::Between(1, 999)),
                Setter::Root => controls.root.nudge(data, 7, Keep::Between(-12, 12)),
                Setter::Scale => controls.scale.set(data, Keep::Below(5)),
                Setter::SampleType => controls.active_track().sample_type.store(data.into()),
            }
        }
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            for player in voices.players.iter_mut().filter(|x| x.cursor.active) {
                player.process(&controls.tracks[player.cursor.track_id], input, output);
            }
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

    let (sender, receiver) = std::sync::mpsc::channel();
    let ui = Ui {
        sender,
        controls: Arc::new(controls),
    };
    let mut audio = Audio {
        receiver,
        controls: Arc::clone(&ui.controls),
        voices: Voices { players },
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
