use dprint_plugin_markdown::{configuration::ConfigurationBuilder, format_text};
use pulldown_cmark::{html, Options, Parser};
fn render(text: &str) -> String {
  let mut options = Options::all();
  options.remove(Options::ENABLE_SMART_PUNCTUATION);
  let mut output = String::new();
  html::push_html(&mut output, Parser::new_ext(text, options));
  output.split_whitespace().collect::<Vec<_>>().join(" ")
}
fn main() {
  let mut count = 0;
  for (path, wrap) in [
    ("../../tests/specs/TextDecoration/TextDecoration_Superscript.txt", false),
    (
      "../../tests/specs/TextDecoration/TextDecoration_Superscript_TextWrap_Always.txt",
      true,
    ),
  ] {
    let source = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
      .unwrap()
      .replace("\r\n", "\n");
    for case in source.split("!! ").skip(1) {
      let (name, rest) = case.split_once(" !!\n").unwrap();
      let (input, _) = rest.split_once("\n\n[expect]\n").unwrap();
      let mut builder = ConfigurationBuilder::new();
      if wrap {
        builder
          .text_wrap(dprint_plugin_markdown::configuration::TextWrap::Always)
          .line_width(20);
      }
      let output = format_text(input, &builder.build(), |_, _, _| Ok(None))
        .unwrap()
        .unwrap_or_else(|| input.to_owned());
      assert_eq!(
        render(input),
        render(&output),
        "rendering changed: {name}\n{input:?}\n{output:?}"
      );
      count += 1;
    }
  }
  println!("{count} superscript cases preserve pulldown-cmark 0.13.6 HTML (whitespace normalized)");
}
