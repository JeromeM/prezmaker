mod cli;
mod orchestrator;

use clap::Parser;
use cli::Cli;
use dialoguer::Select;
use prezmaker_lib::config::Config;
use prezmaker_lib::models::Tracker;
use orchestrator::Orchestrator;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Setup logging
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    // Load config
    let config = match Config::load(cli.config.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Erreur de configuration : {}", e);
            std::process::exit(1);
        }
    };

    // Validate color if provided
    if let Some(ref color) = cli.color {
        if !is_valid_hex_color(color) {
            eprintln!(
                "Couleur invalide : {}. Utilisez un code hex (ex: c0392b)",
                color
            );
            std::process::exit(1);
        }
    }

    // Tracker selection
    let tracker = {
        let items = &["C411", "torr.xyz"];
        let selection = Select::new()
            .with_prompt("Selectionnez le tracker")
            .items(items)
            .default(0)
            .interact()
            .unwrap_or_else(|_| {
                eprintln!("Selection annulee");
                std::process::exit(1);
            });
        match selection {
            0 => Tracker::C411,
            _ => Tracker::TorrXyz,
        }
    };

    let orchestrator = Orchestrator::new(config, cli.language.clone(), cli.color.clone(), tracker);

    match orchestrator.run(&cli.command).await {
        Ok(bbcode) => {
            println!("{}", bbcode);

            // Clipboard
            if cli.clipboard {
                match copy_to_clipboard(&bbcode) {
                    Ok(_) => eprintln!("BBCode copie dans le presse-papiers !"),
                    Err(e) => eprintln!("Impossible de copier dans le presse-papiers : {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Erreur : {}", e);
            std::process::exit(1);
        }
    }
}

fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard =
        arboard::Clipboard::new().map_err(|e| format!("Clipboard init failed: {}", e))?;
    clipboard
        .set_text(text)
        .map_err(|e| format!("Clipboard set failed: {}", e))?;
    Ok(())
}

fn is_valid_hex_color(s: &str) -> bool {
    let s = s.trim_start_matches('#');
    s.len() == 6 && s.chars().all(|c| c.is_ascii_hexdigit())
}
