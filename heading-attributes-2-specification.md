# Heading Attribute Preservation During Markdown Formatting

## Problem and scope

Formatting must preserve the document structure and heading semantics recognized by the selected attribute-aware Markdown parser. For this draft, the provisional dialect is the heading-attributes extension in `pulldown-cmark` 0.13.4; the repository itself does not select an attribute dialect, so instructor confirmation remains open.

The project meeting adds a formatting convention: a valid trailing attribute block must remain intact and on the final physical heading-text line. This is stricter than the parser’s Setext placement rules. The test-first scope includes ATX and Setext levels 1–6, Setext wrapping, and ATX-to-Setext conversion where configured. Paragraph and list behavior must remain unchanged. No production implementation is requested in this draft stage.

CommonMark does not define heading attributes. The semantic oracle is a test-only parser, not the formatter’s parser. The repository AST currently stores heading level, style, and inline children, but no structured attribute metadata (`src/parser/ast.rs`, `Heading`).

## Attribute Block Recognition Rules

These rules summarize the pinned [`pulldown-cmark` 0.13.4 heading-attributes specification](https://github.com/pulldown-cmark/pulldown-cmark/blob/v0.13.4/pulldown-cmark/specs/heading_attrs.txt) and its implementation in `firstpass.rs` (`extract_attribute_block_content_from_header_text`, `parse_attributes`). The upstream specification remains authoritative if this summary conflicts with it.

1. **Position and delimiters.** A block is recognized only as a complete trailing `{…}` suffix of heading text. Trailing spaces or tabs after `}` are permitted. A separator before `{` is recommended for compatibility but is not required (`text{#id}` is recognized). For Setext headings, the parser also accepts a complete block on its own content line immediately before the underline. An ATX heading ends on its physical line; a following line is separate content.
2. **Candidate bounds.** Recognition finds a closing `}` at the last non-trailing-whitespace position, then scans backward for its opening `{`. The candidate cannot contain another `{`, `}`, `<`, `>`, backslash, LF, or CR. Incomplete or malformed candidates are not extracted. `{}` is a complete, valid empty block and assigns no ID, classes, or custom attributes.
3. **Suffix requirement.** A block followed by ordinary heading text is not trailing and is not extracted: `{#id} foo` is heading text. Text after the closing brace that is only permitted trailing whitespace does not invalidate the suffix.
4. **Members.** The candidate’s contents are divided by Rust `split_ascii_whitespace` separators (ASCII space, tab, and form feed; LF and CR already invalidate the candidate; vertical tab and Unicode whitespace are not separators). A token beginning `#` with a nonempty remainder assigns an ID; if repeated, the last ID wins. A token beginning `.` with a nonempty remainder adds a class; class order and duplicates are retained. Other tokens are custom attributes: `name` has no value, `name=` has an empty value, and `name=value` has that value. Custom attributes remain ordered; repeated entries are retained. `#` and `.` alone do not create an ID or class. Markers are not combined within one token: `#id.class` is one ID value, not an ID plus class.
5. **Escaping.** Attribute extraction precedes inline backslash processing in the pinned parser. Thus `\{.foo}` still assigns class `foo`; the backslash remains in the heading text. A backslash inside the candidate block prevents recognition. Do not treat a backslash before the opening brace as making the whole candidate ordinary text.
6. **ATX closing hashes.** The attribute block follows any closing ATX hash sequence: `# H # {#id}` is valid. A candidate before closing hashes is not the final suffix, e.g. `#### non-attribute-block {#id4} ####` does not assign `id4`.
7. **Conservative preservation.** If a formatter cannot establish that syntax is a complete trailing block under these rules, it must preserve the text as ordinary inline content rather than extracting or rewriting it as metadata.

Parser recognition describes semantic placement. The final-line rule below describes formatter output. Those requirements must not be conflated.

## Required formatting behavior

1. Preserve the whole document’s block structure, heading level, inline meaning and structure, ID, ordered classes, and ordered custom attributes.
2. Never insert a physical line break inside a recognized attribute block. Emit the complete block on the same physical line as the final heading-text content line.
3. Treat the block together with its original preceding separator as one trailing formatting unit. Preserve adjacency if the input has no separator (`text{#id}`); do not synthesize a space or newline that changes its source association.
4. The heading content immediately before that separator/block is also indivisible at the boundary: if it is plain text, keep its final whitespace-delimited token with the block; if it is a rich inline node (emphasis, strong, link, code span, inline HTML, image, or another single inline AST node), keep that complete node with the block. Never split the node or its delimiters/destination to satisfy the width. Earlier legal wrapping points remain available.
5. If the final unit cannot fit within `lineWidth`, move earlier content to preceding lines where possible. If the unit itself exceeds the configured width, keep it intact and allow overflow. Do not change paragraph/list wrapping to implement this heading-only convention.
6. ATX headings are parsed from one physical source line and generated without newlines in their children today. Preserve that one-line behavior. If configuration converts one to Setext, apply the Setext requirements to the generated heading content.
7. Preserve existing text and breaks that are not an attribute block. In particular, formatting must not join lines so that ordinary brace-delimited text becomes a valid attribute suffix.

The upstream parser may associate a complete block on its own Setext content line with the heading. The formatter must move/serialize it on the final heading-text line to satisfy this project’s convention. A newline inside the block is invalid and remains ordinary heading content.

## Verified repository behavior and implementation plan

**Verified from the current source:**

- `src/parser/ast.rs:86`: `Heading` has `level`, `style`, and `children`; it has no attribute field.
- `src/parser/block.rs:108`: `parse_atx_heading` handles one physical line and parses its content as inlines. `parse_paragraph` begins at about line 480, gathers paragraph lines, and constructs Setext headings around line 517.
- `src/generation/generate.rs:701`: `gen_heading` selects Setext output when `heading.level < 3` and `configuration.heading_kind == HeadingKind::Setext`, regardless of input `heading.style`. Therefore ATX levels 1 and 2 can become Setext; levels 3–6 remain ATX. This conversion is in scope, not a hypothetical exclusion.
- In that Setext branch, child content is generated before `measure_longest_line_width` determines underline length. The implementation must include an emitted suffix in the content being measured. `gen_atx_heading` around line 761 uses `with_no_new_lines` around line 773.
- `gen_str` around line 1445 feeds text through `TextBuilder`; `add_char` around line 1501 flushes on ASCII spaces, and `space_items` around line 1533 supplies configured wrap signals. The current formatter does not recognize heading attributes as a structured unit.
- Inline content is represented by distinct AST nodes (`src/parser/ast.rs:356`, `Node`; links at line 170). The generator dispatches nodes separately (`src/generation/generate.rs:32`) and generates inline links at about line 2247. Code-span wrapping is configuration-dependent (`gen_code` around line 1144 and `wrap_code_spans` around line 1169). This supports treating a complete trailing rich inline node as the boundary unit instead of assuming every suffix follows one plain-text word.
- Width measurement in `measure_longest_line_width` around line 3392 uses `UnicodeWidthStr::width` (`unicode-width` 0.2.0 is pinned in `Cargo.lock`): displayed columns, not UTF-8 bytes, Unicode scalar count, or UTF-16 code units. The dprint-core printer also receives configured `max_width`; verify end-to-end wrap results with fixtures.
- `tests/specs/headers/Headers_Setext_TextWrap_Always.txt` establishes Setext wrapping conventions; `tests/specs/Text/Text_TextWrap_MaintainAndWrap.txt` expects long ATX headings to remain one line. `tests/spec_test.rs:22` runs applicable fixtures twice.
- Configuration is named `heading.kind`; its Setext option is applied to level-one and level-two generated headings even when their source style was ATX.

**Inference, not yet executed:** with `textWrap: always`, a Setext soft break may become a space if the line fits. Therefore `Heading {.foo\n.bar}\n==` could become `Heading {.foo .bar}` and acquire classes. The original parser should report no classes because the candidate block contains a physical newline. The required outcome is semantic preservation: keep that content from becoming attribute syntax. The exact formatted output and whether this formatter currently joins this particular input are unverified because `cargo` is unavailable in this environment. Do not report a reproduced defect or confirmed root cause until the focused test runs.

Use this test-first sequence when implementation is authorized:

1. Add fixture and parser-oracle regression cases from the matrix. Capture exact output and first confirm whether semantic preservation or the final-line convention fails.
2. If a test fails, fix heading generation narrowly: identify only a complete supported suffix, carry it as an indivisible final unit, preserve ordinary brace text and soft-break semantics, and include emitted text in Setext underline measurement.
3. Add structured parser/AST metadata only if the failing test demonstrates source spans and inline nodes cannot safely preserve the suffix. Do not change paragraphs, lists, global wrapping, or production dependencies without a failing requirement that needs it.
4. Run focused and existing tests, then confirm second-pass stability. If the tests already pass, make no production change.

## Semantic and formatter test strategy

Use `pulldown-cmark` 0.13.4 as a provisional test-only oracle. Compare original and formatted input with exactly the same parser version and options. Run one comparison with `Options::all()` to reproduce the meeting configuration and one with `Options::ENABLE_HEADING_ATTRIBUTES` to isolate the extension. Neither option set is inherently wrong. The pinned API exposes heading metadata as `Tag::Heading { level, id: Option<CowStr>, classes: Vec<CowStr>, attrs: Vec<(CowStr, Option<CowStr>)> }`; the corresponding end is `TagEnd::Heading(HeadingLevel)`.

The semantic assertion must compare the **whole ordered event stream for the document**, not a selected heading or flattened heading text. Canonicalize start/end tags including their metadata and nesting; retain paragraph/list/quote/code/HTML block boundaries and all other block events. Within headings, retain nested emphasis, strong, links and destinations/titles, images, code spans, inline HTML, math, footnote references, and other enabled inline events. Keep text content and event order. For semantic comparison, normalize soft breaks and ordinary whitespace to soft whitespace only where reflow is permitted; preserve hard breaks as a distinct event. Do not normalize away lost words, inline boundaries, changed link/image destinations, block boundaries, or heading metadata. Specifically assert the original and output for row 8 both have empty heading metadata and preserve the brace text, rather than collapsing a newly formed class into equivalent-looking plain text. Parse with identical options on both sides. HTML may supplement but cannot replace this structural comparison.

Formatter fixtures assert exact serialization and the final-line convention. Use `lineWidth: 40`, `textWrap: always` and, for Setext output, `heading.kind: setext`. `\n` in the Input column below denotes a physical newline in the fixture, not two literal characters. Expected shapes are specification targets, not observed output. Every proposed row is **not run** in this environment. For Setext headings, expected underline length follows the generated content measurement (longest rendered line in display columns), rather than being assumed equal to `lineWidth`.

In the matrix, `attrs` lists ordered `(name, optional value)` pairs; `—` means no ID or an empty vector as appropriate. Text values are expected parser text after attribute extraction. Use paragraph/list controls in whole-document event comparison as well as their formatter fixtures.

| # | Fixture input | Configuration | Desired output shape | Expected semantics | Assertions / status |
|---:|---|---|---|---|---|
| 1 | `# Heading text {#my-id}` | default | Same one-line ATX form | H1; `Heading text`; ID `my-id` | Semantic + convention; not run |
| 2 | `# A long ATX heading that exceeds the configured width {#my-id}` | width 40, always | One ATX physical line, allowed over-width | H1; full text; ID `my-id` | Semantic + convention; not run |
| 3 | `# Heading { #section-1 .wide custom_attr=value }` | default | One ATX line, intact suffix | H1; `Heading`; ID `section-1`; class `[wide]`; attr `[(custom_attr,value)]` | Semantic + convention; not run |
| 4 | `A long Setext level one heading with enough words to wrap before its suffix {#my-id}\n==` | width 40, always, Setext | Wrap earlier words; suffix on final text line | H1; full text; ID `my-id` | Semantic + convention; not run |
| 5 | `A long Setext level two heading with enough words to wrap before its suffix {#my-id}\n--` | width 40, always, Setext | As row 4 with `-` underline | H2; full text; ID `my-id` | Semantic + convention; not run |
| 6 | `A long Setext heading with an ID class and custom attribute { #section-1 .wide custom_attr=value }\n==` | width 40, always, Setext | Wrap text; suffix indivisible and final | H1; full text; ID, ordered class and attr preserved | Semantic + convention; not run |
| 7 | `First Setext line\nSecond Setext line\n{#my-id}\n==` | width 40, always, Setext | Serialize suffix on final heading-text line | H1; `First Setext line Second Setext line`; ID `my-id` | Semantic + convention; not run |
| 8 | `Heading {.foo\n.bar}\n==` | width 40, always, Setext | Must not join into a valid suffix; retain soft-break/content distinction needed to avoid promotion | H1; no ID/classes/attrs; brace text stays ordinary | Whole-stream semantic; not run; joining risk inferred, output unverified |
| 9 | `Heading {#my-id .alpha .bravo .charlie .delta .echo .foxtrot}\n==` | width 40, always, Setext | Complete over-width block stays intact on final line | H1; ID `my-id`; six ordered classes | Semantic + convention + overflow; not run |
| 10 | `X { #section-1 .wide custom_attr=value }\n==` | width 40, always, Setext | `X { #section-1 .wide custom_attr=value }` is exactly 40 display columns | H1; ID/class/attr preserved | Semantic + exact boundary; not run |
| 11 | `Earlier words move aside from last X { #section-1 .wide custom_attr=value }\n==` | width 40, always, Setext | Earlier words wrap; final line `X { #section-1 .wide custom_attr=value }` exactly 40 columns | H1; all words and metadata preserved | Semantic + wrap boundary; not run |
| 12 | `A long Setext heading without any attributes that wraps before its final words\n=` | width 40, always, Setext | Ordinary no-attribute wrapping remains as before | H1; all words; no metadata | Semantic + control; not run |
| 13 | `# Heading with {ordinary text} trailing` | default | Retain ordinary brace phrase | H1; full text; no metadata | Semantic + false-positive control; not run |
| 14 | `# Heading with an unclosed {#my-id` | default | Retain incomplete candidate as text | H1; full text; no metadata | Semantic + malformed control; not run |
| 15 | `Heading {#my-id .foo}\n==` | width 40, always, Setext | Do not introduce a newline inside the valid existing block | H1; ID `my-id`, class `[foo]`, same content | Semantic + internal-block integrity; not run |
| 16 | `# Heading {#one #two .a .a}` | default | Preserve suffix | H1; ID `two`; ordered duplicate classes `[a,a]` | Semantic + duplicate rules; not run |
| 17 | `# Heading {flag empty= title=café}` | default | Preserve complete suffix | H1; bare `flag`, empty value for `empty`, nonempty `café` value | Semantic + custom values; not run |
| 18 | `# Heading # {#id}` | default | Attribute follows closing hashes | H1; `Heading`; ID `id` | Semantic + ATX closing hashes; not run |
| 19 | `#### Heading {#id} ####` | default | Keep block before closing hashes as ordinary text | H4; no ID | Semantic + false-positive control; not run |
| 20 | `# \{.foo}` | default | Preserve parser-recognized attribute and backslash text | H1; class `foo`; parser-specific content retained | Semantic + escape rule; not run |
| 21 | `# Heading { #id } foo` | default | Do not extract non-trailing block | H1; complete literal text; no metadata | Semantic + false-positive control; not run |
| 22 | `# Heading {}` and `# Heading {#id1` | default | Preserve empty valid suffix/incomplete ordinary text without invented metadata | H1; empty block has no metadata; incomplete has none | Semantic + syntax boundaries; not run |
| 23 | `# text{#id}` | default | Preserve no-space suffix association | H1; text `text`; ID `id`; no inserted separator/newline | Semantic + adjacency convention; not run |
| 24 | `# **important text** {#id}`; `# [link text](https://example.com) {#id}`; `` # `code span` {#id} `` | width 40, always, Setext conversion | Keep complete preceding inline node and suffix as a unit; do not split delimiters/destination | Original inline event nesting and ID preserved | Semantic + rich-inline convention; not run |
| 25 | `# Some text **important words** [link](https://example.com) {#id}` | width 40, always, Setext | Earlier legal wrap points allowed; complete final link node plus suffix remains intact | Text, emphasis, link destination, and ID unchanged | Semantic + rich-inline convention; not run |
| 26 | `# A long ATX heading with {#id}` | width 40, always, `heading.kind: setext` | ATX H1 converts to wrapped Setext output; block stays intact | Same H1, inline meaning and ID | Semantic + conversion; not run |
| 27 | `# A long ATX heading with {#id .wide custom=value}` | width 40, always, `heading.kind: setext` | Converted Setext output; complete multi-member suffix intact | Same H1, ID/class/attr | Semantic + conversion; not run |
| 28 | `# A long ATX heading with enough words to wrap before {#id}\n\nParagraph control.\n\n- list control` | width 40, always, `heading.kind: setext` | Converted heading wraps; paragraph/list formatting matches baseline | Whole document structure and heading metadata unchanged | Whole-stream semantic + controls; not run |
| 29 | Applicable fixtures above, run through project fixture harness | applicable config | Second format is byte-identical to first | Semantic record remains same after each pass | Stability (`format_twice`); runner behavior verified in source, cases not run |
| 30 | `# Heading\n{#id}` | default | Attribute-like next line remains separate content, not attached to ATX heading | H1 has no ID; following paragraph remains a paragraph | Whole-stream semantic + ATX boundary; not run |
| 31 | `# H {.a<U+000B>.b}` and `# H {.a<U+2002>.b}` | default; replace each `<U+…>` marker with that literal code point | Each is one class whose value includes the embedded code point and `.b`; neither character separates tokens | Semantic + ASCII-whitespace boundary; not run |
| 32 | `XX { #section-1 .wide custom_attr=value }\n==` | width 40, always, Setext | Keep final unit intact at 41 ASCII columns; one-column overflow is allowed | H1; ID/class/attr preserved | Semantic + one-unit overflow; not run |
| 33 | `A×38 界\n==` | width 40, always, Setext; expand `A×38` to 38 ASCII `A`s | Break before `界`, because 38 + separator + 2 display columns exceeds 40 | H1; all text unchanged | Semantic + wide-character width; not run |
| 34 | `A×38 e<U+0301>\n==` | width 40, always, Setext; expand and replace marker with U+0301 | Combined `e` plus combining acute is one display column; line is exactly 40 columns and fits | H1; all text unchanged | Semantic + combining-width boundary; not run |

Rows 10–11 use a 38-character block: `len("{ #section-1 .wide custom_attr=value }") = 38`; with `X` and one preceding space the line is 40 ASCII display columns. Row 32 uses `XX` and the same block, totaling 41 columns. In rows 33–34, `A×38` means 38 literal ASCII `A`s, `<U+0301>` means the literal combining acute code point, and `<U+…>` markers must be expanded before parsing. `界` contributes 2 display columns; `e` plus U+0301 contributes 1 in the pinned `unicode-width` metric. Row 17’s `café` exercises non-ASCII attribute content. These metric assertions describe the pinned width library, not visual rendering by a particular font. Row 9 separately tests a block whose suffix alone exceeds the width.

Row 15 is the converse of row 8: its input contains a parser-valid, single-line block; the assertion is that formatting does not split that block internally. It is distinct from row 8 and is not a duplicate no-attribute wrapping case. Row 31’s `<U+…>` strings and row 33–34’s repetition notation are fixture-author notation and must be expanded before parsing. Stability is the fixture runner’s second-pass property, not a standalone formatter feature.

## Acceptance criteria

1. The supported attribute grammar and pinned parser references are defined, with the dialect marked provisional pending instructor confirmation.
2. Tests distinguish ATX, Setext levels one and two, ATX-to-Setext conversion, and the parser-valid separate-line Setext placement.
3. A valid suffix and its defined trailing inline unit remain intact on the final heading-text line, with overflow permitted when the unit cannot fit.
4. Reflow does not promote ordinary brace text into attribute syntax; heading and whole-document structure, rich inline content, and metadata are preserved.
5. Width behavior uses the formatter’s Unicode display-column metric and covers exact width, one-column overflow, wide/combining characters, and a non-ASCII attribute value.
6. No-attribute, paragraph, and list behavior is unchanged, and formatting is stable on a second pass.

## Risks and unresolved questions

- Confirm `pulldown-cmark` 0.13.4 and its heading-attributes extension as the intended assignment dialect and test oracle.
- Confirm that the meeting’s same-final-line convention applies to all rich inline nodes and to ATX-to-Setext conversion.
- `cargo` is not installed in this environment. No formatter fixture, parser experiment, focused reproduction, or proposed matrix row has been executed; row 8’s join behavior remains a source-based risk, not a reproduced defect.
- The original assignment instructions and November 8 meeting transcript were not available in the inspected workspace. Keep any questions depending on them open.

## Non-goals

- Implementing the production feature in this draft pass.
- Adding `pulldown-cmark` as a production dependency.
- Changing paragraph/list or global wrapping behavior.
- Adding unrelated Markdown extensions.

## References

- [CommonMark ATX headings](https://spec.commonmark.org/current/#atx-headings) and [Setext headings](https://spec.commonmark.org/current/#setext-headings).
- [`pulldown-cmark` 0.13.4 `Options`](https://docs.rs/pulldown-cmark/0.13.4/pulldown_cmark/struct.Options.html), [`Tag`](https://docs.rs/pulldown-cmark/0.13.4/pulldown_cmark/enum.Tag.html), [`Event`](https://docs.rs/pulldown-cmark/0.13.4/pulldown_cmark/enum.Event.html), and [`TagEnd`](https://docs.rs/pulldown-cmark/0.13.4/pulldown_cmark/enum.TagEnd.html).
- [`pulldown-cmark` v0.13.4 heading-attributes specification](https://github.com/pulldown-cmark/pulldown-cmark/blob/v0.13.4/pulldown-cmark/specs/heading_attrs.txt).
- [`unicode-width` 0.2.0 `UnicodeWidthStr`](https://docs.rs/unicode-width/0.2.0/unicode_width/trait.UnicodeWidthStr.html).
