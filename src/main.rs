mod album;
mod api;
mod architecture;
mod error;
mod materials;
mod mechanics;
mod megacity;
mod music;
mod profile;
mod science;
mod intent;
pub mod synesthesia;

use error::AppError;
use axum::response::sse::{Event, Sse};
use axum::{
    Json, Router,
    http::header,
    response::IntoResponse,
    routing::{get, post},
};
use futures::stream::Stream;
use serde::Serialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use api::SearuApi;
use architecture::FloorPlanner;
use mechanics::exporter::TrussExporter;
use megacity::MegaCityPipeline;
use music::dsp::exporter::AudioExporter;

#[tokio::main]
async fn main() {
    println!("🚀 Starting SEARU Generative Design Suite on http://localhost:3000");
    println!("📜 [System Engine] Runtime Parameters Initialized:");

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--benchmark") {
        crate::science::measurements::run_chaos_benchmark(1_000_000, 262144); // 256K bits = 32KB canvas
        return;
    }

    tokio::spawn(autonomous_pulse());

    // Initialize telemetry channel (capacity 1000)
    let (tx, _) = tokio::sync::broadcast::channel(1000);
    crate::science::crucible::TELEMETRY_TX.set(tx).unwrap();

    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .nest_service("/release", ServeDir::new("release"))
        .route("/api/music/generate", post(api_music_generate))
        .route("/api/synesthesia", post(api_synesthesia_generate))
        .route("/api/script/run", post(api_script_run))
        .route("/api/music/fm", get(api_music_fm))
        .route("/api/mechanics/truss", get(api_mechanics_truss))
        .route("/api/materials/match", get(api_materials_match))
        .route("/api/materials/match", post(api_materials_match_post))
        .route("/api/arch/floorplan", get(api_arch_floorplan))
        .route("/api/arch/floorplan", post(api_arch_floorplan_post))
        .route("/api/megacity/pipeline", post(api_megacity_pipeline))
        .route("/api/science/multidomain_fuzz", get(api_science_multidomain_fuzz))
        .route("/api/science/logic_proof", get(api_science_logic_proof))
        .route("/api/album/release", get(api_album_release).post(api_album_release))
        .route("/api/album/tracks", get(api_album_tracks))
        .route("/api/album/track/:filename", get(api_serve_album_track))
        .route("/api/telemetry", get(api_telemetry))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("⚠️  [Shutdown] Received Ctrl-C, forcing graceful shutdown...");
    let oracle = crate::science::oracle::get_oracle();
    if let Ok(o) = oracle.lock() {
        if let Ok(json) = serde_json::to_string_pretty(&o.state) {
            if let Err(e) = std::fs::write("searu_chaos.engram", json) {
                println!("❌ Failed to flush searu_chaos.engram: {}", e);
            } else {
                println!("💾 Chaos Canvas memories successfully flushed to searu_chaos.engram");
            }
        }
    }
}

async fn api_telemetry() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let tx = crate::science::crucible::TELEMETRY_TX.get().unwrap();
    let rx = tx.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|msg| {
        match msg {
            Ok(event) => Some(Ok(Event::default().json_data(event).unwrap())),
            Err(_) => None, // Ignore lag errors
        }
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}

async fn fallback() -> impl IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        "SEARU Edge Runtime: Route not found.",
    )
}

use crate::science::crucible::{TheCrucible, Gene};

#[derive(Serialize)]
pub struct MultidomainResponse {
    pub final_score: f64,
    pub genes: [f64; 9],
}

