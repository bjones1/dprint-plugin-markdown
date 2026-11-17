# Superscript

## Goal

This plugin is a formatter: it parses Markdown and writes it back. The
CodeChat Editor renders that Markdown with pulldown-cmark 0.13
([`Options::all()`](https://docs.rs/pulldown-cmark/0.13.6/pulldown_cmark/struct.Options.html)
without smart punctuation), which reads `^text^` as superscript. This
plugin's parser reads `^` as plain text, so formatting can change what the
CodeChat Editor shows. Today, on upstream `main` (`7428123`), with `␣` a space and `⏎`
a line break (rows 1-3 with any `textWrap`, row 4 with `always` or `never`):

| Input           | Formatted today | Effect                                                                                 |
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
([PR #3](https://github.com/bjones1/dprint-plugin-markdown/pull/3)), and
`<sup>` HTML, which is already kept as inline HTML.

Existing formatter bugs that hit `_` today hit `^` too once it is a
decoration, so these `^` inputs, written back safely today, change rendering
after this change. They are not fixed here, since the fix belongs to the code
shared with `_`:

- With `textWrap: always` or `never`, turning a setext heading into ATX
  drops a line break between CJK characters, so the closing `^` lands inside
  a word and the superscript is lost: `^中^⏎字⏎===` → `# ^中^字` (today
  `_中_⏎字⏎===` → `# _中_字` with any `textWrap`).
- The space before a footnote reference that has a definition is dropped:
  `x ^a^ [^1] y⏎⏎[^1]: n` → `x ^a^[^1] y` (today `_a_` the same way).
- With `textWrap: always` or `never`, joining lines leaves a `*` that starts
  a line inside a superscript unescaped: `^a␣␣⏎*⏎b^` → `^a\⏎* b^`, a list.
- With `hardBreakKind: doubleSpace`, `^a\⏎\⏎b^` → `^a␣␣⏎␣␣⏎b^`, two
  paragraphs.

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
  footnote reference `[^1]`. An escaped `\[` is text, so a `^` after it can
  open.

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
| `\[^1] b^`      | text `[`, then superscript `1] b`           |

`x^2^` is a superscript in Pandoc and in the unreleased pulldown-cmark
`master`, but not in 0.13, which this follows.

Known differences from pulldown-cmark 0.13. All inputs shown except the last
one are formatted without a change in rendering:

- pulldown-cmark keeps one "openers bottom" shared by all `^` and `~` runs.
  This parser keys it by character, run length mod 3 and whether the closer
  can open (`process_emphasis()`), as CommonMark does for `_`. So
  `^a .^^. b^` and `^a ~~b~ c^` have no superscript in pulldown-cmark, and
  one here.
- pulldown-cmark opens a superscript after `[` when the bracket turns out
  not to be a footnote reference: `[^a^ b` (no `]`) and `[^a^](/u)` (a link).
  This parser reads the `^` after `[` as text. So the first row of the Goal
  table still happens after `[`: in `[^a *b^ c*` this parser reads emphasis
  and writes `[^a _b^ c_`, as it does today.

## Data structures

No new AST types or fields. Add one variant to the existing
`TextDecorationKind` enum (`src/parser/ast.rs`), which is used by the `kind`
field of a `TextDecoration`:

```rust
pub enum TextDecorationKind {
  Emphasis,
  Strong,
  Strikethrough,
  Superscript, // added
}
```

A superscript is a `TextDecoration` of that kind, so it goes through the same
parser and generator code as the other three. `gen_text_decoration()` in
`src/generation/generate.rs`, which already writes their delimiters, gets
arms for `^` and `^^` (see Changes).

Its `span` covers the carets and `children` holds what is between them. The
number of carets (1 or 2) is not stored: it is
`children.first().span().start - span.start`. The parser spec JSON writes
`"decoration": "Superscript"`.

## Changes

Parser, `src/parser/inline.rs`:

- `parse()`: `^` goes to `parse_emphasis_delimiter()` with `*`, `_` and `~`.
- `parse_emphasis_delimiter()`: a `^` run cannot open right after a `[`
  that is on the delimiter stack (an escaped `\[` is not).
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
  spaces or a backslash). The read-back check already keeps `x ^␣␣⏎b^`,
  where a backslash would let the `^` open a superscript. This rule is for a
  hard break that only moves a superscript without changing its text, which
  the check misses (`^␣␣␣⏎.^.^ !` → `^\⏎.^.^ !`).
- `decoration_keeps_its_delimiters()`: false for a superscript. An `_`
  written against a word becomes `*`, but `^` has nothing to become. So with
  `wrapUnspacedScripts`, `^中^⏎字` keeps a line break or a space between the
  closing `^` and `字`.

Tests, `tests/commonmark_spec_check.rs`: its HTML renderer maps `Superscript`
to `sup` (the `match` must cover the new kind).

## Tests

Unless shown otherwise, "unchanged" permits the formatter's normal final
newline. Trailing spaces representing hard breaks are significant.

Parser, new file `tests/parser_specs/superscript.txt`: one test per row of
the Syntax table, with the AST (`UPDATE=1` writes it; check it by hand).

Formatter, new file `tests/specs/TextDecoration/TextDecoration_Superscript.txt`
(default config):

| Input                                  | Expected output                        |
| -------------------------------------- | -------------------------------------- |
| `a ^b c^ d`                            | unchanged                              |
| `^^a^^ and ^^^a^^^ and ^bar^^`         | unchanged                              |
| `^^a^^`                                | unchanged                              |
| `x^2^ and 1^st^ place`                 | unchanged                              |
| `*^a^* and ^*a*^`                      | `_^a^_ and ^_a_^`                      |
| `^a *b^ c*`                            | unchanged                              |
| `x ^␣␣⏎b^`                             | unchanged                              |
| `x ^␣\⏎b^`                             | unchanged                              |
| `^^a .^␣␣⏎b`                           | unchanged                              |
| `x ^␣␣⏎[^1] b^` and `^^[^1]^␣␣⏎b`      | unchanged                              |
| `^␣␣␣⏎.^.^ !`                          | `^␣␣⏎.^.^ !`                           |
| `a \[^b *c^ d*`                        | unchanged                              |
| `^中^⏎字⏎===`                          | `# ^中^␣字`                            |

New file `tests/specs/TextDecoration/TextDecoration_Superscript_TextWrap_Always.txt`
(`textWrap: always`, `lineWidth: 20`):

| Input                     | Expected output            |
| ------------------------- | -------------------------- |
| `^中^⏎字`                 | unchanged                  |
| `a ^b c d e f g h i j^ k` | `a ^b c d e f g h i⏎j^ k`  |

For every formatter test, pulldown-cmark 0.13.6 must render the input and the
output the same way, apart from whitespace, and every existing test must
still pass. `tools/superscript-render-check` (run by hand with `cargo run`;
CI does not run it) checks the first part for both formatter test files.
Goal row 4 only changes whitespace, so that check can't see it; the expected
output in the `TextWrap_Always` file covers it.
