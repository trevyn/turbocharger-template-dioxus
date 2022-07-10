use turbocharger::prelude::*;

mod app;

#[wasm_only]
#[wasm_bindgen]
pub fn start_web() {
    dioxus::web::launch(app::App);
}
