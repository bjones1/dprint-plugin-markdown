# Superscript

## Goal

This plugin is a formatter: it parses Markdown and writes it back. The
CodeChat Editor renders that Markdown with pulldown-cmark 0.13
([`Options::all()`](https://docs.rs/pulldown-cmark/0.13.6/pulldown_cmark/struct.Options.html)
without smart punctuation), which reads `^text^` as superscript. This
plugin's parser reads `^` as plain text, so formatting can change what the
CodeChat Editor shows. Today, on `main` (`7428123`), with `␣` a space and `⏎`
a line break (rows 1-3 with any `textWrap`, row 4 with `always` or `never`):

| Input           | Formatted today | What changes when rendered                                                             |
| --------------- | --------------- | -------------------------------------------------------------------------------------- |
| `^a *b^ c*`     | `^a _b^ c_`     | the text `*` becomes `_` (the parser sees emphasis, but `^…^` pairs first, so there is none) |
| `x ^␣␣⏎b^`      | `x ^\⏎b^`       | a superscript is created                                                               |
| `^^a .^␣␣⏎b`    | `^^a .^\⏎b`     | a superscript is removed                                                               |
| `^中^⏎字`       | `^中^ 字`       | a space is added between the CJK characters                                            |

This change adds superscript to the existing parser as one more kind of text
decoration. Then the formatter's existing rules for emphasis delimiters, and
its existing read-back check, cover `^` too, and every superscript is written
back with the carets it was written with. There is only one delimiter
character, so there is nothing to normalize and no new configuration option.

Out of scope: subscript `~text~`
([PR #1](https://github.com/bjones1/dprint-plugin-markdown/pull/1)), and
`<sup>` HTML, which is already kept as inline HTML. Existing formatter bugs
that hit `*` and `_` hit `^` the same way and are not fixed here: for
example, turning a setext heading into ATX drops a line break
(`_中_⏎字⏎===` → `# _中_字`, `**\⏎ab**⏎===` → `# ** ab**`).

## Syntax

`^` is read like `_` under the
[CommonMark emphasis rules](https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis):
where `_` would make emphasis or strong emphasis, `^` makes a superscript.
So:

- A `^` run cannot open or close inside a word.
- Two carets on each side make one superscript, the way `__a__` is one
  strong; three make a superscript inside a superscript, like `___a___`.
- A `^` run pairs only with a `^` run, never with `*`, `_` or `~`.
- Escapes, code spans, math, autolinks and inline HTML are read first, as for
  `_`.
- A `^` right after `[` never opens a superscript: it is the marker of a
  footnote reference `[^1]`.

| Input           | Read as                                     |
| --------------- | ------------------------------------------- |
| `a ^b c^ d`     | superscript `b c`                           |
| `x^2^`          | text (inside a word)                        |
| `^^a^^`         | one superscript `a`                         |
| `^^^a^^^`       | superscript `a` inside a superscript        |
| `^bar^^`        | superscript `bar`, then the text `^`        |
| `\^a^`          | text (escaped)                              |
| `` `x^2^` ``    | code span                                   |
| `*^a^*`         | emphasis around superscript `a`             |
| `^a *b^ c*`     | superscript `a *b`, then the text `c*`      |
| `[^1] b^`       | text (the first `^` is a footnote marker)   |

`x^2^` is a superscript in Pandoc and in the unreleased pulldown-cmark
`master`, but not in 0.13, which this follows.

Known differences from pulldown-cmark 0.13, each formatted without a change
in rendering:

- pulldown-cmark keeps one "openers bottom" shared by all `^` and `~` runs.
  This parser keys it by character, run length mod 3 and whether the closer
  can open (`process_emphasis()`), as CommonMark does for `_`. So
  `^a .^^. b^` and `^a ~~b~ c^` have no superscript in pulldown-cmark, and
  one here.
- pulldown-cmark opens a superscript after `[` when the bracket turns out
  not to be a footnote reference: `[^a^ b` (no `]`) and `[^a^](/u)` (a link).
  This parser reads the `^` after `[` as text.

## Data structures

No new types. A superscript is the existing `TextDecoration`
(`src/parser/ast.rs`) with a new kind:

```rust
pub enum TextDecorationKind {
  Emphasis,
  Strong,
  Strikethrough,
  Superscript,
}
```

Its `span` covers the carets and `children` holds what is between them. The
number of carets (1 or 2) is not stored: it is
`children.first().span().start - span.start`. The parser spec JSON writes
`"decoration": "Superscript"`.

## Changes

Parser, `src/parser/inline.rs`:

- `parse()`: `^` goes to `parse_emphasis_delimiter()` with `*`, `_` and `~`.
- `parse_emphasis_delimiter()`: a `^` run right after `[` cannot open.
- `flanking()`: no change. `^` already takes the `_` branch (not within a
  word).
- `wrap_emphasis()`: `(b'^', _) => TextDecorationKind::Superscript`. The
  number of carets used is chosen as for `_`: two when both runs have at
  least two, otherwise one.

Generator, `src/generation/generate.rs`:

- `gen_text_decoration()`: arms for `"^"` and `"^^"`, before the catch-all
  arm that writes `~~`.
- `decoration_delimiter()` → `resolve()`: `Superscript` returns
  `written_delimiter()`, like `Strikethrough` returns `~~`.
- `written_delimiter()`: `^^` when the content starts two bytes after
  `span.start`, otherwise `^`. Reading the carets off the file text is wrong
  for `^^^a^^^`, where the outer superscript has one caret.
- `unescaped_delimiter_runs()`, `run_can_open()`, `run_can_close()`: treat a
  `^` run like a `_` run. A `^` run left as text then counts as pairable, so
  the existing read-back check (`reads_back_decorations()`) runs and catches
  a rewrite that changes which superscripts there are or the text they hold.
- `gen_hard_break()`: a hard break right after `^` is written as it was (two
  spaces or a backslash). A backslash there would let the `^` open a
  superscript (`x ^␣␣⏎b^`). The read-back check alone misses a hard break that
  only moves a superscript without changing its text
  (`^␣␣␣⏎.^.^ !` → `^\⏎.^.^ !`).
- `decoration_keeps_its_delimiters()`: `^` joins the characters in
  `is_delimiter`.

Tests, `tests/commonmark_spec_check.rs`: its HTML renderer maps `Superscript`
to `sup` (the `match` must cover the new kind).

## Tests

Parser, new file `tests/parser_specs/superscript.txt`: one test per row of
the Syntax table, with the AST (`UPDATE=1` writes it; check it by hand).

Formatter, new file `tests/specs/TextDecoration/TextDecoration_Superscript.txt`
(default config):

| Input                                  | Expected output                        |
| -------------------------------------- | -------------------------------------- |
| `a ^b c^ d`                            | unchanged                              |
| `^^a^^ and ^^^a^^^ and ^bar^^`         | unchanged                              |
| `x^2^ and 1^st^ place`                 | unchanged                              |
| `*^a^* and ^*a*^`                      | `_^a^_ and ^_a_^`                      |
| `^a *b^ c*`                            | unchanged                              |
| `x ^␣␣⏎b^`                             | unchanged                              |
| `^^a .^␣␣⏎b`                           | unchanged                              |
| `x ^␣␣⏎[^1] b^` and `^^[^1]^␣␣⏎b`      | unchanged                              |
| `^␣␣␣⏎.^.^ !`                          | `^␣␣⏎.^.^ !`                           |

New file `tests/specs/TextDecoration/TextDecoration_Superscript_TextWrap_Always.txt`
(`textWrap: always`, `lineWidth: 20`):

| Input                     | Expected output            |
| ------------------------- | -------------------------- |
| `^中^⏎字`                 | unchanged                  |
| `a ^b c d e f g h i j^ k` | `a ^b c d e f g h i⏎j^ k`  |

For every formatter test, pulldown-cmark 0.13.6 must render the input and the
output the same way, apart from whitespace, and every existing test must
still pass.
