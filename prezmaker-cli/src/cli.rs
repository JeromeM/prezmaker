use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "prezmaker",
    about = "Generateur de presentations BBCode",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Copier le BBCode dans le presse-papiers
    #[arg(short = 'c', long = "clipboard", global = true)]
    pub clipboard: bool,

    /// Langue (defaut: fr-FR)
    #[arg(short = 'l', long = "language", global = true)]
    pub language: Option<String>,

    /// Mode debug
    #[arg(short = 'v', long = "verbose", global = true)]
    pub verbose: bool,

    /// Chemin config alternatif
    #[arg(long = "config", global = true)]
    pub config: Option<String>,

    /// Couleur titre hex (defaut: c0392b)
    #[arg(long = "color", global = true)]
    pub color: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generer une fiche BBCode pour un film
    Film {
        /// Titre du film a rechercher
        query: String,

        /// Desactiver l'enrichissement Allocine
        #[arg(long = "no-allocine")]
        no_allocine: bool,
    },

    /// Generer une fiche BBCode pour une serie
    Serie {
        /// Titre de la serie a rechercher
        query: String,

        /// Desactiver l'enrichissement Allocine
        #[arg(long = "no-allocine")]
        no_allocine: bool,
    },

    /// Generer une fiche BBCode pour un jeu video
    Jeu {
        /// Titre du jeu a rechercher
        query: String,
    },

    /// Generer une fiche BBCode pour une application
    App {
        /// Nom de l'application
        name: String,

        /// Version
        #[arg(long)]
        version: Option<String>,

        /// Developpeur
        #[arg(long)]
        developer: Option<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Site web
        #[arg(long)]
        website: Option<String>,

        /// Licence
        #[arg(long)]
        license: Option<String>,

        /// URL du logo
        #[arg(long)]
        logo: Option<String>,

        /// Plateformes (separees par des virgules)
        #[arg(long)]
        platforms: Option<String>,
    },
}
