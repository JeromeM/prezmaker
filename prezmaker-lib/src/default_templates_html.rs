pub fn get_default_html(content_type: &str) -> Option<String> {
    match content_type {
        "jeu" => Some(default_jeu_html()),
        _ => None,
    }
}

const SEPARATOR_URL: &str = "https://raw.githubusercontent.com/JeromeM/prezmaker/main/assets/separator-red.png";

fn sep() -> String {
    format!(r#"<p style="text-align:center"><img src="{}" style="max-width:100%;width:600px"></p>"#, SEPARATOR_URL)
}

fn default_jeu_html() -> String {
    let s = sep();
    format!(r#"{{{{heading:🎮 {{{{titre_maj}}}} 🎮 | font-size:28px;letter-spacing:2px;margin:20px 0}}}}
{s}

{{{{#if cover_url}}}}
<table style="width:100%;margin:16px 0">
<tr>
<td style="width:280px;vertical-align:top;padding:0 16px 0 0">
{{{{img:{{{{cover_url}}}}:264 | border-radius:8px}}}}
</td>
<td style="vertical-align:top;padding:8px 0">
<h3 style="color:#c0392b;margin:0 0 12px 0;font-size:16px">Informations</h3>
{{{{#if date_sortie}}}}<p style="margin:4px 0"><strong>Date de sortie :</strong> {{{{date_sortie}}}}</p>{{{{/if}}}}
{{{{#if developpeurs}}}}<p style="margin:4px 0"><strong>Développeur(s) :</strong> {{{{developpeurs}}}}</p>{{{{/if}}}}
{{{{#if editeurs}}}}<p style="margin:4px 0"><strong>Éditeur(s) :</strong> {{{{editeurs}}}}</p>{{{{/if}}}}
{{{{#if genres}}}}<p style="margin:4px 0"><strong>Genres :</strong> {{{{genres}}}}</p>{{{{/if}}}}
{{{{#if plateformes}}}}<p style="margin:4px 0"><strong>Plateformes :</strong> {{{{plateformes}}}}</p>{{{{/if}}}}
</td>
</tr>
</table>
{{{{/if}}}}

{s}

{{{{#if synopsis}}}}
{{{{section:📝 Description | font-size:20px;margin:16px 0}}}}

<blockquote style="padding:16px 20px;margin:12px 0;border-radius:8px;line-height:1.7">
{{{{synopsis}}}}
</blockquote>

{s}
{{{{/if}}}}

{{{{#if screenshots}}}}
{{{{section:📸 Screenshots | font-size:20px;margin:16px 0}}}}

<table style="width:100%;margin:12px 0">
<tr>
{{{{#if screenshot_1}}}}<td style="padding:4px;text-align:center">{{{{img:{{{{screenshot_1}}}}:400 | border-radius:6px}}}}</td>{{{{/if}}}}
{{{{#if screenshot_2}}}}<td style="padding:4px;text-align:center">{{{{img:{{{{screenshot_2}}}}:400 | border-radius:6px}}}}</td>{{{{/if}}}}
</tr>
<tr>
{{{{#if screenshot_3}}}}<td style="padding:4px;text-align:center">{{{{img:{{{{screenshot_3}}}}:400 | border-radius:6px}}}}</td>{{{{/if}}}}
{{{{#if screenshot_4}}}}<td style="padding:4px;text-align:center">{{{{img:{{{{screenshot_4}}}}:400 | border-radius:6px}}}}</td>{{{{/if}}}}
</tr>
</table>

{s}
{{{{/if}}}}

{{{{#if ratings_count}}}}
{{{{section:⭐ Notes | font-size:20px;margin:16px 0}}}}

<table style="width:100%;margin:12px 0;border-radius:8px">
<tr>
{{{{#if rating_1_source}}}}<th style="padding:10px 20px;text-align:center;color:#c0392b;font-size:14px">{{{{rating_1_source}}}}</th>{{{{/if}}}}
{{{{#if rating_2_source}}}}<th style="padding:10px 20px;text-align:center;color:#c0392b;font-size:14px">{{{{rating_2_source}}}}</th>{{{{/if}}}}
</tr>
<tr>
{{{{#if rating_1_display}}}}<td style="padding:12px 20px;text-align:center;font-size:16px">{{{{rating_1_display}}}}</td>{{{{/if}}}}
{{{{#if rating_2_display}}}}<td style="padding:12px 20px;text-align:center;font-size:16px">{{{{rating_2_display}}}}</td>{{{{/if}}}}
</tr>
</table>

{s}
{{{{/if}}}}

{{{{section:💻 Informations techniques | font-size:20px;margin:16px 0}}}}

<table style="width:100%;margin:12px 0;border-radius:8px">
<tr>
<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:13px">Plateforme</th>
<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:13px">Langue(s)</th>
<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:13px">Taille</th>
{{{{#if tech_taille_installee}}}}<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:13px">Taille installée</th>{{{{/if}}}}
</tr>
<tr>
<td style="padding:10px 16px;text-align:center;font-size:13px">{{{{tech_plateforme}}}}</td>
<td style="padding:10px 16px;text-align:center;font-size:13px">{{{{tech_langues}}}}</td>
<td style="padding:10px 16px;text-align:center;font-size:13px">{{{{tech_taille}}}}</td>
{{{{#if tech_taille_installee}}}}<td style="padding:10px 16px;text-align:center;font-size:13px">{{{{tech_taille_installee}}}}</td>{{{{/if}}}}
</tr>
</table>

{{{{#if config_mini}}}}
{s}

{{{{section:⚙️ Configuration requise | font-size:20px;margin:16px 0}}}}

<table style="width:100%;margin:12px 0;border-radius:8px">
<tr>
{{{{#if config_mini}}}}<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:14px;width:50%">Configuration minimale</th>{{{{/if}}}}
{{{{#if config_reco}}}}<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:14px;width:50%">Configuration recommandée</th>{{{{/if}}}}
</tr>
<tr>
{{{{#if config_mini}}}}<td style="padding:14px 20px;vertical-align:top;font-size:13px;line-height:1.8">{{{{config_mini}}}}</td>{{{{/if}}}}
{{{{#if config_reco}}}}<td style="padding:14px 20px;vertical-align:top;font-size:13px;line-height:1.8">{{{{config_reco}}}}</td>{{{{/if}}}}
</tr>
</table>
{{{{/if}}}}

{{{{#if installation}}}}
{s}

{{{{section:📦 Installation | font-size:20px;margin:16px 0}}}}

<blockquote style="padding:16px 20px;margin:12px 0;border-radius:8px;line-height:1.7">
{{{{installation}}}}
</blockquote>
{{{{/if}}}}

{s}
{{{{footer}}}}"#)
}
