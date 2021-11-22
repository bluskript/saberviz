mod fft;

use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait},
    Stream,
};
use crossbeam_channel::{unbounded, Receiver};
use iced::{
    canvas::{Frame, Path, Program},
    executor,
    widget::canvas::Canvas,
    Application, Color, Command, Length, Point, Rectangle, Settings,
};

use crate::fft::fft;

fn open_audio() -> Result<(Stream, crossbeam_channel::Receiver<Vec<f32>>), anyhow::Error> {
    let (s, r) = unbounded();

    let host = default_host();
    let device = host.default_output_device().unwrap();
    let supported_config = device
        .supported_input_configs()?
        .next()
        .expect("no supported configs")
        .with_max_sample_rate();
    let stream = device.build_input_stream(
        &supported_config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| s.send(data.into()).unwrap(),
        |err| {
            println!("Error: {:?}", err);
        },
    )?;

    Ok((stream, r))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SaberViz::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })?;
    Ok(())
}

struct SaberViz {
    chan: Receiver<Vec<f32>>,
    stream: Stream,
}

impl Application for SaberViz {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (stream, chan) = open_audio().unwrap();
        iced::Subscription::from_recipe();
        let val = SaberViz { chan, stream };
        (val, Command::none())
    }

    fn title(&self) -> String {
        "SaberViz".to_string()
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Canvas::new(FourierGraph {
            samples: vec![0.0; 2048],
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

struct FourierGraph {
    samples: Vec<f32>,
}

impl Program<()> for FourierGraph {
    fn draw(
        &self,
        bounds: Rectangle,
        _cursor: iced::canvas::Cursor,
    ) -> Vec<iced::canvas::Geometry> {
        let frequency_domain = fft(self.samples.clone());
        let mut frame = Frame::new(bounds.size());
        frame.fill(
            &Path::new(|p| {
                frequency_domain.iter().for_each(|c| {
                    p.move_to(Point::new(c.re as f32, c.im as f32));
                });
            }),
            Color::BLACK,
        );
        vec![frame.into_geometry()]
    }
}
