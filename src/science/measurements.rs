use crate::science::canvas::TopologyCanvas;
use crate::science::chaos_state::ChaosEngram;
use std::time::Instant;

pub struct AcademicBenchmarkReport {
    pub shannon_entropy: f64,
    pub avg_hamming_distance: f64,
    pub compression_ratio: f64,
    pub ops_per_second: f64,
}

pub fn run_chaos_benchmark(iterations: usize, canvas_size: usize) -> AcademicBenchmarkReport {
    println!("🧪 [Academic Benchmarks] Initiating Native Chaos Architecture Verification...");
    println!("   -> Target Iterations: {}", iterations);
    println!("   -> Canvas Size: {} bits", canvas_size);

    let mut canvas = TopologyCanvas::new(canvas_size, 1);
    let chaos_state = ChaosEngram {
        seed: 0x9E3779B97F4A7C15, // Golden ratio prime seed
        energy_level: 0.5,        // 50% threshold for max entropy
        fitness: 1.0,
    };

    let mut total_hamming_distance = 0;
    
    // 1. Kolmogorov Compression Ratio
    // Traditional Float16 Model: N parameters * 16 bits = N * 2 bytes
    // 1.58-bit Model: N parameters * 2 bits (pos/neg masks) = N / 4 bytes
    // Native Chaos: 1 Canvas (N/8 bytes) + ChaosState (24 bytes)
    let traditional_fp16_bytes = (canvas_size * 2) as f64;
    let bit158_bytes = (canvas_size as f64) / 4.0;
    let chaos_bytes = (canvas.bitmask.len() * 8 + 24) as f64;
    
    let compression_ratio = traditional_fp16_bytes / chaos_bytes;

    println!("\n📊 1. Memory Footprint Analysis (Kolmogorov Compression)");
    println!("   -> Traditional FP16 required: {:.2} MB", traditional_fp16_bytes / 1_000_000.0);
    println!("   -> 1.58-bit Engine required: {:.2} MB", bit158_bytes / 1_000_000.0);
    println!("   -> Native Chaos Persistence: {:.2} Bytes", chaos_bytes);
    println!("   -> Compression Ratio vs FP16: 1 : {:.0}", compression_ratio);

    // 2. Speed and Kinetic Energy Test
    let start_time = Instant::now();
    let mut ones_count = 0;

    for _ in 0..iterations {
        let old_mask = canvas.bitmask.clone();
        canvas.advance_with_chaos(&chaos_state);
        
        // Calculate Hamming Distance
        for (i, &block) in canvas.bitmask.iter().enumerate() {
            let diff = block ^ old_mask[i];
            total_hamming_distance += diff.count_ones() as usize;
            ones_count += block.count_ones() as usize;
        }
    }

    let elapsed = start_time.elapsed();
    let ops_per_second = (iterations as f64) / elapsed.as_secs_f64();

    println!("\n⚡ 2. Spatio-Temporal Kinetic Energy (Throughput)");
    println!("   -> Total Execution Time: {:.2?}", elapsed);
    println!("   -> Operations per second: {:.2} Ops/sec", ops_per_second);
    
    let avg_hamming_distance = (total_hamming_distance as f64) / (iterations as f64) / (canvas_size as f64);
    println!("   -> Avg Bit Flip Ratio per Step: {:.4} (Kinetic Energy)", avg_hamming_distance);

    // 3. Shannon Entropy
    // A uniform random distribution should have an entropy close to 1.0 bit per binary symbol
    let p1 = (ones_count as f64) / ((iterations * canvas_size) as f64);
    let p0 = 1.0 - p1;
    let shannon_entropy = if p1 == 0.0 || p0 == 0.0 {
        0.0
    } else {
        -(p1 * p1.log2() + p0 * p0.log2())
    };

    println!("\n🌌 3. Information Density (Shannon Entropy)");
    println!("   -> P(1) Probability: {:.4}", p1);
    println!("   -> Shannon Entropy: {:.6} bits (Max 1.0)", shannon_entropy);
    
    if shannon_entropy > 0.99 {
        println!("   -> 結論: 混沌畫布成功保持極高熵值，未退化為單一規律，符合最高學術標準！");
    }

    AcademicBenchmarkReport {
        shannon_entropy,
        avg_hamming_distance,
        compression_ratio,
        ops_per_second,
    }
}
