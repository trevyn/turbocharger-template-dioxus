#![allow(non_snake_case)]
#![cfg_attr(feature = "wasm", allow(dead_code))]

use turbocharger::prelude::*;

automod::dir!("src/app");

#[frontend]
pub fn App(cx: Scope) -> Element {
    rsx!(cx,
        p { "Hello World!" },
        p { connection_info::ConnectionInfo() },
        p { connection_local::ConnectionLocal() }
        p { stream::Stream() }
        p { stream_result::StreamResult() }
        p { turbosql::AdminWithAuth() }
    )
}
