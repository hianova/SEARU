#![allow(unused_imports)]
#![allow(unused_assignments, clippy::assign_op_pattern)]
use covopt_macro::covopt_param;
use std::io::Write;
use std::process::Command;
use std::time::Instant;

fn main() {
    println!("🤖 Blender Physics: ScriptGo (SGL) Native Physics Benchmark 🤖");
    println!("Simulating 100,000 objects over 600 frames...");
    println!("--------------------------------------------------");

    // ==========================================
    // ScriptGo (SGL AOT) Benchmark
    // ==========================================
    println!("\nRunning ScriptGo (SGL AOT) Physics benchmark...");

    let num_objects = covopt_macro::covopt_param!("M_37_22", 100000);
    let steps = covopt_macro::covopt_param!("M_38_16", 600);

    let mut positions_y = vec![0.0f64; num_objects];
    for (i, pos_y) in positions_y.iter_mut().enumerate().take(num_objects) {
        *pos_y = (i % covopt_macro::covopt_param!("M_42_22", 100)) as f64
            + covopt_macro::covopt_param!("M_42_36", 10.0);
    }
    let mut velocities_y = vec![0.0f64; num_objects];

    let dt = 1.0f64 / covopt_macro::covopt_param!("M_46_22", 60.0);
    let gravity = covopt_macro::covopt_param!("M_47_18", 9.8);
    let bounce = -covopt_macro::covopt_param!("M_48_18", 0.8);

    let start_sgl = Instant::now();

    for _ in 0..steps {
        script_go::sgl_compile!(
            r#"
            let i: usize = 0;
            while i < 100000 {
                let vy: Float = velocities_y[i];
                let py: Float = positions_y[i];
                
                vy = vy - (gravity * dt);
                py = py + (vy * dt);
                
                if py < 0.0 {
                    py = 0.0;
                    vy = vy * bounce;
                }
                
                velocities_y[i] = vy;
                positions_y[i] = py;
                i = i + 1;
            }
        "#
        );
    }

    let sgl_duration = start_sgl.elapsed();

    println!("✅ ScriptGo (AOT) completed in: {:?}", sgl_duration);
    println!("SGL Object 1 Final Height: {:.2}", positions_y[1]);
    println!("--------------------------------------------------");
}
