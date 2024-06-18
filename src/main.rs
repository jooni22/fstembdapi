// src/main.rs
use fastembed_axum::server::run::start_server;
use fastembed_axum::embedding::ModelSource;

fn main() {
    // Wywołaj funkcję start_server z odpowiednimi argumentami
    start_server(None, ModelSource::HuggingFace);
}