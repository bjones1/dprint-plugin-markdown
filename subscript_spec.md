# Markdown subscript support (draft)

This document specifies proposed behavior, not an implemented feature.

## Current behavior

`tests/parser_specs/emphasis.txt` classifies `~hello~` as strikethrough.
`tests/specs/TextDecoration/TextDecoration_All.txt` expects `~a~` to become
`~~a~~`. In `src/parser/inline.rs`, `flanking` disallows intraword single
tildes, and `wrap_emphasis` maps both tilde lengths to strikethrough.
This proposal intentionally changes single-tilde strikethrough semantics.

## Proposed syntax

Follow [Pandoc's subscript convention](https://pandoc.org/MANUAL.html#superscripts-and-subscripts):
`~content~` denotes subscript, including within words (`H~2~O`). Content
must be nonempty. Opening and closing delimiters must each be exactly one
unescaped tilde; surrounding letters or punctuation do not prevent recognition.

Content may contain inline formatting but no unescaped whitespace. Permit
backslash-escaped ASCII spaces (`\ `); reject tabs and physical newlines,
even when preceded by a backslash. Whitespace restrictions apply throughout
the candidate's source content. Escaped spaces remain escaped when formatted.

An escaped tilde (`\~`) is literal, not a delimiter; existing backslash-pair
handling determines whether a tilde is escaped. Tildes inside code spans
are literal and cannot open or close subscript. Empty, invalid, or unmatched
single-tilde candidates remain ordinary Markdown text, never strikethrough.
Rejected candidates must remain non-subscript after formatting and reparsing.
Preserve whitespace that prevents recognition whenever normalizing it would
create valid subscript; idempotence (`format_twice`) alone does not prove
semantic preservation.

Double-tilde runs take precedence and retain existing strikethrough matching
and formatting. Never split `~~` into subscript delimiters or pair a single
tilde with part of a longer run. As a deliberate compatibility restriction,
inline runs of three or more tildes remain literal; full Pandoc delimiter
compatibility is not claimed. Fenced code remains unaffected.

## Strategy and acceptance

Extend `TextDecorationKind` in `src/parser/ast.rs` with subscript. Adapt the
inline delimiter parser for these boundaries while retaining source spans
and children. Update `gen_text_decoration` and `decoration_delimiter` in
`src/generation/generate.rs` to emit single tildes for subscript and double
tildes for strikethrough. Preserve content and escapes without wrapping
inside subscript.

Representative expected results (syntax retained):

- `H~2~O` and `~a~`: subscript.
- `P~a\ cat~`: subscript containing an escaped space.
- `~~deleted~~`: strikethrough.
- `~a b~`, `~unfinished`, and `\~a\~`: no subscript.
- `` `~a~` ``: literal code.
- `~a\  b~` → `~a\  b~`: preserve both spaces and verify non-subscript interpretation after formatting and reparsing.

Future parser and formatter cases must cover these rules and require
`format(format(input)) == format(input)`, following the existing formatter
harness's `format_twice` convention. These proposed cases have not been run.
