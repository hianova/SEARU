use covopt_macro::covopt_evolve;

#[covopt_evolve(bounds = "temperature < 1000.0, stress < 500.0", fuzzer = "neo_thermodynamics")]
pub struct NeoThermodynamics {
    pub temperature: f64,
    pub stress: f64,
}

pub fn get_injected_laws() -> &'static str {
    // 讀取由巨集在背後偷偷生成的 JSON 隱藏常數
    // 根據 covopt-macro 的實作，它會生成 `__COVOPT_EVOLVE_NeoThermodynamics_METADATA`
    __COVOPT_EVOLVE_NeoThermodynamics_METADATA
}
