#![allow(unused)]
#![allow(dead_code)]

mod album;
mod api;
mod architecture;
mod fractal;
mod materials;
mod mechanics;
mod megacity;
mod music;
mod pcb_routing;
mod procedural_animation;
pub mod profile;
pub mod language;
mod science;
mod sensory;
mod typography;
mod ui_layout;
mod visual;

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
use fractal::FractalEngine;
use mechanics::exporter::TrussExporter;
use megacity::MegaCityPipeline;
use music::dsp::exporter::AudioExporter;
use pcb_routing::PcbRouter;
use typography::TypographyGenerator;
use visual::exporter::SvgExporter;

#[tokio::main]
async fn main() {
    println!("🚀 Starting SEARU Autonomous Aesthetic Entity on http://localhost:3000");
    println!("📜 [Agentic Autopoiesis] Injected Physics Laws Loaded:");
    println!("   -> {}", crate::science::dynamic_laws::get_injected_laws());

    tokio::spawn(autonomous_pulse());

    // Initialize telemetry channel (capacity 1000)
    let (tx, _) = tokio::sync::broadcast::channel(1000);
    crate::science::crucible::TELEMETRY_TX.set(tx).unwrap();

    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/api/music/bach", get(api_music_bach).post(api_music_bach_post))
        .route("/api/music/generate", post(api_music_generate))
        .route("/api/music/fm", get(api_music_fm))
        .route("/api/visual/art", get(api_visual_art).post(api_visual_art_post))
        .route("/api/mechanics/truss", get(api_mechanics_truss).post(api_mechanics_truss))
        .route("/api/materials/match", get(api_materials_match).post(api_materials_match_post))
        .route("/api/architecture/floorplan", get(api_arch_floorplan).post(api_arch_floorplan_post))
        .route("/api/ui_layout/optimize", get(api_ui_layout).post(api_ui_layout))
        .route("/api/pcb_routing/route", get(api_pcb_route).post(api_pcb_route))
        .route("/api/typography/glyph", get(api_typography_glyph).post(api_typography_glyph))
        .route("/api/procedural_animation/curve", get(api_anim_curve).post(api_anim_curve))
        .route("/api/megacity/pipeline", post(api_megacity_pipeline))
        .route("/api/science/multidomain_fuzz", post(api_science_multidomain_fuzz))
        .route("/api/fractal/universe", get(api_fractal_universe).post(api_fractal_universe))
        .route("/api/album/release", get(api_album_release).post(api_album_release))
        .route("/api/album/tracks", get(api_album_tracks))
        .route("/api/album/track/:filename", get(api_serve_album_track))
        .route("/api/telemetry", get(api_telemetry))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

async fn api_music_bach() -> impl IntoResponse {
    let sample_rate = 44100;
    let bach_audio_buffer =
        SearuApi::generate_bach_progression(&[60.0, 64.0, 67.0], 4, 1.2, sample_rate);
    let bytes = AudioExporter::encode_to_wav_bytes(&bach_audio_buffer, sample_rate).unwrap();
    ([(header::CONTENT_TYPE, "audio/wav")], bytes)
}

async fn api_music_bach_post(
    axum::Json(req): axum::Json<api::BachRequest>,
) -> impl IntoResponse {
    let sample_rate = 44100;
    let root = req.root_note.unwrap_or(60.0);
    let num_chords = req.num_chords.unwrap_or(4);
    let sec = req.seconds_per_chord.unwrap_or(1.2);
    let start_chord = [root, root + 4.0, root + 7.0];

    let bach_audio_buffer =
        SearuApi::generate_bach_progression(&start_chord, num_chords, sec, sample_rate);
    let bytes = AudioExporter::encode_to_wav_bytes(&bach_audio_buffer, sample_rate).unwrap();
    ([(header::CONTENT_TYPE, "audio/wav")], bytes)
}

async fn api_music_fm(
    axum::extract::Query(query): axum::extract::Query<api::FmRequest>,
) -> impl IntoResponse {
    let diss = query.dissonance.unwrap_or(0.5);
    let patch = crate::music::fm_synth::FmOptimizer::optimize_patch(diss);
    let buffer = crate::music::fm_synth::FmSynthesizer::render(&patch, 440.0, 2.0, 44100);
    let bytes = AudioExporter::encode_to_wav_bytes(&buffer, 44100).unwrap();
    ([(header::CONTENT_TYPE, "audio/wav")], bytes)
}

async fn api_music_generate(
    axum::Json(profile): axum::Json<crate::profile::ArtistProfile>,
) -> impl IntoResponse {
    let sample_rate = 44100;
    let audio_buffer = SearuApi::generate_music_with_profile(&profile);
    let bytes = AudioExporter::encode_to_wav_bytes(&audio_buffer, sample_rate).unwrap();
    ([(header::CONTENT_TYPE, "audio/wav")], bytes)
}

async fn api_visual_art() -> impl IntoResponse {
    let shapes = SearuApi::generate_visual_art(12, 200.0, 8);
    let svg_str = SvgExporter::to_svg_string(&shapes);
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
}

async fn api_visual_art_post(
    axum::Json(req): axum::Json<api::VisualRequest>,
) -> impl IntoResponse {
    let num_shapes = req.num_shapes.unwrap_or(12);
    let base_hue = req.base_hue.unwrap_or(200.0);
    let depth = req.fractal_depth.unwrap_or(8);
    let shapes = SearuApi::generate_visual_art(num_shapes, base_hue, depth);
    let svg_str = SvgExporter::to_svg_string(&shapes);
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
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
) -> impl IntoResponse {
    let rooms = SearuApi::optimize_floorplan(profile);
    let svg_str = FloorPlanner::to_svg_string(&rooms);
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
}

async fn api_ui_layout() -> impl IntoResponse {
    Json(SearuApi::optimize_ui_layout())
}

async fn api_pcb_route() -> impl IntoResponse {
    let traces = SearuApi::route_pcb();
    let svg_str = PcbRouter::to_svg_string(&traces);
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
}

async fn api_typography_glyph() -> impl IntoResponse {
    let glyph = SearuApi::generate_glyph();
    let svg_str = TypographyGenerator::to_svg_string(&glyph);
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
}

async fn api_anim_curve() -> impl IntoResponse {
    Json(SearuApi::optimize_animation_transition())
}

async fn api_megacity_pipeline(
    axum::Json(profile): axum::Json<crate::profile::MegaCityProfile>,
) -> impl IntoResponse {
    let svg_str = MegaCityPipeline::run_pipeline(profile);
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
}

async fn api_fractal_universe() -> impl IntoResponse {
    let svg_str = FractalEngine::generate_universe();
    ([(header::CONTENT_TYPE, "image/svg+xml")], svg_str)
}

async fn api_album_tracks() -> impl IntoResponse {
    let tracks = SearuApi::list_album_tracks();
    Json(tracks)
}

async fn api_serve_album_track(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> impl IntoResponse {
    let safe_name = std::path::Path::new(&filename)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("default");
    let path = format!("release/{}", safe_name);

    if let Ok(bytes) = std::fs::read(&path) {
        let content_type = if safe_name.ends_with(".wav") {
            "audio/wav"
        } else if safe_name.ends_with(".svg") {
            "image/svg+xml"
        } else if safe_name.ends_with(".mid") {
            "audio/midi"
        } else {
            "application/octet-stream"
        };
        ([(header::CONTENT_TYPE, content_type)], bytes).into_response()
    } else {
        (axum::http::StatusCode::NOT_FOUND, "Track file not found").into_response()
    }
}

async fn autonomous_pulse() {
    use crate::science::crucible::{TheCrucible, Gene};
    use crate::science::oracle::{get_oracle, DomainContext};
    use tokio::time::{sleep, Duration};
    use rand::Rng;

    // Ensure the oracle is initialized and loaded from disk right away
    {
        drop(get_oracle().lock().unwrap());
    }

    loop {
        // Sleep between 5 and 15 seconds to simulate background thinking/dreaming
        let sleep_duration = rand::rng().random_range(5..15);
        sleep(Duration::from_secs(sleep_duration)).await;

        let mut rng = rand::rng();
        let domain = match rng.random_range(0..3) {
            0 => DomainContext::Architecture {
                height: rng.random_range(0.1..1.0),
                stress: rng.random_range(0.1..1.0),
            },
            1 => DomainContext::Music {
                tension: rng.random_range(0.1..1.0),
                density: rng.random_range(0.1..1.0),
            },
            _ => DomainContext::Language {
                prosody_complexity: rng.random_range(0.1..1.0),
                emotional_tension: rng.random_range(0.1..1.0),
            },
        };

        println!("💭 [Autonomous Pulse] Dreaming about {:?}", domain);

        let genome_dim = get_oracle().lock().unwrap().genome_dimension;
        let initial_genes: Vec<Gene> = (0..genome_dim).map(|i| Gene {
            name: format!("dream_gene_{}", i),
            bounds: (0.0, 1.0),
            current_value: rng.random_range(0.0..1.0),
        }).collect();
        
        let (fit, _, final_genes) = TheCrucible::anneal_with_sublime(
            initial_genes,
            domain,
            |genes| {
                crate::science::universal_objective::evaluate_dissonance(genes)
            },
            500 // Short dream iteration
        );

        let lang_decode = crate::language::conlang::decode_genes_to_language(&final_genes);
        let music_decode = crate::music::composer::decode_genes_to_midi(&final_genes);
        let choreo_decode = crate::sensory::choreography::decode_genes_to_choreography(&final_genes);
        let gastro_decode = crate::sensory::gastronomy::decode_genes_to_gastronomy(&final_genes);

        println!("🌌 [Universal Decoder] Epiphany rendered!");
        println!("   -> Universal Fitness: {:.4}", fit);
        println!("   -> Language Projection: {}", lang_decode);
        println!("   -> Music Projection: {}", music_decode);
        println!("   -> Choreography Projection: {}", choreo_decode);
        println!("   -> Gastronomy Projection: {}", gastro_decode);
    }
}

async fn api_album_release() -> impl IntoResponse {
    std::thread::spawn(|| {
        album::AlbumProducer::release_album(10);
    });
    Json(
        serde_json::json!({"status": "Parallel Album production (10 tracks) started! Check the /release directory in a few seconds."}),
    )
}
