use std::{
    f64::consts::{E, PI},
    ops::{Add, Sub},
};

use num_complex::Complex32;
use num_traits::{Num, ToPrimitive};
use std::ops::Mul;

fn inner_fft(mut amplitudes: Vec<Complex32>) -> Vec<Complex32> {
    let len = amplitudes.len();
    if len <= 1 {
        return amplitudes;
    }
    let half = len / 2;

    // divide even and odd
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for i in 0..half {
        even.push(amplitudes[i * 2]);
        odd.push(amplitudes[i * 2 + 1]);
    }

    // recursively apply FFT
    even = inner_fft(even);
    odd = inner_fft(odd);

    // combine result
    let a = -2.0 * PI;
    for i in 0..half {
        // the progress along the signal
        let progress = i as f32 / len as f32;
        // fourier transform
        let pos = Complex32::new(0.0, a as f32 * progress as f32)
            .exp()
            .mul(odd[i]);
        odd[i] = even[i].add(pos);
        amplitudes[i] = odd[i];
        even[i] = even[i].sub(pos);
        amplitudes[i + half] = even[i];
    }

    amplitudes
}

pub fn fft<T: ToPrimitive + Num>(amplitudes: Vec<T>) -> Vec<Complex32> {
    return inner_fft(
        amplitudes
            .iter()
            .map(|x| Complex32::new(x.to_f32().unwrap(), 0.0))
            .collect(),
    );;
}

pub fn inverse_fft(mut amplitudes: Vec<Complex32>) -> Vec<Complex32> {
    let len = amplitudes.len();
    let inverse = 1 / len;

    // conjugate imaginaries
    for i in 0..len {
        amplitudes[i].im = -amplitudes[i].im;
    }

    // apply fourier transform
    amplitudes = inner_fft(amplitudes);

    for i in 0..len {
        let mut current_amp = amplitudes[i];
        // conjugate again
        current_amp.im = -current_amp.im;
        // scale
        current_amp.re = current_amp.re * inverse as f32;
        current_amp.im = current_amp.im * inverse as f32;
    }
    return amplitudes;
}
