use super::output_format::OutputFormat;
use super::{bbcode, html};

pub fn center(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::center(content),
        OutputFormat::Html => html::center(content, style),
    }
}

pub fn bold(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::bold(content),
        OutputFormat::Html => html::bold(content, style),
    }
}

pub fn italic(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::italic(content),
        OutputFormat::Html => html::italic(content, style),
    }
}

pub fn underline(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::underline(content),
        OutputFormat::Html => html::underline(content, style),
    }
}

pub fn small(fmt: OutputFormat, content: &str) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::small(content),
        OutputFormat::Html => html::small(content, None),
    }
}

pub fn color(fmt: OutputFormat, hex: &str, content: &str) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::color(hex, content),
        OutputFormat::Html => html::color(hex, content, None),
    }
}

pub fn size(fmt: OutputFormat, px: u32, content: &str) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::size(px, content),
        OutputFormat::Html => html::size(px, content, None),
    }
}

pub fn img(fmt: OutputFormat, url: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::img(url),
        OutputFormat::Html => html::img(url, style),
    }
}

pub fn img_width(fmt: OutputFormat, url: &str, width: u32, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::img_width(url, width),
        OutputFormat::Html => html::img_width(url, width, style),
    }
}

pub fn img_dim(fmt: OutputFormat, url: &str, width: u32, height: u32) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::img_dim(url, width, height),
        OutputFormat::Html => html::img_dim(url, width, height, None),
    }
}

pub fn url(fmt: OutputFormat, href: &str, label: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::url(href, label),
        OutputFormat::Html => html::url(href, label, style),
    }
}

pub fn h1(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::h1(content),
        OutputFormat::Html => html::h1(content, style),
    }
}

pub fn h2(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::h2(content),
        OutputFormat::Html => html::h2(content, style),
    }
}

pub fn h3(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::h3(content),
        OutputFormat::Html => html::h3(content, style),
    }
}

pub fn hr(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::hr(),
        OutputFormat::Html => html::hr(style),
    }
}

pub fn quote(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::quote(content),
        OutputFormat::Html => html::quote(content, style),
    }
}

pub fn alert(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::alert(content),
        OutputFormat::Html => html::alert(content, style),
    }
}

pub fn spoiler(fmt: OutputFormat, label: &str, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::spoiler(label, content),
        OutputFormat::Html => html::spoiler(label, content, style),
    }
}

pub fn table(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::table(content),
        OutputFormat::Html => html::table(content, style),
    }
}

pub fn tr(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::tr(content),
        OutputFormat::Html => html::tr(content, style),
    }
}

pub fn td(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::td(content),
        OutputFormat::Html => html::td(content, style),
    }
}

pub fn th(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::th(content),
        OutputFormat::Html => html::th(content, style),
    }
}

pub fn field(fmt: OutputFormat, label: &str, value: &str) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::field(label, value),
        OutputFormat::Html => html::field(label, value),
    }
}

pub fn colored_rating(fmt: OutputFormat, value: f64, max: f64) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::colored_rating(value, max),
        OutputFormat::Html => html::colored_rating(value, max),
    }
}

pub fn heading_title(fmt: OutputFormat, text: &str, color_hex: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::heading_title(text, color_hex),
        OutputFormat::Html => {
            let inner = html::h1(&html::color(color_hex, text, None), None);
            html::center(&inner, style)
        }
    }
}

pub fn section_heading(fmt: OutputFormat, title: &str, color_hex: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::section_heading(title, color_hex),
        OutputFormat::Html => {
            let inner = html::h2(&html::color(color_hex, title, None), None);
            html::center(&inner, style)
        }
    }
}

pub fn sub_heading(fmt: OutputFormat, title: &str, color_hex: &str, style: Option<&str>) -> String {
    section_heading(fmt, title, color_hex, style)
}

pub fn inline_heading(fmt: OutputFormat, text: &str, color_hex: &str) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::inline_heading(text, color_hex),
        OutputFormat::Html => html::inline_heading(text, color_hex),
    }
}

pub fn footer(fmt: OutputFormat, pseudo: &str) -> String {
    match fmt {
        OutputFormat::Bbcode => bbcode::footer(pseudo),
        OutputFormat::Html => html::footer(pseudo),
    }
}

// --- Opening/closing tags for block mode ---

