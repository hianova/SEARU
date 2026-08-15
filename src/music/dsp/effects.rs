pub struct Effects;

struct CombFilter {
    buffer: Vec<f32>,
    pos: usize,
    feedback: f32,
    damping: f32,
    store: f32,
}

impl CombFilter {
    fn new(delay_samples: usize, feedback: f32, damping: f32) -> Self {
        Self {
            buffer: vec![0.0; delay_samples],
            pos: 0,
            feedback,
            damping,
            store: 0.0,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.pos];
        // Low-pass filter for damping high frequencies over time
        self.store = (output * (1.0 - self.damping)) + (self.store * self.damping);
        self.buffer[self.pos] = input + self.store * self.feedback;
        self.pos = (self.pos + 1) % self.buffer.len();
        output
    }
}

struct AllPassFilter {
    buffer: Vec<f32>,
    pos: usize,
    feedback: f32,
}

impl AllPassFilter {
    fn new(delay_samples: usize, feedback: f32) -> Self {
        Self {
            buffer: vec![0.0; delay_samples],
            pos: 0,
            feedback,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let delayed = self.buffer[self.pos];
        let output = -input + delayed;
        self.buffer[self.pos] = input + delayed * self.feedback;
        self.pos = (self.pos + 1) % self.buffer.len();
        output
    }
}

impl Effects {
    /// Applies a Chaotic Spatial Acoustic Model (Schroeder/Freeverb style)
    /// `mix`: 0.0 (Dry) to 1.0 (Wet)
    /// `room_size`: 0.0 (Small room) to 1.0 (Massive cathedral)
    /// `damping`: 0.0 (Bright/reflective) to 1.0 (Dark/absorptive)
    pub fn process_reverb(input: &[f32], mix: f32, room_size: f32, damping: f32) -> Vec<f32> {
        // Prime numbers to ensure chaotic diffusion without resonant buildup
        let fcf_delays = [1557, 1613, 1493, 1231, 1063, 1151, 877, 971];
        let apf_delays = [223, 557, 347, 113];

        let feedback = (room_size.clamp(0.0, 1.0) * 0.28) + 0.7; // 0.7 to 0.98

        let mut combs: Vec<CombFilter> = fcf_delays
            .iter()
            .map(|&d| CombFilter::new(d, feedback, damping))
            .collect();
        let mut allpasses: Vec<AllPassFilter> = apf_delays
            .iter()
            .map(|&d| AllPassFilter::new(d, 0.5))
            .collect();

        let mut output = vec![0.0; input.len()];

        for (i, &sample) in input.iter().enumerate() {
            let mut reverb_signal = 0.0;

            // Parallel Comb Filters (Room Resonances)
            for comb in combs.iter_mut() {
                reverb_signal += comb.process(sample);
            }

            // Series All-Pass Filters (Chaotic Diffusion)
            for apf in allpasses.iter_mut() {
                reverb_signal = apf.process(reverb_signal);
            }

            output[i] = (sample * (1.0 - mix) + reverb_signal * mix * 0.15).clamp(-1.0, 1.0);
        }

        output
    }

    /// Soft Clipping Overdrive for Sub-Bass Saturation
    pub fn soft_clip(input: &[f32], drive: f32) -> Vec<f32> {
        let mut output = vec![0.0; input.len()];
        for (i, &sample) in input.iter().enumerate() {
            let driven = sample * drive;
            // Hyperbolic tangent for smooth saturation
            output[i] = driven.tanh();
        }
        output
    }
}
