use turbocharger::prelude::*;

#[backend]
fn stream_result() -> impl Stream<Item = Result<String, tracked::StringError>> {
    try_stream!({
        let mut i = 0;
        loop {
            yield format!("{}", i);
            i += 1;
            if i == 5 {
                None?;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    })
}

#[frontend]
pub fn StreamResult(cx: Scope) -> Element {
    use_stream(&cx, stream_result, |s, v| *s = Some(v)).read().as_ref().and_then(|r| match r {
        Ok(r) => rsx!(cx, p { "stream_result: {r}" }),
        Err(e) => rsx!(cx, p { "stream_result handling an error: {e}" }),
    })
}
