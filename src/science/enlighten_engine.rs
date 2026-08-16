use vec101::core::Vec101SuperBlock;

/// Liquid Node (液態節點)：結合連續時間常微分方程 (ODE) 的神經元。
#[derive(Clone, Debug)]
#[repr(C, align(64))]
pub struct LiquidNodeInt8 {
    pub tau_scaled: i32,
    pub state: f32,
}

impl Default for LiquidNodeInt8 {
    fn default() -> Self {
        Self::new()
    }
}

impl LiquidNodeInt8 {
    pub fn new() -> Self {
        Self {
            tau_scaled: 10,
            state: 0.0,
        }
    }

    /// 根據 vec101 的 INT32 輸出 (Dot Product Accumulation) 進行時間積分，並鉗制為 i8
    pub fn step(&mut self, dot_product: i32, dt: f32) -> i8 {
        vec101::compute::liquid_step_i8(dot_product, dt, &mut self.state, self.tau_scaled)
    }
}

/// 整合了 vec101 引擎的液態 KAN 網路層
#[derive(Clone)]
#[repr(C, align(64))]
pub struct LiquidKanLayerFast {
    pub nodes: Vec<LiquidNodeInt8>,
    pub w_stream: Vec<u8>,
    pub s_stream: Vec<i32>,
    pub x_stream: Vec<i8>,
}

impl LiquidKanLayerFast {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        let nodes = vec![LiquidNodeInt8::new(); output_dim];
        let blocks_per_row = input_dim.div_ceil(256);
        let sb_size = core::mem::size_of::<Vec101SuperBlock>();
        let w_stream_len = blocks_per_row * sb_size * output_dim;
        let mut w_stream = vec![0u8; w_stream_len];

        unsafe {
            use rand::{Rng, SeedableRng};
            let mut rng = rand::rngs::StdRng::seed_from_u64(42);
            let ptr = w_stream.as_mut_ptr() as *mut Vec101SuperBlock;
            for i in 0..(blocks_per_row * output_dim) {
                let mut sb: Vec101SuperBlock = core::mem::zeroed();
                let min_scale = 1i16;
                let max_scale = 5i16;
                for s in sb.scales.iter_mut() {
                    *s = rng.random_range(min_scale..max_scale);
                }
                for o in sb.offsets.iter_mut() {
                    *o = 0;
                }
                for block in sb.blocks.iter_mut() {
                    for w in block.w_pos_bits.iter_mut() {
                        *w = rng.random();
                    }
                    for w in block.w_neg_bits.iter_mut() {
                        *w = rng.random();
                    }
                }
                std::ptr::write_unaligned(ptr.add(i), sb);
            }
        }
        let s_stream = vec![1; output_dim];
        let x_stream = vec![0i8; input_dim];

