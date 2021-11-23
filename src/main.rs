mod fft;
mod widgets;

use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait},
    Stream,
};
use crossbeam_channel::bounded;
use nannou::prelude::*;
use num_complex::Complex32;
use widgets::{complex_path, frequency_graph};

fn open_audio() -> Result<(Stream, crossbeam_channel::Receiver<Vec<f32>>), anyhow::Error> {
    let (s, r) = bounded(1);

    let host = default_host();
    let device = host.default_output_device().unwrap();
    let supported_config = device
        .supported_input_configs()?
        .next()
        .expect("no supported configs")
        .with_max_sample_rate();
    let stream = device.build_input_stream(
        &supported_config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let _ = s.try_send(data.into());
            return ();
        },
        |err| {
            println!("Error: {:?}", err);
        },
    )?;

    Ok((stream, r))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    nannou::app(model).update(update).run();
    Ok(())
}

fn model(app: &App) -> Model {
    let (stream, channel) = open_audio().unwrap();

    let main_window = app
        .new_window()
        .view(view)
        .event(window_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        main_window,
        channel,
        _stream: stream,
        frequency_domain: vec![0.0.into(), 100.0.into()],
        zoom: 1.0,
        paused: false,
    }
}

struct Model {
    main_window: WindowId,
    _stream: Stream,
    channel: crossbeam_channel::Receiver<Vec<f32>>,
    frequency_domain: Vec<Complex32>,
    zoom: f32,
    paused: bool,
}

fn window_event(_app: &App, model: &mut Model, ev: WindowEvent) {
    match ev {
        MousePressed(button) => {
            if button == MouseButton::Middle {
                model.paused = !model.paused;
            }
        }
        MouseWheel(delta, _touch_phase) => {
            if let MouseScrollDelta::LineDelta(rows, lines) = delta {
                model.zoom += lines;
            }
        }
        _ => {}
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for samples in model.channel.try_recv() {
        if model.paused {
            return;
        }
        let result = fft::fft(samples);
        model.frequency_domain = result;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();

    draw.background().color(BLACK);
    complex_path(
        &draw,
        win,
        model.frequency_domain.clone(),
        model.zoom,
        WHITE,
    );
    frequency_graph(
        &draw,
        win,
        model.frequency_domain.clone(),
        44100,
        WHITE,
        0.0005,
    );
    draw.to_frame(app, &frame).unwrap();
}
