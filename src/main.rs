use clap::Parser;
use overlay_e2e_tests::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let exit_code = match cli.run().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        },
    };
    std::process::exit(exit_code);
}
