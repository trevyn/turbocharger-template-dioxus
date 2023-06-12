#![allow(non_snake_case)]
#![cfg_attr(feature = "wasm", allow(dead_code))]

use turbocharger::prelude::*;

turbomod!("src/app");

#[frontend]
pub fn App(cx: Scope) -> Element {
    render!(
        p { "Hello World!" },
        p { connection_info::ConnectionInfo{} },
        p { connection_local::ConnectionLocal{} }
        p { stream::Stream{} }
        p { stream_result::StreamResult{} }
        p { turbosql::AdminWithAuth{} }
    )
}
