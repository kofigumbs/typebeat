extern crate anyhow;
extern crate cpal;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{OutputCallbackInfo, Sample, SampleRate};
use std::sync::mpsc::{channel, Receiver};
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::WebViewBuilder;

struct Controller {
    receiver: Receiver<String>,
}

impl Controller {
    fn run(&self, data: &mut [f32], _info: &OutputCallbackInfo) {
        for message in self.receiver.try_iter() {
            println!("{}", message);
        }
        for sample in data.iter_mut() {
            *sample = Sample::from(&0.0);
        }
    }
}

fn main() -> Result<()> {
    let device = cpal::default_host()
        .default_output_device()
        .context("no default audio device")?;
    let config = device.default_output_config()?;
    std::assert_eq!(config.channels(), 2);
    std::assert_eq!(config.sample_rate(), SampleRate(44100));
    let (sender, receiver) = channel();
    let controller = Controller { receiver };
    let stream = device.build_output_stream(
        &config.into(),
        move |data, info| controller.run(data, info),
        |_| {},
    )?;
    stream.play()?;

    let mut path = std::env::current_dir()?;
    path.push("ui");
    path.push("index.html");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Typebeat")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_url(&format!("file://{}", path.display()).to_string())?
        .with_rpc_handler(move |_window, request| {
            let _result = sender.send(request.method);
            None
        })
        .build()?;
    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
