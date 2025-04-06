mod args;
mod settings;
mod etcher;
mod run_tasks;
mod youtube_dl;
mod ui;
mod embedsource;
mod timer;

use clap::Parser;
use crate::args::Arguments;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Steganographic Data Handling");
    
    let args = Arguments::parse();
    
    match args.command {
        Some(cmd) => run_tasks::run_by_arguments(cmd).await?,
        None => {
            // Use interactive UI when no command is provided
            let new_command = ui::enrich_arguments(None).await?;
            run_tasks::run_by_arguments(new_command).await?;
        }
    }
    
    Ok(())
}