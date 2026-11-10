//! The attribute block a heading may end with (ex. the `{#id .class}` of
//! `# Title {#id .class}`), as `heading-attributes-spec.md` describes it.
//!
//! CommonMark has no such thing, so the block is only ever looked for here,
//! after the heading's content has been read the way CommonMark reads it. The
//! syntax of what goes between the braces is based on Python-Markdown's
//! `attr_list`:
//!
//! ```text
//! block = "{" [":"] *SP attr *(1*SP attr) *SP "}"
//! attr  = "#" name / "." name / name "=" value / name
//! value = DQUOTE *dq-char DQUOTE / "'" *sq-char "'" / 1*char
//! ```
//!
//! where `char` is anything but whitespace and `{}="'\`, `name` is one or more
//! of them that doesn't start with `#`, `.` or `:`, and a quoted value runs to
//! the next quote of its kind.

use super::ast::*;

/// Where the attribute block that ends a heading's last line of content starts
/// within that line, when it ends with one.
///
/// `line` is the heading's last line of content with its trailing whitespace
/// removed, and `line_start` is where it starts in the file. `children` are the
/// inline nodes the whole of the heading's content was read as, which decide
/// whether the block's `{` is plain text or part of something that starts
/// before it.
pub fn attribute_block_start(line: &str, line_start: usize, children: &[Node]) -> Option<usize> {
  if !line.ends_with('}') {
    return None;
  }
  // a block can't hold a `{` outside a quoted value, and a quoted value can't
  // run past the `}` that ends the line, so at most one `{` reads as the
  // start of a block that runs to the end of the line
  let start = line
    .char_indices()
    .filter(|(_, c)| *c == '{')
    .map(|(index, _)| index)
    .find(|&index| block_len(&line[index..]) == Some(line.len() - index))?;

  // the text is separated from the block by spaces and nothing else, and has
  // to be on the same line as it. A block touching the text belongs to what it
  // touches in the dialects that have them, and a tab is kept where it is
  // written in a heading's text today, so it can't be turned into the space
  let before = line[..start].trim_end_matches(' ');
  if before.len() == start || before.ends_with('\t') || before.trim_matches(['\t', ' ']).is_empty() {
    return None;
  }

  // the `{` has to be text of the heading itself rather than part of a code
  // span, a link, a decoration or html that starts before it
  let position = line_start + start;
  let is_plain_text = children
    .iter()
    .find(|child| child.span().start <= position && position < child.span().end)
    .is_some_and(|child| matches!(child, Node::Text(_)));
  is_plain_text.then_some(start)
}

/// The length of the block at the start of `text`, through its closing `}`,
/// when one is written there.
fn block_len(text: &str) -> Option<usize> {
  let mut len = None;
  read_block(text, |piece| {
    if let BlockPiece::End(end) = piece {
      len = Some(end);
    }
  })?;
  len
}

/// A part of an attribute block, as the formatter writes it out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockPiece {
  /// Text that's written as it is, given as the byte range it covers.
  Text(usize, usize),
  /// A run of spaces outside of a quoted value, given as the byte range it
  /// covers, which is written as one space.
  Spaces(usize, usize),
  /// Where the block ends, which is just past its closing `}`.
  End(usize),
}

/// Reads the block at the start of `text`, handing each of its pieces to
/// `on_piece` in order, or returns `None` when no block is written there.
///
/// The pieces handed over before a `None` are of no use, since they're of
/// something that turned out not to be a block.
pub fn read_block(text: &str, mut on_piece: impl FnMut(BlockPiece)) -> Option<()> {
  let bytes = text.as_bytes();
  if bytes.first() != Some(&b'{') {
    return None;
  }
  let mut position = 1;
  if bytes.get(position) == Some(&b':') {
    position += 1;
  }
  on_piece(BlockPiece::Text(0, position));

  let mut attribute_count = 0;
  loop {
    let spaces_end = position + text[position..].len() - text[position..].trim_start_matches(' ').len();
    if spaces_end > position {
      on_piece(BlockPiece::Spaces(position, spaces_end));
    }
    let had_spaces = spaces_end > position;
    position = spaces_end;

    match bytes.get(position) {
      Some(b'}') => {
        // a block holds at least one attribute
        if attribute_count == 0 {
          return None;
        }
        on_piece(BlockPiece::Text(position, position + 1));
        on_piece(BlockPiece::End(position + 1));
        return Some(());
      }
      None => return None,
      // attributes are separated by spaces
      Some(_) if attribute_count > 0 && !had_spaces => return None,
      Some(_) => {}
    }

    let attribute_end = attribute_len(&text[position..])? + position;
    on_piece(BlockPiece::Text(position, attribute_end));
    position = attribute_end;
    attribute_count += 1;
  }
}

/// The length of the attribute at the start of `text`.
fn attribute_len(text: &str) -> Option<usize> {
  match text.as_bytes().first()? {
    b'#' | b'.' => Some(1 + name_len(&text[1..])?),
    _ => {
      let name = name_len(text)?;
      if text.as_bytes().get(name) != Some(&b'=') {
        return Some(name);
      }
      let value = &text[name + 1..];
      let value_len = match value.as_bytes().first()? {
        quote @ (b'"' | b'\'') => 2 + value[1..].find(*quote as char)?,
        _ => chars_len(value),
      };
      (value_len > 0).then_some(name + 1 + value_len)
    }
  }
}

