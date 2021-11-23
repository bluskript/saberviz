use nannou::{color::IntoLinSrgba, draw::properties::ColorScalar, geom, Draw};
use num_complex::Complex32;

pub fn complex_path<C>(
    draw: &Draw,
    rect: geom::Rect<f32>,
    numbers: Vec<Complex32>,
    zoom: f32,
    color: C,
) where
    C: IntoLinSrgba<ColorScalar>,
{
    let points = numbers.iter().map(|&samp| {
        (
            samp.re * rect.w() * zoom / 10000.0,
            samp.im * rect.h() * zoom / 10000.0,
        )
    });
    draw.polyline().weight(0.5).color(color).points(points);
}

pub fn frequency_graph<C>(
    draw: &Draw,
    rect: geom::Rect<f32>,
    frequency_domain: Vec<Complex32>,
    sample_rate: usize,
    color: C,
    height: f32,
) where
    C: IntoLinSrgba<ColorScalar>,
{
    let points = frequency_domain[..frequency_domain.len() / 2]
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let frequency =
                (i as f32 * (sample_rate as f32 / 2.0)) / (frequency_domain.len() as f32 / 2.0);
            let magnitude = (v.re * v.re + v.im * v.im).sqrt();
            (
                (frequency / 100.0) * rect.w() as f32,
                magnitude * rect.h() * height,
            )
        });
    draw.polyline()
        .weight(0.3)
        .color(color)
        .points(points)
        .xy(rect.bottom_left());
}
