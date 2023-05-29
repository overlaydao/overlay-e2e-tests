use clap::Parser;
use overlay_e2e_tests::Cli;
use tracing::error;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .unwrap();
    let cli = Cli::parse();
    let exit_code = match cli.run().await {
        Ok(_) => 0,
        Err(err) => {
            error!("{:?}", err);
            1
        },
    };
    std::process::exit(exit_code);
}