async fn api_science_multidomain_fuzz() -> impl IntoResponse {
    let mut genes = vec![
        Gene { name: "enstrophy".into(), bounds: (0.0, 10.0), current_value: 0.5 },
        Gene { name: "pressure_gradient".into(), bounds: (0.0, 10.0), current_value: 0.5 },
        Gene { name: "viscosity".into(), bounds: (0.1, 50.0), current_value: 1.0 },
        Gene { name: "local_strain".into(), bounds: (0.0, 50.0), current_value: 0.5 },
        Gene { name: "stiffness".into(), bounds: (10.0, 500.0), current_value: 100.0 },
        Gene { name: "damping".into(), bounds: (1.0, 50.0), current_value: 10.0 },
        Gene { name: "freq".into(), bounds: (0.1, 20.0), current_value: 1.0 },
        Gene { name: "radius".into(), bounds: (1.0, 50.0), current_value: 5.0 },
        Gene { name: "power".into(), bounds: (1.0, 100.0), current_value: 10.0 },
    ];
    
    let (final_score, best_genes) = TheCrucible::anneal(
        genes,
        |g| crate::science::multidomain_fuzz::evaluate_multidomain(g, 25.0, 100.0, 0.5),
        500
    );
    
    let mut best_array = [0.0; 9];
    for i in 0..9 {
        best_array[i] = best_genes[i].current_value;
    }
    
    Json(MultidomainResponse {
        final_score,
        genes: best_array,
    })
}

#[derive(Serialize)]
pub struct LogicProofResponse {
    pub sat_solved: bool,
    pub sat_result: Vec<bool>,
    pub syllogism_solved: bool,
    pub syllogism_result: bool,
    pub graph_color_solved: bool,
    pub graph_color_result: Vec<u8>,
}

async fn api_science_logic_proof() -> impl IntoResponse {
    let sat_out = crate::science::logic_sat::prove_sat();
    let syllogism_out = crate::science::logic_syllogism::prove_syllogism();
    let graph_out = crate::science::logic_graph_color::prove_graph_coloring();
    
    Json(LogicProofResponse {
        sat_solved: sat_out.is_ok(),
        sat_result: sat_out.unwrap_or_default(),
        syllogism_solved: syllogism_out.is_ok(),
        syllogism_result: syllogism_out.unwrap_or_default(),
        graph_color_solved: graph_out.is_ok(),
        graph_color_result: graph_out.unwrap_or_default(),
    })
}

async fn api_music_fm(
    axum::extract::Query(query): axum::extract::Query<api::FmRequest>,
) -> Result<impl IntoResponse, AppError> {
    let diss = query.dissonance.unwrap_or(0.5);
    let patch = crate::music::fm_synth::FmOptimizer::optimize_patch(diss);
    let buffer = crate::music::fm_synth::FmSynthesizer::render(&patch, 440.0, 2.0, 44100);
    let bytes = AudioExporter::encode_to_wav_bytes(&buffer, 44100)?;
    Ok(([(header::CONTENT_TYPE, "audio/wav")], bytes))
}

async fn api_music_generate(
    axum::Json(profile): axum::Json<crate::profile::ArtistProfile>,
) -> Result<impl IntoResponse, AppError> {
    profile.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let sample_rate = 44100;
    let audio_buffer = SearuApi::generate_music_with_profile(&profile);
    let bytes = AudioExporter::encode_to_wav_bytes(&audio_buffer, sample_rate)?;
    Ok(([(header::CONTENT_TYPE, "audio/wav")], bytes))
}


async fn api_mechanics_truss() -> impl IntoResponse {
    let truss = SearuApi::optimize_mechanics_truss();
    let svg_str = TrussExporter::to_svg_string(&truss);
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
}

#[derive(Serialize)]
struct PbrResponse {
    albedo: [f64; 3],
    roughness: f64,
    metallic: f64,
}

async fn api_materials_match() -> impl IntoResponse {
    let target_front = [0.1, 0.2, 0.8];
    let target_edge = [0.2, 0.8, 0.9];
    let mat = SearuApi::match_pbr_material(target_front, target_edge);
    Json(PbrResponse {
        albedo: mat.albedo,
        roughness: mat.roughness,
        metallic: mat.metallic,
    })
}

