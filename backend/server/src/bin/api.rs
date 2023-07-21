use std::net::SocketAddr;

use clap::{Parser, Subcommand};
use color_eyre::{eyre::Context, Help, Result};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(
        short,
        long,
        default_value = "postgres://test@localhost/test",
        env = "API_DATABASE_URL"
    )]
    database_url: String,

    #[clap(short, long, default_value = "127.0.0.1:8070", env = "API_BIND")]
    bind: SocketAddr,

    #[clap(flatten)]
    verbosity: uchat_server::logging::Verbosity,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// generate a session signing key
    GenKey,
}

async fn run() -> Result<()> {
    color_eyre::install()?;

    let use_dotenv = dotenvy::dotenv();

    let args = Cli::parse();

    uchat_server::logging::setup(args.verbosity);

    match use_dotenv {
        Ok(path) => {
            tracing::debug!(target: "uchat_server", dot_env_found = true, path= %path.to_string_lossy())
        }
        Err(_) => tracing::debug!(target: "uchat_server", dot_env_found = false),
    }

    if let Some(Command::GenKey) = args.command {
        let mut rng = uchat_crypto::new_rng();
        tracing::info!(target: "uchat_server", "generating private key...");
        let (key, _) = uchat_server::cli::gen_keys(&mut rng)?;
        let path = "private_key.base64";
        std::fs::write(path, key.as_str())?;
        tracing::info!(target: "uchat_server", path=path, "private key saved to disk");
        tracing::info!(target: "uchat_server", path=path, "set API_PRIVATE_KEY environment variable with the content of the key in order to use it");
    }

    tracing::debug!(target: "uchat_server", "loading signing keys");
    let signing_keys = uchat_server::cli::load_keys()?;

    tracing::info!(target: "uchat_server", database_url = args.database_url, "connecting to database");
    let db_pool = uchat_query::AsyncConnectionPool::new(&args.database_url)
        .await
        .with_suggestion(|| "check database URL")
        .with_suggestion(|| "ensure correct database access rights")
        .with_suggestion(|| "make sure database exists")?;

    let state = uchat_server::AppState {
        db_pool,
        signing_keys,
        rng: uchat_crypto::new_rng(),
    };

    tracing::info!(target: "uchat_server", bind_addr = %args.bind);

    let router = uchat_server::router::new_router(state);

    let server = axum::Server::try_bind(&args.bind)
        .wrap_err_with(|| "server initialization error")
        .with_suggestion(|| "check bind address")
        .with_suggestion(|| "check if other services are using the same port")?;

    let server = server.serve(router.into_make_service());

    tracing::info!(target: "uchat_server", "listening");

    if let Err(e) = server.await {
        tracing::error!(target: "uchat_server",  server_error = %e);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