/// The length of the name at the start of `text`.
fn name_len(text: &str) -> Option<usize> {
  if text.starts_with(['#', '.', ':']) {
    return None;
  }
  let len = chars_len(text);
  (len > 0).then_some(len)
}

/// The length of the run of `char`s at the start of `text`: anything but
/// whitespace and the characters that have a meaning of their own in a block.
fn chars_len(text: &str) -> usize {
  text
    .find([' ', '\t', '\n', '\r', '{', '}', '=', '"', '\'', '\\'])
    .unwrap_or(text.len())
}

#[cfg(test)]
mod test {
  use super::*;

  fn is_block(text: &str) -> bool {
    block_len(text) == Some(text.len())
  }

  #[test]
  fn reads_valid_blocks() {
    for text in [
      "{#id}",
      "{.class}",
      "{key=value}",
      "{key=\"a  {b} 'c'\"}",
      "{key='a \"b\"'}",
      "{checked}",
      "{-}",
      "{: #id .a}",
      "{  #id   .a  }",
      "{#id #id}",
      "{#ID1.class1, #ID2.class2}",
      "{t=\"*a*\"}",
      "{t=\"a\\\"}",
      "{t=\"\"}",
      "{t=\"a\tb\"}",
      "{data-x=1 #a}",
      "{#a#b}",
      "{:#id}",
    ] {
      assert!(is_block(text), "{text}");
    }
  }

  #[test]
  fn rejects_invalid_blocks() {
    for text in [
      "{}",
      "{ }",
      "{:}",
      "{::x}",
      "{#}",
      "{.}",
      "{##id}",
      "{#.a}",
      "{key=}",
      "{a=\"b}",
      "{a=b=c}",
      "{x\\y}",
      "{#id\t.c}",
      "{\t#id}",
      "{#id",
      "{a {b}",
      "{a}b}",
      "{a=\"b\"c}",
      "{=a}",
      "{a=b }c",
      "{: }",
      "{ :#id}",
    ] {
      assert!(!is_block(text), "{text}");
    }
  }

  /// The block the first heading of the markdown ends with, as its text, and
  /// the text of the heading read without it.
  fn heading_attributes(markdown: &str) -> Option<(String, String)> {
    let bump = dprint_core::formatting::bumpalo::Bump::new();
    let file = crate::parser::parse(markdown, &bump).unwrap();
    fn first_heading<'n, 'a>(nodes: &'n [Node<'a>]) -> Option<&'n Heading<'a>> {
      nodes.iter().find_map(|node| match node {
        Node::Heading(heading) => Some(heading),
        node => first_heading(node.children()),
      })
    }
    let heading = first_heading(file.children).unwrap_or_else(|| panic!("no heading in {markdown:?}"));
    let attributes = heading.attributes.as_ref()?;
    assert_eq!(attributes.span.text(markdown), attributes.text);
    let start = attributes.children.first().unwrap().span().start;
    let end = attributes.children.last().unwrap().span().end;
    Some((attributes.text.to_string(), markdown[start..end].to_string()))
  }

  fn attributes_of(markdown: &str, block: &str, text: &str) {
    assert_eq!(
      heading_attributes(markdown),
      Some((block.to_string(), text.to_string())),
      "{markdown:?}"
    );
  }

  #[test]
  fn finds_the_block_a_heading_ends_with() {
    attributes_of("# Title {#id}", "{#id}", "Title");
    attributes_of("# *a*   {#id .c} ##", "{#id .c}", "*a*");
    attributes_of("# Title ## {#id}", "{#id}", "Title ##");
    attributes_of("Alpha\nbeta {.c}\n===", "{.c}", "Alpha\nbeta");
    attributes_of("> Alpha\n> beta {.c}\n> ---", "{.c}", "Alpha\n> beta");
    attributes_of("# Title   {a=\" {b   c=\"d}\"}", "{b   c=\"d}\"}", "Title   {a=\"");
  }

  #[test]
  fn leaves_headings_without_a_block() {
    for markdown in [
      "# Title{#id}",
      "# Title\t{#id}",
      "# Title \t {#id}",
      "#    {#id}",
      "# Title {}",
      "# Title {#id} text",
      "# Title {#id} {bad",
      "# Title `{#id` }",
      "# Title [a {b](u) c}",
      "# *a {*}",
      "# Title \\{#id}",
      "Title\n{#id}\n===",
      "Title {#id\n.c}\n===",
    ] {
      assert_eq!(heading_attributes(markdown), None, "{markdown:?}");
    }
  }

  #[test]
  fn hands_over_the_pieces_of_a_block() {
    let mut pieces = Vec::new();
    read_block("{:  #id   t=\"a  b\" }", |piece| pieces.push(piece)).unwrap();
    assert_eq!(
      pieces,
      vec![
        BlockPiece::Text(0, 2),
        BlockPiece::Spaces(2, 4),
        BlockPiece::Text(4, 7),
        BlockPiece::Spaces(7, 10),
        BlockPiece::Text(10, 18),
        BlockPiece::Spaces(18, 19),
        BlockPiece::Text(19, 20),
        BlockPiece::End(20),
      ]
    );
  }
}
