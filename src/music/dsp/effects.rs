pub struct Effects;

impl Effects {
    pub fn simple_delay(input: &[f32], delay_samples: usize, feedback: f32, mix: f32) -> Vec<f32> {
        let mut output = vec![0.0; input.len()];
        let mut delay_buffer = vec![0.0; delay_samples];
        let mut write_pos = 0;

        for (i, &sample) in input.iter().enumerate() {
            let delayed = delay_buffer[write_pos];
            
            // Output = dry + wet
            output[i] = sample + delayed * mix;
            
            // Feed back into delay buffer
            delay_buffer[write_pos] = sample + delayed * feedback;
            
            write_pos = (write_pos + 1) % delay_samples;
        }
        
        // Normalize slightly to prevent clipping if feedback builds up
        for s in output.iter_mut() {
            *s = s.clamp(-1.0, 1.0);
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
