#![allow(non_snake_case)]
#![cfg_attr(feature = "wasm", allow(dead_code))]

use turbocharger::prelude::*;

mod connection_info;
mod connection_local;
mod stream;
mod stream_result;
mod turbosql;

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
