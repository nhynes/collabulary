#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

mod game;
mod server;
mod vocab;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let vocab = vocab::Vocab::load_from_path("../vocab/chinese/vocab.tsv")?;
    let game = game::Game::new(vocab);

    warp::serve(server::filters(game))
        .bind(("0.0.0.0".parse::<std::net::IpAddr>().unwrap(), 1234))
        .await;

    Ok(())
}

fn init_tracing() {
    let base_subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_target(true);

    if cfg!(not(debug_assertions)) {
        base_subscriber.json().with_ansi(false).init();
    } else {
        base_subscriber
            .without_time()
            .pretty()
            .with_test_writer()
            .compact()
            .try_init()
            .ok();
    }
}
