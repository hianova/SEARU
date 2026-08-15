//! Digital Signal Processing (DSP) for RobotGo
//! Provides heapless FFT and spectrum analysis for resonance and vibration detection.

/// A structure to hold static/stack complex values during FFT computation.
#[derive(Clone, Copy, Default)]
struct Complex32 {
    re: f32,
    im: f32,
}

impl Complex32 {
    #[inline(always)]
    fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self::new(self.re + other.re, self.im + other.im)
    }

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self::new(self.re - other.re, self.im - other.im)
    }

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    #[inline(always)]
    fn norm(self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

#[allow(clippy::approx_constant)]
static TWIDDLE_COS: [f32; 128] = [
    1.0000000000,
    0.999_698_8,
    0.998_795_45,
    0.997_290_43,
    0.995_184_7,
    0.992_479_56,
    0.989_176_5,
    0.985_277_65,
    0.980_785_25,
    0.975_702_1,
    0.970_031_26,
    0.963_776_05,
    0.956_940_35,
    0.949_528_16,
    0.941_544_06,
    0.932_992_8,
    0.923_879_5,
    0.914_209_8,
    0.903_989_3,
    0.893_224_3,
    0.881_921_3,
    0.870_086_97,
    0.857_728_6,
    0.844_853_6,
    0.831_469_6,
    0.817_584_8,
    0.803_207_5,
    0.788_346_4,
    0.773_010_43,
    0.757_208_8,
    0.740_951_1,
    0.724_247_1,
    0.707_106_77,
    0.689_540_57,
    0.671_559,
    0.653_172_85,
    0.634_393_3,
    0.615_231_6,
    0.595_699_3,
    0.575_808_17,
    0.555_570_24,
    0.534_997_64,
    0.514_102_76,
    0.492_898_2,
    0.471_396_74,
    0.449_611_34,
    0.427_555_08,
    0.405_241_3,
    0.382_683_43,
    0.359_895_05,
    0.336_889_86,
    0.313_681_75,
    0.290_284_66,
    0.266_712_75,
    0.242_980_18,
    0.219_101_24,
    0.195_090_32,
    0.170_961_89,
    0.146_730_47,
    0.122_410_68,
    0.098_017_14,
    0.073_564_57,
    0.049_067_676,
    0.024_541_229,
    0.0000000000,
    -0.024_541_229,
    -0.049_067_676,
    -0.073_564_57,
    -0.098_017_14,
    -0.122_410_68,
    -0.146_730_47,
    -0.170_961_89,
    -0.195_090_32,
    -0.219_101_24,
    -0.242_980_18,
    -0.266_712_75,
    -0.290_284_66,
    -0.313_681_75,
    -0.336_889_86,
    -0.359_895_05,
    -0.382_683_43,
    -0.405_241_3,
    -0.427_555_08,
    -0.449_611_34,
    -0.471_396_74,
    -0.492_898_2,
    -0.514_102_76,
    -0.534_997_64,
    -0.555_570_24,
    -0.575_808_17,
    -0.595_699_3,
    -0.615_231_6,
    -0.634_393_3,
    -0.653_172_85,
    -0.671_559,
    -0.689_540_57,
    -0.707_106_77,
    -0.724_247_1,
    -0.740_951_1,
    -0.757_208_8,
    -0.773_010_43,
    -0.788_346_4,
    -0.803_207_5,
    -0.817_584_8,
    -0.831_469_6,
    -0.844_853_6,
    -0.857_728_6,
    -0.870_086_97,
    -0.881_921_3,
    -0.893_224_3,
    -0.903_989_3,
    -0.914_209_8,
    -0.923_879_5,
    -0.932_992_8,
    -0.941_544_06,
    -0.949_528_16,
    -0.956_940_35,
    -0.963_776_05,
    -0.970_031_26,
    -0.975_702_1,
    -0.980_785_25,
    -0.985_277_65,
    -0.989_176_5,
    -0.992_479_56,
    -0.995_184_7,
    -0.997_290_43,
    -0.998_795_45,
    -0.999_698_8,
];

#[allow(clippy::approx_constant)]
static TWIDDLE_SIN: [f32; 128] = [
    -0.0000000000,
    -0.024_541_229,
    -0.049_067_676,
    -0.073_564_57,
    -0.098_017_14,
    -0.122_410_68,
    -0.146_730_47,
    -0.170_961_89,
    -0.195_090_32,
    -0.219_101_24,
    -0.242_980_18,
    -0.266_712_75,
    -0.290_284_66,
    -0.313_681_75,
    -0.336_889_86,
    -0.359_895_05,
    -0.382_683_43,
    -0.405_241_3,
    -0.427_555_08,
    -0.449_611_34,
    -0.471_396_74,
    -0.492_898_2,
    -0.514_102_76,
    -0.534_997_64,
    -0.555_570_24,
    -0.575_808_17,
    -0.595_699_3,
    -0.615_231_6,
    -0.634_393_3,
    -0.653_172_85,
    -0.671_559,
    -0.689_540_57,
    -0.707_106_77,
    -0.724_247_1,
    -0.740_951_1,
    -0.757_208_8,
    -0.773_010_43,
    -0.788_346_4,
    -0.803_207_5,
    -0.817_584_8,
    -0.831_469_6,
    -0.844_853_6,
    -0.857_728_6,
    -0.870_086_97,
    -0.881_921_3,
    -0.893_224_3,
    -0.903_989_3,
    -0.914_209_8,
    -0.923_879_5,
    -0.932_992_8,
    -0.941_544_06,
    -0.949_528_16,
    -0.956_940_35,
    -0.963_776_05,
    -0.970_031_26,
    -0.975_702_1,
    -0.980_785_25,
    -0.985_277_65,
    -0.989_176_5,
    -0.992_479_56,
    -0.995_184_7,
    -0.997_290_43,
    -0.998_795_45,
    -0.999_698_8,
    -1.0000000000,
    -0.999_698_8,
    -0.998_795_45,
    -0.997_290_43,
    -0.995_184_7,
    -0.992_479_56,
    -0.989_176_5,
    -0.985_277_65,
    -0.980_785_25,
    -0.975_702_1,
    -0.970_031_26,
    -0.963_776_05,
    -0.956_940_35,
    -0.949_528_16,
    -0.941_544_06,
    -0.932_992_8,
    -0.923_879_5,
    -0.914_209_8,
    -0.903_989_3,
    -0.893_224_3,
    -0.881_921_3,
    -0.870_086_97,
    -0.857_728_6,
    -0.844_853_6,
    -0.831_469_6,
    -0.817_584_8,
    -0.803_207_5,
    -0.788_346_4,
    -0.773_010_43,
    -0.757_208_8,
    -0.740_951_1,
    -0.724_247_1,
    -0.707_106_77,
    -0.689_540_57,
    -0.671_559,
    -0.653_172_85,
    -0.634_393_3,
    -0.615_231_6,
    -0.595_699_3,
    -0.575_808_17,
    -0.555_570_24,
    -0.534_997_64,
    -0.514_102_76,
    -0.492_898_2,
    -0.471_396_74,
    -0.449_611_34,
    -0.427_555_08,
    -0.405_241_3,
    -0.382_683_43,
    -0.359_895_05,
    -0.336_889_86,
    -0.313_681_75,
    -0.290_284_66,
    -0.266_712_75,
    -0.242_980_18,
    -0.219_101_24,
    -0.195_090_32,
    -0.170_961_89,
    -0.146_730_47,
    -0.122_410_68,
    -0.098_017_14,
    -0.073_564_57,
    -0.049_067_676,
    -0.024_541_229,
];

/// A zero-allocation, heapless Cooley-Tukey Radix-2 FFT implementation on stack memory.
/// Supports N = 256 samples.
#[inline(always)]
pub fn heapless_fft_256(samples: &[i16], out_magnitudes: &mut [f32; 128]) {
    let mut data = [Complex32::default(); 256];
    let n = 256;

    // 1. Copy samples and apply bit-reversal permutation using single-instruction reverse_bits
    for i in 0..n {
        let rev = (i as u8).reverse_bits() as usize;
        let val = if i < samples.len() {
            samples[i] as f32
        } else {
            0.0
        };
        data[rev] = Complex32::new(val, 0.0);
    }

    // 2. Cooley-Tukey butterfly stage calculations using precomputed twiddle factors
    let mut len = 2;
    while len <= n {
        let step = 256 / len;

        let mut i = 0;
        while i < n {
            for j in 0..(len / 2) {
                let idx = j * step;
                let w = Complex32::new(TWIDDLE_COS[idx], TWIDDLE_SIN[idx]);
                let u = data[i + j];
                let v = data[i + j + len / 2].mul(w);
                data[i + j] = u.add(v);
                data[i + j + len / 2] = u.sub(v);
            }
            i += len;
        }
        len <<= 1;
    }

    // 3. Compute magnitude spectrum for the first half
    for i in 0..128 {
        out_magnitudes[i] = data[i].norm();
    }
}

/// Perform heapless FFT and identify the peak resonant frequency and amplitude.
#[inline(always)]
pub fn local_fixed_fft(samples: &[i16], sample_rate: f32) -> (u32, u16) {
    let mut magnitudes = [0.0f32; 128];
    heapless_fft_256(samples, &mut magnitudes);

    let mut max_mag = -1.0f32;
    let mut peak_index = 0usize;

    // Exclude the DC offset component (index 0) from the peak finder
    for i in 1..128 {
        if magnitudes[i] > max_mag {
            max_mag = magnitudes[i];
            peak_index = i;
        }
    }

    let freq_resolution = sample_rate / 256.0f32;
    let peak_freq = (peak_index as f32) * freq_resolution;

    // Convert peak amplitude to Q15 (normalize relative to max possible amplitude)
    let amp_q15 = ((max_mag / 256.0) * 32768.0).min(65535.0) as u16;

    (peak_freq as u32, amp_q15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_peak_detection() {
        let mut samples = [0i16; 256];
        let sample_rate = 1000.0;

        // Generate a 100Hz sine wave
        for i in 0..256 {
            let t = (i as f32) / sample_rate;
            let val = (2.0 * 3.1415926 * 100.0 * t).sin() * 30000.0;
            samples[i] = val as i16;
        }

        let (peak_hz, amp) = local_fixed_fft(&samples, sample_rate);

        // Resolution is ~3.9Hz (1000/256), so peak should be around 100 +/- 4Hz
        assert!(
            (96..=104).contains(&peak_hz),
            "Peak frequency {} is not near 100Hz",
            peak_hz
        );
        assert!(amp > 10000, "Amplitude {} is too low", amp);
    }
}
