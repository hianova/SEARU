mod api;
mod science;
mod music;
mod visual;
mod mechanics;
mod materials;
mod architecture;
mod ui_layout;
mod pcb_routing;
mod typography;
mod procedural_animation;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::header,
    Json
};
use serde::Serialize;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;

use api::SearuApi;
use music::dsp::exporter::AudioExporter;
use visual::exporter::SvgExporter;
use mechanics::exporter::TrussExporter;
use architecture::FloorPlanner;
use pcb_routing::PcbRouter;
use typography::TypographyGenerator;

#[tokio::main]
async fn main() {
    println!("🚀 Starting SEARU Web UI Server on http://localhost:3000");

    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/api/music/bach", get(api_music_bach))
        .route("/api/visual/art", get(api_visual_art))
        .route("/api/mechanics/truss", get(api_mechanics_truss))
        .route("/api/materials/match", get(api_materials_match))
        .route("/api/architecture/floorplan", get(api_arch_floorplan))
        .route("/api/ui_layout/optimize", get(api_ui_layout))
        .route("/api/pcb_routing/route", get(api_pcb_route))
        .route("/api/typography/glyph", get(api_typography_glyph))
        .route("/api/procedural_animation/curve", get(api_anim_curve))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn api_music_bach() -> impl IntoResponse {
    let sample_rate = 44100;
    let bach_audio_buffer = SearuApi::generate_bach_progression(&[60.0, 64.0, 67.0], 4, 1.2, sample_rate);
    let bytes = AudioExporter::encode_to_wav_bytes(&bach_audio_buffer, sample_rate).unwrap();
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
    Json(PbrResponse { albedo: mat.albedo, roughness: mat.roughness, metallic: mat.metallic })
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
