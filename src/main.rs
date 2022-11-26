mod app;

use clap::Parser;

#[derive(clap::Parser)]
struct Opts {
    #[clap(short, long)]
    cert_path: Option<String>,
    #[clap(short, long)]
    key_path: Option<String>,
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    #[derive(rust_embed::RustEmbed)]
    #[folder = "src-frontend/dist"]
    struct Frontend;

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    log::warn!("warn enabled");
    log::info!("info enabled");
    log::debug!("debug enabled");
    log::trace!("trace enabled");

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], opts.port));

    match (opts.key_path, opts.cert_path) {
        (Some(_key_path), Some(_cert_path)) => {
            eprintln!("Serving HTTPS on port {}", opts.port);
            turbocharger::serve_tls::<Frontend>(&addr).await;
        }
        (None, None) => {
            eprintln!("Serving (unsecured) HTTP on port {}", opts.port);
            #[cfg(debug_assertions)]
            opener::open(format!("http://127.0.0.1:{}", 3000)).ok();
            turbocharger::serve::<Frontend>(&addr).await;
        }
        _ => eprintln!("Both key-path and cert-path must be specified for HTTPS."),
    }
}