        Self {
            nodes,
            w_stream,
            s_stream,
            x_stream,
        }
    }

    /// 前向傳播：利用 vec101 高速引擎計算乘加，再透過 LiquidNode 進行時間積分
    #[inline(never)]
    pub fn forward(&mut self, inputs: &[i8], dt: f32) -> Vec<i8> {
        self.x_stream.copy_from_slice(inputs);
        let mut out_buffer = vec![0i32; self.nodes.len()];

        let mut engine = vec101::core::Vec101EngineBorrow::new(
            &self.w_stream,
            &self.x_stream,
            &self.s_stream,
            &mut out_buffer,
            1,                // batch_size
            self.nodes.len(), // num_rows
            self.w_stream.len() / (self.nodes.len() * core::mem::size_of::<Vec101SuperBlock>()), // blocks_per_row
        )
        .expect("Failed to initialize vec101 borrow engine");

        engine.compute();

        let mut outputs = Vec::with_capacity(self.nodes.len());
        for (i, node) in self.nodes.iter_mut().enumerate() {
            let dot = out_buffer.get(i).copied().unwrap_or(0);
            outputs.push(node.step(dot, dt));
        }
        outputs
    }

    /// Block-Level NEAT: 擴充網路寬度，增加一個 SuperBlock (8 個輸出節點)
    pub fn grow_block(&mut self) {
        let output_dim = self.nodes.len() + 8;
        self.nodes.resize(output_dim, LiquidNodeInt8::new());
        let input_dim = self.x_stream.len();

        vec101::util::feeder::append_superblocks(&mut self.w_stream, 8, input_dim, 1, 5);
        self.s_stream.resize(output_dim, 1);
    }

    pub fn grow_input_dim(&mut self, added_dim: usize) {
        let old_input_dim = self.x_stream.len();
        let new_input_dim = old_input_dim + added_dim;
        let output_dim = self.nodes.len();
        let old_blocks = old_input_dim.div_ceil(256);
        let new_blocks = new_input_dim.div_ceil(256);
        self.x_stream.resize(new_input_dim, 0);

        if new_blocks > old_blocks {
            let sb_size = core::mem::size_of::<Vec101SuperBlock>();
            let mut new_w_stream = vec![0u8; new_blocks * sb_size * output_dim];
            for row in 0..output_dim {
                let old_start = row * old_blocks * sb_size;
                let old_end = old_start + old_blocks * sb_size;
                let new_start = row * new_blocks * sb_size;
                let new_end = new_start + old_blocks * sb_size;
                new_w_stream[new_start..new_end]
                    .copy_from_slice(&self.w_stream[old_start..old_end]);
            }
            self.w_stream = new_w_stream;
        }
    }

    pub fn save_weights(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        writer.write_all(&(self.w_stream.len() as u64).to_le_bytes())?;
        writer.write_all(&self.w_stream)?;
        Ok(())
    }

    pub fn load_weights(&mut self, reader: &mut impl std::io::Read) -> std::io::Result<()> {
        let mut len_buf = [0u8; 8];
        reader.read_exact(&mut len_buf)?;
        let len = u64::from_le_bytes(len_buf) as usize;
        
        if len != self.w_stream.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Weight dimension mismatch"));
        }
        reader.read_exact(&mut self.w_stream)?;
        Ok(())
    }
}

/// ENLIGHTEN 硬體加速主引擎 (Vec101 Backend)
#[derive(Clone)]
#[repr(C, align(64))]
pub struct EnlightenEngineFast {
    pub layers: Vec<LiquidKanLayerFast>,
    pub residuals: Option<Vec<Vec<i8>>>,
}

impl EnlightenEngineFast {
    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            layers.push(LiquidKanLayerFast::new(layer_sizes[i], layer_sizes[i + 1]));
        }
        Self {
            layers,
            residuals: None,
        }
    }

    /// 演化突變：以給定的機率翻轉 1.58-bit 權重的記憶體位元
    pub fn mutate(&mut self, mutation_rate: f32) {
        use rand::Rng;
        let mut rng = rand::rng();
        let neat_rate = 0.001_f32; // Default NEAT growth rate
        for i in 0..self.layers.len() {
            if rng.random::<f32>() < neat_rate {
                self.layers[i].grow_block();
                if i + 1 < self.layers.len() {
                    self.layers[i + 1].grow_input_dim(8);
                }
            }
        }
        for layer in self.layers.iter_mut() {
            vec101::util::feeder::mutate_weights(&mut layer.w_stream, mutation_rate);
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        let mut file = File::create(path)?;
        for layer in &self.layers {
            layer.save_weights(&mut file)?;
        }
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        let mut file = File::open(path)?;
        for layer in &mut self.layers {
            layer.load_weights(&mut file)?;
        }
        Ok(())
    }

    /// 給定連續的 INT8 輸入序列與時間步長 dt，利用 AVX2/NEON 進行超高速推論
    #[inline(never)]
    pub fn forward_sequence(&mut self, sequence: &[Vec<i8>], dt: f32) -> Vec<Vec<i8>> {
        for layer in self.layers.iter_mut() {
            for node in layer.nodes.iter_mut() {
                node.state = 0.0;
            }
        }
        let mut results = Vec::with_capacity(sequence.len());
        for (step, input) in sequence.iter().enumerate() {
            let mut current = input.clone();
            let mut current_residuals = Vec::new();

            for (layer_idx, layer) in self.layers.iter_mut().enumerate() {
                if let Some(ref res_cache) = self.residuals {
                    let attn_res_threshold = 0.01_f32;
                    if step > 0 && dt < attn_res_threshold && layer_idx < res_cache.len() {
                        for (c, r) in current.iter_mut().zip(res_cache[layer_idx].iter()) {
                            *c = (*c / 2) + (*r / 2);
                        }
                    }
                }
                current_residuals.push(current.clone());
                current = layer.forward(&current, dt);
            }
            self.residuals = Some(current_residuals);
            results.push(current);
        }
        results
    }
}
