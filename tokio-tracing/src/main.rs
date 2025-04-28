use tracing_subscriber::fmt::format::FmtSpan;

#[tracing::instrument]
async fn hello_world() {
    println!("Hello!");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber= tracing_subscriber::fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .with_thread_ids(true)
    .with_target(false)
    .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
    .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up");
    tracing::warn!("Good idea?");
    tracing::error!("RUH ROH!");
    
    hello_world().await;

    Ok(())
}
