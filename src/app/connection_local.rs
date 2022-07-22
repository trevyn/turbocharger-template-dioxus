use turbocharger::prelude::*;

#[backend]
async fn get_connection_local_number() -> i64 {
    connection_local!(number: &mut i64);
    *number += 1;
    *number
}

#[frontend]
pub fn ConnectionLocal(cx: Scope) -> Element {
    let number = use_state(&cx, || 0i64);

    use_future(&cx, (), |_| {
        to_owned![number];
        async move {
            loop {
                wait_ms(1000).await;
                number.set(get_connection_local_number().await);
                number.needs_update();
            }
        }
    });

    rsx!(cx, "connection_local: {number}")
}
