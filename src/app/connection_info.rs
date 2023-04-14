use turbocharger::prelude::*;

#[backend]
async fn get_connection_info() -> String {
    format!("remote_addr: {:?}, user_agent: {:?}", remote_addr!(), user_agent!())
}

#[frontend]
pub fn ConnectionInfo(cx: Scope) -> Element {
    use_future(cx, (), |_| get_connection_info()).value().and_then(|r| render!("{r}"))
}
