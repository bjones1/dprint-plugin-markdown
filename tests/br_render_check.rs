//! Formats every spec case whose input holds a `<br>` with `lineBreakAfterBr`
//! off and on, renders both outputs with pulldown-cmark, and checks the two
//! render to the same html, so that the line break the option writes after a
//! `<br>` never changes the page.
//!
//! Whitespace after a forced line break isn't rendered, and other runs of
//! whitespace render as one space, so the html is compared with that
//! whitespace removed and collapsed, except within the elements that show it,
//! such as `<pre>`.
//!
//! pulldown-cmark has no GFM `www.` autolinks and doesn't let a table interrupt
//! a paragraph, so the exceptions for those are checked by the spec files only.

use std::path::Path;

use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::ConfigKeyValue;
use dprint_core::configuration::GlobalConfiguration;
use dprint_plugin_markdown::configuration::resolve_config;
use dprint_plugin_markdown::format_text;
use pulldown_cmark::html;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;

/// The elements whose content shows its whitespace, or breaks on a line break.
const PREFORMATTED: [&str; 7] = ["pre", "listing", "xmp", "plaintext", "textarea", "script", "style"];

fn render(markdown: &str) -> String {
  let mut options = Options::all();
  options.remove(Options::ENABLE_SMART_PUNCTUATION);
  let mut out = String::new();
  html::push_html(&mut out, Parser::new_ext(markdown, options));
  out
}

/// The name of the preformatted element whose start tag begins the text.
fn preformatted_start(text: &str) -> Option<&'static str> {
  let name = text.strip_prefix('<')?;
  PREFORMATTED.into_iter().find(|tag| {
    name.get(..tag.len()).is_some_and(|start| start.eq_ignore_ascii_case(tag))
      && !name[tag.len()..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '-')
  })
}

/// The html with runs of whitespace collapsed to one space and the whitespace
/// after each `<br>` tag removed, outside of the preformatted elements.
fn normalize(html: &str) -> String {
  let mut out = String::new();
  let mut rest = html;
  let mut after_br = false;
  while let Some(c) = rest.chars().next() {
    if let Some(tag) = preformatted_start(rest) {
      // the element is kept as it is, up to the end of its end tag
      let close = format!("</{tag}");
      let end = rest
        .to_ascii_lowercase()
        .find(&close)
        .and_then(|start| rest[start..].find('>').map(|end| start + end + 1))
        .unwrap_or(rest.len());
      out.push_str(&rest[..end]);
      rest = &rest[end..];
      after_br = false;
    } else if c.is_whitespace() {
      let end = rest.find(|c: char| !c.is_whitespace()).unwrap_or(rest.len());
      if !after_br {
        out.push(' ');
      }
      rest = &rest[end..];
    } else if c == '<' {
      let end = rest.find('>').map_or(rest.len(), |end| end + 1);
      after_br = rest.get(..3).is_some_and(|start| start.eq_ignore_ascii_case("<br"));
      out.push_str(&rest[..end]);
      rest = &rest[end..];
    } else {
      out.push(c);
      rest = &rest[c.len_utf8()..];
      after_br = false;
    }
  }
  out.trim().to_string()
}

/// The configuration a spec file's `~~ key: value, … ~~` header names.
fn config_of(header: &str) -> ConfigKeyMap {
  let mut map = ConfigKeyMap::new();
  for pair in header.trim_matches(|c| c == '~' || c == ' ').split(", ") {
    let (key, value) = pair.split_once(": ").unwrap();
    let value = match value {
      "true" => ConfigKeyValue::from_bool(true),
      "false" => ConfigKeyValue::from_bool(false),
      _ => value.parse().map(ConfigKeyValue::from_i32).unwrap_or_else(|_| ConfigKeyValue::from_str(value)),
    };
    map.insert(key.to_string(), value);
  }
  map
}

fn format_with(input: &str, mut config: ConfigKeyMap, line_break_after_br: bool) -> String {
  config.insert("lineBreakAfterBr".to_string(), ConfigKeyValue::from_bool(line_break_after_br));
  let config = resolve_config(config, &GlobalConfiguration::default()).config;
  format_text(input, &config, |_, _, _| Ok(None)).unwrap().unwrap_or_else(|| input.to_string())
}

/// Each spec file under the directory, at any depth.
fn spec_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
  for entry in std::fs::read_dir(dir).unwrap() {
    let path = entry.unwrap().path();
    if path.is_dir() {
      spec_files(&path, files);
    } else if path.extension().is_some_and(|extension| extension == "txt") {
      files.push(path);
    }
  }
}

#[test]
fn br_line_breaks_keep_the_rendered_html() {
  let mut files = Vec::new();
  spec_files(Path::new("tests/specs"), &mut files);
  let mut checked = 0;
  for path in files {
    let text = std::fs::read_to_string(&path).unwrap();
    let (config, cases) = match text.split_once('\n') {
      Some((header, cases)) if header.starts_with("~~") => (config_of(header), cases),
      _ => (ConfigKeyMap::new(), text.as_str()),
    };
    for case in cases.split("\n!! ").map(|case| case.trim_start_matches("!! ")) {
      let Some((title, body)) = case.split_once(" !!\n") else {
        continue;
      };
      let Some((input, _)) = body.split_once("\n[expect]\n") else {
        continue;
      };
      if !input.to_ascii_lowercase().contains("<br") {
        continue;
      }
      let off = format_with(input, config.clone(), false);
      let on = format_with(input, config.clone(), true);
      assert_eq!(
        normalize(&render(&off)),
        normalize(&render(&on)),
        "{}, {title}: lineBreakAfterBr changes the rendered html\n--- with it off\n{off}\n--- with it on\n{on}",
        path.display(),
      );
      checked += 1;
    }
  }
  assert!(checked > 240, "only {checked} cases were checked");
}

#[test]
fn normalize_keeps_preformatted_whitespace() {
  assert_eq!(normalize("<p>a<br>\nb  c</p>"), "<p>a<br>b c</p>");
  assert_eq!(normalize("<p>x <pre>a<br>\nb</pre></p>"), "<p>x <pre>a<br>\nb</pre></p>");
}
