
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

    tokio::spawn(autonomous_pulse());

    // Initialize telemetry channel (capacity 1000)
    let (tx, _) = tokio::sync::broadcast::channel(1000);
    crate::science::crucible::TELEMETRY_TX.set(tx).unwrap();

    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .nest_service("/release", ServeDir::new("release"))
        .route("/api/music/generate", post(api_music_generate))
        .route("/api/synesthesia", post(api_synesthesia_generate))
        .route("/api/music/fm", get(api_music_fm))
        .route("/api/mechanics/truss", get(api_mechanics_truss))
        .route("/api/materials/match", get(api_materials_match))
        .route("/api/materials/match", post(api_materials_match_post))
        .route("/api/arch/floorplan", get(api_arch_floorplan))
        .route("/api/arch/floorplan", post(api_arch_floorplan_post))
        .route("/api/megacity/pipeline", post(api_megacity_pipeline))
        .route("/api/science/multidomain_fuzz", post(api_science_multidomain_fuzz))
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
        if let Err(e) = o.engine.save("searu.engram") {
            println!("❌ Failed to flush searu.engram: {}", e);
        } else {
            println!("💾 Oracle memories successfully flushed to searu.engram");
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

#[derive(Serialize)]
pub struct MultidomainResponse {
    pub final_score: u32,
    pub genes: [f64; 9],
}

async fn api_science_multidomain_fuzz() -> impl IntoResponse {
    let objective = crate::science::multidomain_fuzz::MultiDomainFuzzObjective::new(25.0, 100.0, 0.5);
    let config = crate::science::assembly_funnel::FunnelConfig {
        tier1_population: 1000,
        tier2_retention_ratio: 0.1,
        tier3_dfs_depth: 2,
        stagnation_patience: 10,
        stagnation_delta: 0.5,
        rng_seed: 42,
        min_slope_window: 0,
        min_slope_threshold: 0.0,
        hard_limit_gen: 500,
        hard_limit_score: 0,
        use_diffusion: true,
    };
    
    let (result, best_candidate) = crate::science::chaos_runner::ChaosRunner::launch_tunable(objective, config, "Multi-Domain Co-evolution");
    
    let best_genes = best_candidate.unwrap_or([0.0; 9]);
    
    Json(MultidomainResponse {
        final_score: result.best_score(),
        genes: best_genes,
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
