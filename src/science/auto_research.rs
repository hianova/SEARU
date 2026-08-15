use crate::science::assembly_funnel::FunnelObserver;
use std::sync::Mutex;

static THEORY_ARCHIVE: std::sync::LazyLock<Mutex<Vec<String>>> =
    std::sync::LazyLock::new(|| Mutex::new(Vec::new()));

fn get_theory_archive() -> &'static Mutex<Vec<String>> {
    &THEORY_ARCHIVE
}

pub struct AutoResearchConfig {
    pub mode: String, // e.g. "CPU", "GPU", "Dual"
}

pub struct AutoResearcher {
    pub config: AutoResearchConfig,
    pub log_gen: bool,
    pub prefix: String,
    pub history: Vec<u32>,
}

impl AutoResearcher {
    pub fn new(config: AutoResearchConfig) -> Self {
        Self {
            config,
            log_gen: true,
            prefix: "[AutoResearcher]".to_string(),
            history: Vec::new(),
        }
    }

    pub fn with_generation_log(mut self, log_gen: bool) -> Self {
        self.log_gen = log_gen;
        self
    }

    pub fn get_active_rules() -> Vec<String> {
        get_theory_archive().lock().unwrap().clone()
    }

    pub fn get_history_deltas(&self) -> Vec<f32> {
        let mut deltas = Vec::new();
        if self.history.len() < 2 {
            return deltas;
        }
        for i in 1..self.history.len() {
            let delta = self.history[i] as f32 - self.history[i - 1] as f32;
            deltas.push(delta);
        }
        deltas
    }
}

impl FunnelObserver for AutoResearcher {
    fn on_step(&mut self, msg: &str) {
        println!("{} {}", self.prefix, msg);
    }

    fn on_success(&mut self, msg: &str) {
        println!("{} \x1b[1;32mSuccess:\x1b[0m {}", self.prefix, msg);
    }

    fn on_warning(&mut self, msg: &str) {
        println!("{} \x1b[1;33mWarning:\x1b[0m {}", self.prefix, msg);
    }

    fn on_error(&mut self, msg: &str) {
        println!("{} \x1b[1;31mError:\x1b[0m {}", self.prefix, msg);
    }

    fn confirm(&mut self, prompt: &str) -> bool {
        use std::io::Write;
        print!("{} \x1b[1m{} [y/N]\x1b[0m ", self.prefix, prompt);
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            let t = input.trim().to_lowercase();
            t == "y" || t == "yes"
        } else {
            false
        }
    }

    fn on_generation_complete(
        &mut self,
        generation: u64,
        global_iters: u64,
        best_fitness: (u32, u32),
        total_found: usize,
    ) {
        self.history.push(best_fitness.0);
        if self.log_gen && generation % 10 == 0 {
            println!(
                "{} Gen: {} | Iters: {} | Best: {} | Pop: {}",
                self.prefix, generation, global_iters, best_fitness.0, total_found
            );
        }
    }

    fn on_archive_success(&mut self, generation: u64, _global_iters: u64, fitness: (u32, u32)) {
        self.on_success(&format!(
            "Archive Success! Gen: {}, Fitness: {:?}",
            generation, fitness
        ));
    }
}
