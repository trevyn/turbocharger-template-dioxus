use turbocharger::prelude::*;

#[backend]
fn stream() -> impl Stream<Item = String> {
    stream! {
     let mut i = 0;
     loop {
      yield format!("{}", i);
      i += 1;
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
     }
    }
}

#[frontend]
pub fn Stream(cx: Scope) -> Element {
    use_stream(&cx, stream, |s, v| *s = Some(v))
        .read()
        .as_ref()
        .and_then(|r| rsx!(cx, p { "stream: {r}" }))
}
