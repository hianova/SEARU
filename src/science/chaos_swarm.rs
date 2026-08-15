use crate::science::assembly_funnel::{AssemblyFunnel, EvolutionResult, FunnelConfig};
use crate::science::auto_research::{AutoResearchConfig, AutoResearcher};
use crate::science::ScienceObjective;
use std::thread;

pub struct ChaosSwarm;

impl ChaosSwarm {
    pub fn launch_swarm_tunable<T: Clone + Send + Sync + 'static>(
        objective: impl ScienceObjective<T> + Clone + Send + Sync + 'static,
        mut config: FunnelConfig,
        description: &str,
        num_islands: usize,
        epoch_size: u64,
        num_epochs: usize,
    ) -> (EvolutionResult, Option<T>) {
        println!("🌌 [ChaosSwarm] Igniting Spectral Swarm Topology...");
        println!("   -> Objective: {}", description);
        println!("   -> Islands: {}, Epoch Size: {}, Total Epochs: {}", num_islands, epoch_size, num_epochs);

        // We will store the global best archive across all epochs
        let mut global_archive = crate::science::assembly_funnel::ParetoArchive::<T>::new();
        let mut best_score_overall = u32::MAX;
        let mut final_result = EvolutionResult::Success((u32::MAX, 0));

        for epoch in 0..num_epochs {
            println!("\n🌀 [ChaosSwarm] --- Epoch {}/{} ---", epoch + 1, num_epochs);
            
            // Adjust hard_limit_gen for the epoch so run_evolution_loop terminates after epoch_size
            let epoch_config = FunnelConfig {
                hard_limit_gen: epoch_size,
                ..config.clone()
            };

            // Capture context for threading
            let island_configs: Vec<_> = (0..num_islands)
                .map(|i| {
                    let mut c = epoch_config.clone();
                    // Diversify RNG seed per island based on epoch
                    c.rng_seed = c.rng_seed.wrapping_add((i as u32 * 100) + epoch as u32 * 13);
                    c
                })
                .collect();

            let mut all_histories = vec![];

            // Spawn parallel islands
            thread::scope(|s| {
                let mut island_handles = vec![];

                for (i, island_config) in island_configs.into_iter().enumerate() {
                    let obj_clone = objective.clone();
                    // Inject elites from global archive into tier1
                    let injected_elites = global_archive.elites.clone();
                    
                    let handle = s.spawn(move || {
                        let config_ar = AutoResearchConfig {
                            mode: format!("Island-{}", i),
                        };
                        let mut observer = AutoResearcher::new(config_ar).with_generation_log(false);
                        observer.prefix = format!("[Island-{}]", i);
                        
                        let mut funnel = AssemblyFunnel::new(island_config);
                        
                        // Seed global elites into this island's archive
                        for elite in injected_elites {
                            funnel.archive.try_add(elite.0, elite.1, &elite.2);
                        }

                        let res = funnel.run_evolution_loop(&obj_clone, &mut observer);
                        
                        // Return the funnel's archive and the observer's history for spectral analysis
                        (res, funnel.archive, observer.get_history_deltas())
                    });
                    island_handles.push(handle);
                }

                // Join threads and merge archives
                for handle in island_handles {
                    let (res, archive, history) = handle.join().unwrap();
                    all_histories.push(history);
                    
                    // Track absolute best result
                    if res.best_score() < best_score_overall {
                        best_score_overall = res.best_score();
                        final_result = res;
                    }

                    // Merge into global archive
                    for elite in archive.elites {
                        global_archive.try_add(elite.0, elite.1, &elite.2);
                    }
                }
            });
            
            let current_best = global_archive.elites.iter().min_by_key(|e| e.0).map(|e| e.0).unwrap_or(u32::MAX);
            println!("🌐 [ChaosSwarm] Epoch Complete. Merged Archives. Global Best: {}", current_best);

            // --- Spectral Stagnation Analysis ---
            // Take the history of the first island (or average them) for FFT
            let target_history = &all_histories[0];
            if target_history.len() > 10 {
                // Scale deltas into i16 for our heapless FFT
                let mut samples = vec![0i16; 256];
                for (i, &delta) in target_history.iter().enumerate() {
                    if i >= 256 { break; }
                    // Magnify delta. Delta is usually small or 0.
                    // If delta is 0, we want it to be 0. If delta is large, limit to i16::MAX
                    let scaled = (delta * 100.0) as i32;
                    samples[i] = scaled.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                }

                let mut magnitudes = [0.0f32; 128];
                crate::science::fft::heapless_fft_256(&samples, &mut magnitudes);

                // Analyze frequency bands
                // Low frequency (bins 0-10): slow, creeping convergence (stagnation in a smooth valley)
                // High frequency (bins 50-100): rapid thrashing
                let low_power: f32 = magnitudes[0..10].iter().sum();
                let high_power: f32 = magnitudes[50..100].iter().sum();
                
                println!("📊 [Spectral Analysis] LowFreq Power: {:.2}, HighFreq Power: {:.2}", low_power, high_power);

                if low_power > high_power * 10.0 && current_best > config.hard_limit_score {
                    println!("🚨 [Paradigm Shift] Spectral Stagnation Detected!");
                    println!("🚨 The convergence curve is too smooth and flat. Stuck in a deep local minimum.");
                    println!("🚨 Triggering QUANTUM TUNNELING (Mass Extinction) for the next epoch!");
                    // We don't wipe the global_archive, but we could artificially jump the RNG seed significantly
                    // or modify the config to force diffusion
                    config.use_diffusion = true;
                    config.tier1_population *= 2; // Temporarily double population to explore wider
                } else {
                    // Reset to normal if not stagnated
                    config.use_diffusion = false;
                    config.tier1_population = config.tier1_population.min(10000); // cap
                }
            }
        }

        println!("\n🌌 [ChaosSwarm] Swarm Evolution Complete. Final Best Score: {}", best_score_overall);
        let best_t = global_archive.elites.iter().min_by_key(|e| e.0).map(|e| e.2.clone());
        (final_result, best_t)
    }
}