async fn api_materials_match_post(
    axum::Json(req): axum::Json<api::MaterialRequest>,
) -> impl IntoResponse {
    let target_front = [req.target_r, req.target_g, req.target_b];
    let target_edge = [
        (req.target_r + 0.2).min(1.0),
        (req.target_g + 0.2).min(1.0),
        (req.target_b + 0.2).min(1.0),
    ];
    let mat = SearuApi::match_pbr_material(target_front, target_edge);
    Json(PbrResponse {
        albedo: mat.albedo,
        roughness: mat.roughness,
        metallic: mat.metallic,
    })
}

async fn api_arch_floorplan() -> impl IntoResponse {
    let rooms = SearuApi::optimize_floorplan(crate::profile::ArchProfile::default());
    let svg_str = FloorPlanner::to_svg_string(&rooms);
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
}

async fn api_arch_floorplan_post(
    axum::Json(profile): axum::Json<crate::profile::ArchProfile>,
) -> Result<impl IntoResponse, AppError> {
    profile.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let rooms = SearuApi::optimize_floorplan(profile);
    let svg_str = FloorPlanner::to_svg_string(&rooms);
    Ok(([(header::CONTENT_TYPE, "image/svg+xml")], svg_str))
}


async fn api_megacity_pipeline(
    axum::Json(profile): axum::Json<crate::profile::MegaCityProfile>,
) -> Result<impl IntoResponse, AppError> {
    profile.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let result_json = MegaCityPipeline::run_pipeline(profile);
    Ok(([(header::CONTENT_TYPE, "application/json")], result_json))
}


async fn api_album_tracks() -> impl IntoResponse {
    let tracks = SearuApi::list_album_tracks();
    Json(tracks)
}

async fn api_serve_album_track(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let safe_name = std::path::Path::new(&filename)
        .file_name()
        .and_then(|f| f.to_str())
        .ok_or_else(|| AppError::Validation("Invalid filename".to_string()))?;
    let path = format!("release/{}", safe_name);

    let bytes = std::fs::read(&path)
        .map_err(|_| AppError::NotFound(format!("Track file '{}' not found", safe_name)))?;

    let content_type = if safe_name.ends_with(".wav") {
        "audio/wav"
    } else if safe_name.ends_with(".svg") {
        "image/svg+xml"
    } else if safe_name.ends_with(".mid") {
        "audio/midi"
    } else {
        "application/octet-stream"
    };
    Ok(([(header::CONTENT_TYPE, content_type)], bytes))
}

async fn autonomous_pulse() {
    use crate::science::crucible::{TheCrucible, Gene};
    use crate::science::oracle::get_oracle;
    use tokio::time::{sleep, Duration};
    use rand::Rng;

    // Ensure the oracle is initialized and loaded from disk right away
    {
        drop(get_oracle().lock().unwrap());
    }

    loop {
        // Sleep between 5 and 15 seconds to simulate background optimization cadence
        let sleep_duration = rand::rng().random_range(5..15);
        sleep(Duration::from_secs(sleep_duration)).await;

        let mut rng = rand::rng();
        let domain = match rng.random_range(0..2) {
            0 => crate::science::oracle::DomainContext::Architecture {
                height: rng.random_range(0.1..1.0),
                stress: rng.random_range(0.1..1.0),
            },
            _ => crate::science::oracle::DomainContext::Music {
                tension: rng.random_range(0.1..1.0),
                density: rng.random_range(0.1..1.0),
            },
        };

        println!("⚙️ [Background Worker] Optimizing domain: {:?}", domain);

        let genome_dim = get_oracle().lock().unwrap().genome_dimension;
        let initial_genes: Vec<Gene> = (0..genome_dim).map(|i| Gene {
            name: format!("param_{}", i),
            bounds: (0.0, 1.0),
            current_value: rng.random_range(0.0..1.0),
        }).collect();
        
        let (fit, _, final_genes) = TheCrucible::anneal_with_sublime(
            initial_genes,
            domain,
            |genes| {
                crate::science::universal_objective::evaluate_dissonance(genes)
            },
            500 // Short background step
        );

        let music_decode = crate::music::composer::decode_genes_to_midi(&final_genes);

        println!("✨ [Generator Engine] Multi-domain synthesis completed");
        println!("   -> Global Loss Score: {:.4}", fit);
        println!("   -> Audio Score: {}", music_decode);
    }
}

