use crate::args::Commands;

pub mod embed;
pub mod dislodge;
pub mod download;

pub async fn run_by_arguments(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Embed(args) => embed::run_embed(args).await,
        Commands::Dislodge(args) => dislodge::run_dislodge(args).await,
        Commands::Download(args) => download::run_download(args).await,
    }
}