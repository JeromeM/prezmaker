pub fn get_default_html(content_type: &str) -> Option<String> {
    match content_type {
        "jeu" => Some(DEFAULT_JEU_HTML.to_string()),
        _ => None,
    }
}

const DEFAULT_JEU_HTML: &str = r#"{{heading:🎮 {{titre_maj}} 🎮 | font-size:28px;letter-spacing:2px;margin:20px 0}}
{{hr | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent);margin:16px 0}}

{{#if cover_url}}
<table style="width:100%;border-collapse:collapse;margin:16px 0">
<tr>
<td style="width:280px;vertical-align:top;padding:0 16px 0 0">
{{img:{{cover_url}}:264 | border-radius:8px;box-shadow:0 4px 16px rgba(0,0,0,0.4)}}
</td>
<td style="vertical-align:top;padding:8px 0">
<h3 style="color:#c0392b;margin:0 0 12px 0;font-size:16px">Informations</h3>
{{#if date_sortie}}<p style="margin:4px 0"><strong>Date de sortie :</strong> {{date_sortie}}</p>{{/if}}
{{#if developpeurs}}<p style="margin:4px 0"><strong>Développeur(s) :</strong> {{developpeurs}}</p>{{/if}}
{{#if editeurs}}<p style="margin:4px 0"><strong>Éditeur(s) :</strong> {{editeurs}}</p>{{/if}}
{{#if genres}}<p style="margin:4px 0"><strong>Genres :</strong> {{genres}}</p>{{/if}}
{{#if plateformes}}<p style="margin:4px 0"><strong>Plateformes :</strong> {{plateformes}}</p>{{/if}}
{{#if link}}<p style="margin:12px 0"><a href="{{link}}" style="color:#3498db;text-decoration:none">Voir la fiche</a></p>{{/if}}
</td>
</tr>
</table>
{{/if}}

{{#if synopsis}}
{{hr | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent);margin:16px 0}}

{{section:📝 Description | font-size:20px;margin:16px 0}}

<blockquote style="background:#0a1628;border-left:4px solid #c0392b;padding:16px 20px;margin:12px 0;border-radius:0 8px 8px 0;line-height:1.7">
{{synopsis}}
</blockquote>
{{/if}}

{{#if screenshots}}
{{hr | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent);margin:16px 0}}

{{section:📸 Screenshots | font-size:20px;margin:16px 0}}

<table style="width:100%;border-collapse:collapse;margin:12px 0">
<tr>
{{#if screenshot_1}}<td style="padding:4px;text-align:center">{{img:{{screenshot_1}}:400 | border-radius:6px;box-shadow:0 2px 8px rgba(0,0,0,0.3)}}</td>{{/if}}
{{#if screenshot_2}}<td style="padding:4px;text-align:center">{{img:{{screenshot_2}}:400 | border-radius:6px;box-shadow:0 2px 8px rgba(0,0,0,0.3)}}</td>{{/if}}
</tr>
<tr>
{{#if screenshot_3}}<td style="padding:4px;text-align:center">{{img:{{screenshot_3}}:400 | border-radius:6px;box-shadow:0 2px 8px rgba(0,0,0,0.3)}}</td>{{/if}}
{{#if screenshot_4}}<td style="padding:4px;text-align:center">{{img:{{screenshot_4}}:400 | border-radius:6px;box-shadow:0 2px 8px rgba(0,0,0,0.3)}}</td>{{/if}}
</tr>
</table>
{{/if}}

{{#if ratings_count}}
{{hr | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent);margin:16px 0}}

{{ratings_table}}
{{/if}}

{{hr | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent);margin:16px 0}}

{{section:💻 Informations techniques | font-size:20px;margin:16px 0}}

<table style="width:100%;border-collapse:collapse;margin:12px 0;border-radius:8px;overflow:hidden">
<tr style="background:#1a2744">
<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:13px;border-bottom:2px solid #c0392b">Plateforme</th>
<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:13px;border-bottom:2px solid #c0392b">Langue(s)</th>
<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:13px;border-bottom:2px solid #c0392b">Taille</th>
{{#if tech_taille_installee}}<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:13px;border-bottom:2px solid #c0392b">Taille installée</th>{{/if}}
</tr>
<tr style="background:#0f1b32">
<td style="padding:10px 16px;text-align:center;font-size:13px">{{tech_plateforme}}</td>
<td style="padding:10px 16px;text-align:center;font-size:13px">{{tech_langues}}</td>
<td style="padding:10px 16px;text-align:center;font-size:13px">{{tech_taille}}</td>
{{#if tech_taille_installee}}<td style="padding:10px 16px;text-align:center;font-size:13px">{{tech_taille_installee}}</td>{{/if}}
</tr>
</table>

{{#if config_mini}}
{{hr | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent);margin:16px 0}}

{{section:⚙️ Configuration requise | font-size:20px;margin:16px 0}}

<table style="width:100%;border-collapse:collapse;margin:12px 0;border-radius:8px;overflow:hidden">
<tr style="background:#1a2744">
{{#if config_mini}}<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:14px;border-bottom:2px solid #c0392b;width:50%">Configuration minimale</th>{{/if}}
{{#if config_reco}}<th style="padding:10px 16px;text-align:center;color:#c0392b;font-size:14px;border-bottom:2px solid #c0392b;width:50%">Configuration recommandée</th>{{/if}}
</tr>
<tr style="background:#0f1b32">
{{#if config_mini}}<td style="padding:14px 20px;vertical-align:top;font-size:13px;line-height:1.8">{{config_mini}}</td>{{/if}}
{{#if config_reco}}<td style="padding:14px 20px;vertical-align:top;font-size:13px;line-height:1.8">{{config_reco}}</td>{{/if}}
</tr>
</table>
{{/if}}

{{#if installation}}
{{hr | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent);margin:16px 0}}

{{section:📦 Installation | font-size:20px;margin:16px 0}}

<blockquote style="background:#0a1628;border-left:4px solid #3498db;padding:16px 20px;margin:12px 0;border-radius:0 8px 8px 0;line-height:1.7">
{{installation}}
</blockquote>
{{/if}}

{{hr | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent);margin:16px 0}}
{{footer}}"#;
