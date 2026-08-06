use std::f32::consts::PI;

#[derive(Debug)]
pub enum NoiseColor {
    White,    // 純白噪音 (均勻分布)
    Gaussian, // 高斯白噪音 (常態分布)
    Binary,   // 二元噪音 (-1 或 1)
    Brown,    // 棕噪音 (能量集中低頻，像瀑布)
    Pink,     // 粉紅噪音 (能量隨頻率遞減，自然界常見)
    Blue,     // 藍噪音 (能量集中高頻，像嘶嘶聲)
    Violet,   // 紫噪音 (更高頻的藍噪音)
    Velvet,   // 天鵝絨噪音 (稀疏的脈衝)
    Crackle,  // 爆音/黑膠噪音 (偶發的高強度隨機突波)
    Wind,     // 風聲 (以低頻 LFO 調變的棕噪音)
}

pub struct NoiseGenerator;

impl NoiseGenerator {
    pub fn generate(color: NoiseColor, seconds: f32, sample_rate: u32) -> Vec<f32> {
        let total_samples = (seconds * sample_rate as f32) as usize;
        let mut buffer = Vec::with_capacity(total_samples);

        let mut last_brown = 0.0;
        let mut last_white = 0.0;
        let mut last_blue = 0.0;

        let mut pink_state = [0.0; 7];
        let mut pink_keys = 0u32;
        let mut phase: f32 = 0.0;

        for _ in 0..total_samples {
            // 生成基礎均勻分佈白噪音 -1.0 到 1.0
            let white = (rand::random::<f32>() * 2.0) - 1.0;

            let sample = match color {
                NoiseColor::White => white,

                NoiseColor::Gaussian => {
                    let u1 = rand::random::<f32>().max(0.0001);
                    let u2 = rand::random::<f32>();
                    (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos() * 0.3
                }

                NoiseColor::Binary => {
                    if rand::random::<bool>() {
                        1.0
                    } else {
                        -1.0
                    }
                }

                NoiseColor::Brown => {
                    last_brown = (last_brown + 0.05 * white) / 1.05;
                    last_brown * 3.5
                }

                NoiseColor::Pink => {
                    let mut sum = 0.0;
                    pink_keys += 1;
                    let mut diff = pink_keys;
                    for j in 0..7 {
                        if diff & 1 != 0 {
                            pink_state[j] = (rand::random::<f32>() * 2.0) - 1.0;
                            break;
                        }
                        diff >>= 1;
                    }
                    for s in pink_state.iter() {
                        sum += *s;
                    }
                    sum * 0.15
                }

                NoiseColor::Blue => {
                    let out = white - last_white;
                    last_white = white;
                    out * 0.5
                }

                NoiseColor::Violet => {
                    let current_blue = white - last_white;
                    last_white = white;
                    let out = current_blue - last_blue;
                    last_blue = current_blue;
                    out * 0.3
                }

                NoiseColor::Velvet => {
                    if rand::random::<f32>() < 0.01 {
                        if rand::random::<bool>() { 1.0 } else { -1.0 }
                    } else {
                        0.0
                    }
                }

                NoiseColor::Crackle => {
                    if rand::random::<f32>() < 0.005 {
                        let amp = 0.5 + (rand::random::<f32>() * 0.5);
                        if rand::random::<bool>() { amp } else { -amp }
                    } else {
                        0.0
                    }
                }

                NoiseColor::Wind => {
                    last_brown = (last_brown + 0.05 * white) / 1.05;
                    phase += 0.5 * PI * 2.0 / sample_rate as f32; // 0.5 Hz LFO
                    if phase > 2.0 * PI {
                        phase -= 2.0 * PI;
                    }
                    let lfo = (phase.sin() * 0.5 + 0.5) * 0.8 + 0.2;
                    last_brown * 3.5 * lfo
                }
            };

            buffer.push(sample.clamp(-1.0, 1.0));
        }
        buffer
    }
}