pub fn open_center(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => "[center]".to_string(),
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<div style=\"text-align:center;{}\">", s),
            _ => "<div style=\"text-align:center\">".to_string(),
        },
    }
}

pub fn close_center(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Bbcode => "[/center]".to_string(),
        OutputFormat::Html => "</div>".to_string(),
    }
}

pub fn open_quote(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => "[quote]".to_string(),
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<blockquote style=\"{}\">", s),
            _ => "<blockquote>".to_string(),
        },
    }
}

pub fn close_quote(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Bbcode => "[/quote]".to_string(),
        OutputFormat::Html => "</blockquote>".to_string(),
    }
}

pub fn open_bold(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => "[b]".to_string(),
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<strong style=\"{}\">", s),
            _ => "<strong>".to_string(),
        },
    }
}

pub fn close_bold(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Bbcode => "[/b]".to_string(),
        OutputFormat::Html => "</strong>".to_string(),
    }
}

pub fn open_italic(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => "[i]".to_string(),
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<em style=\"{}\">", s),
            _ => "<em>".to_string(),
        },
    }
}

pub fn close_italic(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Bbcode => "[/i]".to_string(),
        OutputFormat::Html => "</em>".to_string(),
    }
}

pub fn open_underline(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => "[u]".to_string(),
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => {
                format!("<span style=\"text-decoration:underline;{}\">", s)
            }
            _ => "<span style=\"text-decoration:underline\">".to_string(),
        },
    }
}

pub fn close_underline(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Bbcode => "[/u]".to_string(),
        OutputFormat::Html => "</span>".to_string(),
    }
}

pub fn open_table(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => "[table]\n".to_string(),
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => {
                format!("<table style=\"width:100%;border-collapse:collapse;{}\">\n", s)
            }
            _ => "<table style=\"width:100%;border-collapse:collapse\">\n".to_string(),
        },
    }
}

pub fn close_table(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Bbcode => "[/table]".to_string(),
        OutputFormat::Html => "</table>".to_string(),
    }
}

pub fn open_tr(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => "[tr]\n".to_string(),
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<tr style=\"{}\">\n", s),
            _ => "<tr>\n".to_string(),
        },
    }
}

pub fn close_tr(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Bbcode => "[/tr]\n".to_string(),
        OutputFormat::Html => "</tr>\n".to_string(),
    }
}

pub fn open_spoiler(fmt: OutputFormat, label: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Bbcode => {
            if label.is_empty() {
                "[spoiler]".to_string()
            } else {
                format!("[spoiler={}]", label)
            }
        }
        OutputFormat::Html => {
            let style_attr = match style {
                Some(s) if !s.is_empty() => format!(" style=\"{}\"", s),
                _ => String::new(),
            };
            if label.is_empty() {
                format!("<details{}>", style_attr)
            } else {
                format!("<details{}><summary>{}</summary>", style_attr, label)
            }
        }
    }
}

pub fn close_spoiler(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Bbcode => "[/spoiler]".to_string(),
        OutputFormat::Html => "</details>".to_string(),
    }
}

// --- HTML-exclusive tags (BBCode fallback) ---

pub fn open_details(fmt: OutputFormat, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<details style=\"{}\">", s),
            _ => "<details>".to_string(),
        },
        OutputFormat::Bbcode => "[spoiler]".to_string(),
    }
}

pub fn close_details(fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Html => "</details>".to_string(),
        OutputFormat::Bbcode => "[/spoiler]".to_string(),
    }
}

pub fn summary(fmt: OutputFormat, text: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<summary style=\"{}\">{}</summary>", s, text),
            _ => format!("<summary>{}</summary>", text),
        },
        OutputFormat::Bbcode => String::new(), // no-op in BBCode
    }
}

pub fn p(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<p style=\"{}\">{}</p>", s, content),
            _ => format!("<p>{}</p>", content),
        },
        OutputFormat::Bbcode => format!("{}\n", content),
    }
}

pub fn pre(fmt: OutputFormat, content: &str, style: Option<&str>) -> String {
    match fmt {
        OutputFormat::Html => match style {
            Some(s) if !s.is_empty() => format!("<pre style=\"{}\">{}</pre>", s, content),
            _ => format!("<pre>{}</pre>", content),
        },
        OutputFormat::Bbcode => format!("[code]{}[/code]", content),
    }
}
