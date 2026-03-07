use std::process::Command;
use tracing::debug;

pub struct ClaudeClient;

impl ClaudeClient {
    pub fn new() -> Self {
        Self
    }

    /// Verifie que la CLI claude est disponible
    pub fn is_available(&self) -> bool {
        Command::new("claude")
            .arg("--version")
            .output()
            .is_ok()
    }

    /// Ecrit une description de jeu en francais via claude CLI
    pub fn write_game_description(&self, game_title: &str, english_summary: Option<&str>) -> anyhow::Result<String> {
        debug!("Generation description jeu via claude CLI : {}", game_title);

        let context = match english_summary {
            Some(en) => format!(
                "\n\nVoici un resume en anglais pour t'aider :\n{}",
                en
            ),
            None => String::new(),
        };

        let prompt = format!(
            "Ecris une description en francais pour le jeu video \"{}\". \
            La description doit etre engageante, informative et faire environ 2-3 paragraphes. \
            Retourne uniquement la description, sans titre, sans commentaire ni explication.{}",
            game_title, context
        );

        let output = Command::new("claude")
            .arg("-p")
            .arg(&prompt)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("claude CLI error: {}", stderr);
        }

        let text = String::from_utf8(output.stdout)?.trim().to_string();
        debug!("Reponse claude CLI : {} caracteres", text.len());
        Ok(text)
    }
}
