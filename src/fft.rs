use std::{
    f64::consts::{E, PI},
    ops::{Add, Sub},
};

use num_complex::Complex64;
use num_traits::{Num, ToPrimitive};
use std::ops::Mul;

trait CExp {
    fn cexp(self) -> Complex64;
}

impl CExp for Complex64 {
    // Euler's formula
    // e^(i*x) = cos(x) + i*sin(x)
    fn cexp(self) -> Complex64 {
        let exp = self.re.exp();
        Complex64::new(exp * self.re.cos(), exp * self.re.sin())
    }
}

fn inner_fft(mut amplitudes: Vec<Complex64>) -> Vec<Complex64> {
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
        let progress = i / len;
        // fourier transform
        let pos = Complex64::new(0.0, a * progress as f64).cexp().mul(odd[i]);
        let left_half = even[i].add(pos);
        let right_half = even[i].sub(pos);
        amplitudes[i] = left_half;
        odd[i] = left_half;
        even[i] = right_half;
        amplitudes[half + i] = right_half;
    }

    amplitudes
}

pub fn fft<T: ToPrimitive + Num>(amplitudes: Vec<T>) -> Vec<Complex64> {
    return inner_fft(
        amplitudes
            .iter()
            .map(|x| Complex64::new(x.to_f64().unwrap(), 0.0))
            .collect(),
    );
}

pub fn inverse_fft(mut amplitudes: Vec<Complex64>) -> Vec<Complex64> {
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
        current_amp.re = current_amp.re * inverse as f64;
        current_amp.im = current_amp.im * inverse as f64;
    }
    return amplitudes;
}
