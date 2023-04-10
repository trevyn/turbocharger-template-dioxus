use turbocharger::prelude::*;

mod app;

#[wasm_only]
#[wasm_bindgen]
pub fn start_web() {
    dioxus_web::launch(app::App);
}
