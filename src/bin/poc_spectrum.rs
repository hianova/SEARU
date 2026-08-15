use hound;
use rand::Rng;
use std::fs;

fn main() {
    let sample_rate = 44100;
    let duration_secs = 5.0;
    let total_samples = (sample_rate as f64 * duration_secs) as usize;
    
    // C Major 9 chord frequencies
    // C3, E3, G3, B3, D4
    let chord_freqs = [130.81, 164.81, 196.00, 246.94, 293.66];
    
    fs::create_dir_all("release/poc").unwrap_or_default();
    
    println!("🧪 Generating POC Spectrum Experiments...");
    
    // A: Flat (All harmonics amplitude = 1.0)
    generate_wav("release/poc/A_flat.wav", &chord_freqs, total_samples, sample_rate, |_| 1.0);
    println!("✅ Generated A_flat.wav (White Noise Spectrum)");
    
    // B: Random Jitter (Amplitude = rng(0.0..1.0))
    generate_wav("release/poc/B_jitter.wav", &chord_freqs, total_samples, sample_rate, |_| rand::rng().random_range(0.0..1.0));
    println!("✅ Generated B_jitter.wav (Random Evolutionary Jitter)");
    
    // C: Euler / Mathematical (1/f decay)
    generate_wav("release/poc/C_math.wav", &chord_freqs, total_samples, sample_rate, |h| 1.0 / (h as f32));
    println!("✅ Generated C_math.wav (Perfect 1/f Mathematical Spectrum)");
    
    println!("🎉 All POC files generated in release/poc/ directory.");
}

fn generate_wav<F>(filename: &str, chord_freqs: &[f64], total_samples: usize, sample_rate: u32, mut harmonic_amp_fn: F)
where
    F: FnMut(usize) -> f32,
{
    let mut track = vec![0.0; total_samples];
    let num_harmonics = 16;
    
    // Cache the amplitudes so that they are constant over time (especially important for jitter)
    let amps: Vec<f32> = (1..=num_harmonics).map(|h| harmonic_amp_fn(h)).collect();
    
    for (i, sample) in track.iter_mut().enumerate() {
        let t = i as f64 / sample_rate as f64;
        let mut mixed = 0.0;
        
        for &freq in chord_freqs {
            let mut note_audio = 0.0;
            for h in 1..=num_harmonics {
                let harmonic_freq = freq * h as f64;
                if harmonic_freq > 20000.0 { break; } // Nyquist limit
                
                let amplitude = amps[h - 1] as f64;
                // Add slight detune to emulate acoustic beating
                let detune = 1.0 + (h as f64 * 0.0005);
                note_audio += (harmonic_freq * detune * t * std::f64::consts::TAU).sin() * amplitude;
            }
            mixed += note_audio;
        }
        
        // Envelope (Fade in and out to prevent clicks)
        let fade_len = 22050; // 0.5 second fade
        let mut env = 1.0;
        if i < fade_len {
            env = i as f64 / fade_len as f64;
        } else if i > total_samples - fade_len {
            env = (total_samples - i) as f64 / fade_len as f64;
        }
        
        *sample = (mixed * env * 0.05) as f32; // Scale down to prevent clipping
    }
    
    // Normalize and save
    let mut max_val: f32 = 0.01;
    for &s in track.iter() {
        if s.abs() > max_val { max_val = s.abs(); }
    }
    
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for sample in track {
        let normalized = (sample / max_val) * 0.9;
        writer.write_sample((normalized * 32767.0) as i16).unwrap();
    }
}
