use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

const SYSTEM_PROMPT: &str = "Tu es un redacteur specialise en jeux video. \
Tu reponds UNIQUEMENT en francais. Toute ta production doit etre integralement en langue francaise. \
Tu ne dois jamais repondre en anglais, meme si le contenu fourni est en anglais. \
Tu retournes uniquement la description demandee, sans titre, sans commentaire, sans explication.";

const NFO_SYSTEM_PROMPT: &str = r#"Tu es un generateur de fichiers NFO pour des releases de contenus multimedia.
A partir du BBCode fourni, genere un fichier NFO en texte brut avec un style ASCII art.

Regles :
- Utilise des separateurs avec des caracteres === ou --- pour delimiter les sections
- Aligne le texte proprement (largeur ~70 caracteres)
- Extrais les informations du BBCode : titre, description, infos techniques, etc.
- Le resultat doit etre du texte brut uniquement, pas de BBCode, pas de HTML
- Retourne UNIQUEMENT le contenu NFO, sans commentaire ni explication
- Le texte doit rester en francais

Exemple de structure :
======================================================================
                         TITRE DU CONTENU
======================================================================

  Type ........... : Film / Serie / Jeu / Application
  Genre .......... : Action, Aventure
  Annee .......... : 2024
  Langue ......... : Francais

======================================================================
                           DESCRIPTION
======================================================================

  Description du contenu extraite du BBCode...

======================================================================
                        INFOS TECHNIQUES
======================================================================

  Format ......... : MKV
  Taille ......... : 4.2 Go

======================================================================
"#;

pub struct LlmClient {
    provider: String,
    api_key: String,
    http: Client,
}

// OpenAI-compatible request/response (Groq, Mistral)
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessageResponse,
}

#[derive(Deserialize)]
struct ChatMessageResponse {
    content: String,
}

// Gemini request/response
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiRequest {
    system_instruction: GeminiContent,
    contents: Vec<GeminiContent>,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContentResponse,
}

#[derive(Deserialize)]
struct GeminiContentResponse {
    parts: Vec<GeminiPartResponse>,
}

#[derive(Deserialize)]
struct GeminiPartResponse {
    text: String,
}

impl LlmClient {
    pub fn new(provider: &str, api_key: &str) -> Self {
        Self {
            provider: provider.to_lowercase(),
            api_key: api_key.to_string(),
            http: Client::new(),
        }
    }

    pub async fn generate_game_description(
        &self,
        title: &str,
        english_summary: Option<&str>,
    ) -> anyhow::Result<String> {
        let user_prompt = Self::build_user_prompt(title, english_summary);
        self.generate_with_prompt(SYSTEM_PROMPT, &user_prompt).await
    }

    pub async fn generate_with_prompt(
        &self,
        system: &str,
        user: &str,
    ) -> anyhow::Result<String> {
        debug!("LLM ({}) generation, prompt len: {}", self.provider, user.len());

        match self.provider.as_str() {
            "groq" => self.call_openai_compatible_custom(
                "https://api.groq.com/openai/v1/chat/completions",
                "llama-3.3-70b-versatile",
                system,
                user,
            ).await,
            "mistral" => self.call_openai_compatible_custom(
                "https://api.mistral.ai/v1/chat/completions",
                "mistral-small-latest",
                system,
                user,
            ).await,
            "gemini" => self.call_gemini_custom(system, user).await,
            other => anyhow::bail!("Provider LLM inconnu : {}", other),
        }
    }

    pub async fn generate_nfo(&self, bbcode: &str) -> anyhow::Result<String> {
        let system = NFO_SYSTEM_PROMPT;
        let user = format!(
            "Voici le BBCode de la presentation. Genere le fichier NFO correspondant :\n\n{}",
            bbcode
        );
        self.generate_with_prompt(system, &user).await
    }

    fn build_user_prompt(title: &str, english_summary: Option<&str>) -> String {
        let context = match english_summary {
            Some(en) => format!(
                "\n\nVoici le resume original en anglais a traduire et reecrire en francais :\n{}",
                en
            ),
            None => String::new(),
        };

        format!(
            "Ecris une description engageante et informative en francais (2-3 paragraphes) \
            pour le jeu video \"{}\".{}",
            title, context
        )
    }

    async fn call_openai_compatible_custom(
        &self,
        url: &str,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
    ) -> anyhow::Result<String> {
        let body = ChatRequest {
            model: model.to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: 0.7,
        };

        let resp = self.http
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            warn!("LLM API error {}: {}", status, text);
            anyhow::bail!("LLM API error {}: {}", status, text);
        }

        let data: ChatResponse = resp.json().await?;
        let text = data.choices
            .into_iter()
            .next()
            .map(|c| c.message.content.trim().to_string())
            .unwrap_or_default();

        debug!("LLM reponse : {} caracteres", text.len());
        Ok(text)
    }

    async fn call_gemini_custom(&self, system_prompt: &str, user_prompt: &str) -> anyhow::Result<String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            self.api_key
        );

        let body = GeminiRequest {
            system_instruction: GeminiContent {
                parts: vec![GeminiPart {
                    text: system_prompt.to_string(),
                }],
            },
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: user_prompt.to_string(),
                }],
            }],
        };

        let resp = self.http
            .post(&url)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            warn!("Gemini API error {}: {}", status, text);
            anyhow::bail!("Gemini API error {}: {}", status, text);
        }

        let data: GeminiResponse = resp.json().await?;
        let text = data.candidates
            .and_then(|c| c.into_iter().next())
            .and_then(|c| c.content.parts.into_iter().next())
            .map(|p| p.text.trim().to_string())
            .unwrap_or_default();

        debug!("Gemini reponse : {} caracteres", text.len());
        Ok(text)
    }
}
