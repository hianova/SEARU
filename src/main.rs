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
mod science;
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
    println!("🚀 Starting SEARU Web UI Server on http://localhost:3000");

    // Initialize telemetry channel (capacity 1000)
    let (tx, _) = tokio::sync::broadcast::channel(1000);
    crate::science::crucible::TELEMETRY_TX.set(tx).unwrap();

    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/api/music/bach", get(api_music_bach))
        .route("/api/music/generate", post(api_music_generate))
        .route("/api/visual/art", get(api_visual_art))
        .route("/api/mechanics/truss", get(api_mechanics_truss))
        .route("/api/materials/match", get(api_materials_match))
        .route("/api/architecture/floorplan", get(api_arch_floorplan))
        .route("/api/ui_layout/optimize", get(api_ui_layout))
        .route("/api/pcb_routing/route", get(api_pcb_route))
        .route("/api/typography/glyph", get(api_typography_glyph))
        .route("/api/procedural_animation/curve", get(api_anim_curve))
        .route("/api/megacity/pipeline", post(api_megacity_pipeline))
        .route("/api/fractal/universe", get(api_fractal_universe))
        .route("/api/album/release", get(api_album_release))
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

async fn api_music_bach() -> impl IntoResponse {
    let sample_rate = 44100;
    let bach_audio_buffer =
        SearuApi::generate_bach_progression(&[60.0, 64.0, 67.0], 4, 1.2, sample_rate);
    let bytes = AudioExporter::encode_to_wav_bytes(&bach_audio_buffer, sample_rate).unwrap();
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
    let shapes = SearuApi::generate_visual_art(10, 5);
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

async fn api_arch_floorplan() -> impl IntoResponse {
    let rooms = SearuApi::optimize_floorplan();
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

async fn api_album_release() -> impl IntoResponse {
    std::thread::spawn(|| {
        album::AlbumProducer::release_album(10);
    });
    Json(
        serde_json::json!({"status": "Parallel Album production (10 tracks) started! Check the /release directory in a few seconds."}),
    )
}
