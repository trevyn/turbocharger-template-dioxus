mod app;

// server

#[cfg(not(target_arch = "wasm32"))]
mod server;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    server::main().await;
}

// wasm

#[cfg(target_arch = "wasm32")]
use turbocharger::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    // wasm_logger::init(wasm_logger::Config::default());
    // console_error_panic_hook::set_once();

    dioxus_web::launch(app::App);
}
