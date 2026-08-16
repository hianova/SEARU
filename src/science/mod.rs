pub mod assembly_funnel;
pub mod chaos_runner;
pub mod chaos_swarm;
pub mod chaos_state;
pub mod coevolution_context;
pub mod crucible;
pub mod canvas;
pub mod oracle;
pub mod measurements;
pub mod fft;
pub mod multidomain_fuzz;
pub mod universal_objective;
pub mod resonance_objective;
pub mod aerodynamic_objective;
pub mod topology_isolation_objective;
pub mod metamaterial_objective;


pub trait ScienceObjective<T: Clone + Send + Sync>: Sync {
    fn evaluate_fitness(&self, candidate: &T) -> (u32, u32);
    fn evaluate_fitness_batch(&self, candidates: &[T], out_fitness: &mut [(u32, u32)]) {
        use rayon::prelude::*;
        out_fitness.par_iter_mut().enumerate().for_each(|(i, out)| {
            *out = self.evaluate_fitness(&candidates[i]);
        });
    }
    fn generate_seed(&self, seed: usize, parent: Option<&T>) -> T;
    fn perturb(&self, candidate: &T, scale: f32, seed: usize) -> T;
    fn is_valid(&self, candidate: &T) -> bool;
    fn check_archival(&self, candidate: &T, fitness: (u32, u32)) -> bool;
    fn periodic_validate_and_visualize(&self, _candidate: &T) {}
    fn distill_theory(&self, _old_candidate: &T, _new_candidate: &T, _fitness_jump: u32) {}
    fn crossover(&self, parent_a: &T, parent_b: &T, _seed: usize) -> [T; 4] {
        [
            parent_a.clone(),
            parent_b.clone(),
            parent_a.clone(),
            parent_b.clone(),
        ]
    }
    fn apply_theory_shortcuts(&self, _candidate: &mut T) -> bool {
        false
    }
    fn denoise_step(&self, _candidate: &mut T, _noise_level: f32, _seed: usize) {}
}