static ALBUM_IN_PROGRESS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

async fn api_album_release() -> Result<impl IntoResponse, AppError> {
    use std::sync::atomic::Ordering;
    if ALBUM_IN_PROGRESS.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err(AppError::Conflict("Album production is already in progress. Please wait for the current batch to finish.".to_string()));
    }

    std::thread::spawn(|| {
        album::AlbumProducer::release_album(10);
        ALBUM_IN_PROGRESS.store(false, Ordering::SeqCst);
    });
    
    Ok((
        axum::http::StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "status": "success",
            "message": "Parallel Album production (10 tracks) started! Check the /release directory in a few seconds."
        })),
    ))
}

async fn api_synesthesia_generate(
    axum::Json(intent): axum::Json<crate::intent::DesignIntent>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    let (obj_path, wav_path) = crate::synesthesia::SynesthesiaEngine::generate_experience(intent)
        .map_err(|e| AppError::Internal(e))?;
    
    Ok(axum::Json(serde_json::json!({
        "status": "success",
        "architecture_obj": obj_path,
        "music_wav": wav_path
    })))
}

#[derive(serde::Deserialize)]
pub struct ScriptRequest {
    pub script: String,
}

async fn api_script_run(
    axum::Json(req): axum::Json<ScriptRequest>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    let mut is_high_level = req.script.contains("let ") || req.script.contains("=");
    let mut code = vec![];
    let mut vars_map = std::collections::HashMap::new();

    if is_high_level {
        let (bytecode, vmap) = script_go::compiler::compile_high_level(&req.script)
            .map_err(|e| AppError::Internal(format!("DSL Compile Error: {}", e)))?;
        code = bytecode;
        // Convert no_std_tool HashMap to std::collections::HashMap if needed, or just iterate
        for (k, v) in vmap {
            vars_map.insert(k.clone(), v);
        }
    } else {
        code = script_go::assembler::parse_asm(&req.script)
            .map_err(|e| AppError::Internal(format!("Script Compile Error: {:?}", e)))?;
    }

    let mut vm = script_go::vm::ScriptVm::new();
    vm.run(&code).map_err(|e| AppError::Internal(format!("Script Runtime Error: {:?}", e)))?;
    
    // Extract variables. If high-level, use map. If assembly, use R1-R4
    let mut r1 = vm.registers[1] as f64;
    let mut r2 = vm.registers[2] as f64;
    let mut r3 = vm.registers[3] as f64;
    let mut r4 = vm.registers[4] as f64;

    if is_high_level {
        if let Some(&reg) = vars_map.get("aggression") { r1 = vm.registers[reg as usize] as f64; }
        if let Some(&reg) = vars_map.get("elegance") { r2 = vm.registers[reg as usize] as f64; }
        if let Some(&reg) = vars_map.get("density") { r3 = vm.registers[reg as usize] as f64; }
        if let Some(&reg) = vars_map.get("industrialism") { r4 = vm.registers[reg as usize] as f64; }
    }

    let aggression = r1 / 100.0;
    let elegance = r2 / 100.0;
    let density = r3 / 100.0;
    let industrialism = r4 / 100.0;

    let intent = crate::intent::DesignIntent {
        aggression: aggression.clamp(0.0, 1.0),
        elegance: elegance.clamp(0.0, 1.0),
        density: density.clamp(0.0, 1.0),
        industrialism: industrialism.clamp(0.0, 1.0),
    };

    let (obj_path, wav_path) = crate::synesthesia::SynesthesiaEngine::generate_experience(intent)
        .map_err(|e| AppError::Internal(e))?;

    Ok(axum::Json(serde_json::json!({
        "status": "success",
        "architecture_obj": obj_path,
        "music_wav": wav_path,
        "registers": {
            "r1": r1 as u64,
            "r2": r2 as u64,
            "r3": r3 as u64,
            "r4": r4 as u64,
        }
    })))
}
