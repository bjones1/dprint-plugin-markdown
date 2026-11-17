# Heading attributes

A heading may end with an attribute block, as in `# Title {#id .class key="value"}`. The formatter keeps the block together and attached to the heading while it formats the heading text as it already does. CommonMark does not define attribute blocks. `pulldown-cmark` can read them with [`Options::ENABLE_HEADING_ATTRIBUTES`](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/struct.Options.html#associatedconstant.ENABLE_HEADING_ATTRIBUTES), but this formatter has its own parser and doesn't use `pulldown-cmark`, so this specification defines block recognition and formatting itself, based on [Python-Markdown's `attr_list`](https://python-markdown.github.io/extensions/attr_list/), using the grammar below. This adds no configuration options and does not check what the attributes mean (for example, duplicate ids are kept).

Everything not described here is unchanged: a heading without a recognized block, including every case below that fails recognition, is formatted exactly as it is today, with its braces treated as ordinary text.

In this specification, whitespace means spaces (U+0020) and tabs (U+0009), and line endings (`\r\n`) are normalized to `\n` before recognition.

## Recognition

A heading has an attribute block when all of these hold:

1. The raw heading content (after stripping the ATX opening marker and any ATX closing sequence, e.g., `# Title {#id} ##` -> `Title {#id}`) ends with a block, ignoring trailing whitespace. Specifically, the block starts at the `{` from which the entire remainder of the content (up to trailing whitespace) matches the block grammar below, tracking quoted values so that `{` and `}` inside quotes are treated as literal characters rather than block delimiters. Determining whether the raw content ends with a valid block is performed **before** inline Markdown parsing of the heading text, so that Markdown delimiters inside the block (such as `**neat**` in `# foo { bar="test **neat** stuff" }`) are not parsed as inline elements and cannot pair with delimiters in the heading text. Only a single trailing block is recognized; any earlier `{...}` followed by other text is treated as ordinary heading text.
2. That `{` is not inside an inline element that starts before it. In this specification, an inline element is a construct with delimiters of its own: a code span, emphasis, strong emphasis, strikethrough, link, image, autolink, footnote reference, inline HTML, or inline math. Emphasis, strong emphasis, and strikethrough never take the `{` in, because their delimiters in the block don't pair with ones in the text (rule 1), and an autolink can't hold the space before it. The others do when the heading content is read as it is today, with the document's link and footnote definitions: a code span, inline HTML, or inline math whose end is in the block, and a link, image, or footnote reference whose label, destination, or title runs into it (as in `# Title [a {b](u) c}`). A bracket that doesn't form one of these is ordinary text, so `# Range [0, 1) {#r}` has a block, and so does `# Title [a {b] c}` unless `[a {b]` is a defined link.
3. Heading text with at least one non-whitespace character comes before the block, separated from it by one or more spaces (so a backslash-escaped brace such as `\{...}`, where `{` immediately follows `\`, is never recognized), and the whole block, from `{` to `}`, is on the line where that text ends. A block that starts or continues on another line of a Setext heading is not recognized, because joining the lines could change what the text before it reads as (a `+` line would become a list item). A block touching the text is not recognized either: Python-Markdown requires the space, and dialects with inline attributes apply a touching block to the element before it, such as a link or a `[span]`.

The block grammar:

```
block = "{" [":"] *SP attr *(1*SP attr) *SP "}"
attr  = "#" name / "." name / name "=" value / name
value = DQUOTE *dq-char DQUOTE / "'" *sq-char "'" / 1*char
```

Here `SP` is one space; `char` is any character except whitespace, `{`, `}`, `=`, `"`, `'`, and `\`; `dq-char` is any character except `"`; `sq-char` is any character except `'`; and `name` is one or more `char`s not starting with `#`, `.`, or `:`.

A quoted value ends at the next quote of the same kind. A backslash inside it does not escape the quote (for example, in `title="a\"`, the backslash is a literal character and the following `"` closes the value, whereas `title="a\"b"` is invalid). A quoted value may contain braces, whitespace, and Markdown syntax. Outside quoted values, a tab makes the block invalid, and so does a tab between the text and the block, since the formatter keeps tabs in heading text today.

Examples of invalid blocks: `{}`, `{:}`, `{::x}`, `{#}`, `{.}`, `{key=}`, `{a="b}`, `{a=b=c}`, `{x\y}`.

### Parser integration

After finding a heading and removing its heading markers, the parser checks the original text for an attribute block at the end. It does this before parsing inline Markdown, such as bold text or links.

While checking the block, the parser treats its contents as plain text. For example, `**neat**` inside an attribute value cannot become bold or match `**` elsewhere in the heading.

The parser also checks whether the opening `{` is inside Markdown that started earlier, such as a code span. If it is, the block is rejected and the whole heading is parsed normally.

If the block is accepted, the parser reads only the heading text before it as Markdown. It keeps the attribute block separately so the formatter can preserve its contents.

## Output

- The heading text is formatted as it is today. It is followed by exactly one space and the block. The space replaces the spaces between the text and the block. Hashes at the end of the text are not escaped, because the block follows them on the line. Decisions today's formatter makes from the whole heading content, such as how to write emphasis or a code span's backticks so that they read back the same, are made from the text alone, since nothing in the block can pair with them, and the output is read back with the block as plain text. Where those decisions fall back to writing the heading content exactly as in the file, that includes the spaces before the block and the block itself.
- Inside the block, each run of spaces outside quoted values becomes one space, unless it is inside an inline element that starts within the block, when the block is read on its own as Markdown with the document's link and footnote definitions. Today's formatter keeps the spaces inside a code span, inline HTML, and math, so this exception keeps the output stable where a block on a Setext line of its own, which isn't recognized, is joined onto the text's line by formatting and is then recognized. Everything else in the block is written exactly as in the source: the optional colon, attribute order, duplicates, whitespace in quoted values, and any Markdown-like text, which is not escaped or reformatted.
- The block is never broken across lines and never starts a line. The space before it is not a place where a line can break, so where the heading text wraps, the existing wrapping treats the block as part of the text's last word. The final line may exceed `lineWidth`.
- ATX headings stay on one line, as today. Level 1 and 2 headings become Setext only where the formatter already converts them, with its existing check made on the heading content including the block. The one exception is a heading whose text without the block opens a backtick code fence, as in `# ``` alpha {t=`b`}`: it stays ATX, because only the block's backtick keeps the line from being a fence, and wrapping could move the block to a later line. The underline is sized with the existing rule, counting the block as part of its line.
- Formatting the output again with the same configuration produces the same output, except where today's formatting of the heading text is already unstable.

## Examples

### ATX

With `heading.kind: atx`:

```markdown
# Title {  #id   .note   title="a  {b}"  }

# Title {:  checked  key='x'} ##

# Title ## {#id}

# Title {t="*a*"}

# foo { bar="test **neat** stuff" }

# A   title \{draft}

# A   title {#id} {unfinished

#    {#id}

# Title {}

# Title `{#id` }

# Title{#id}
```

Output:

```markdown
# Title { #id .note title="a  {b}" }

# Title {: checked key='x'}

# Title ## {#id}

# Title {t="*a*"}

# foo { bar="test **neat** stuff" }

# A title \{draft}

# A title {#id} {unfinished

# {#id}

# Title {}

# Title `{#id` }

# Title{#id}
```

The last six headings have no recognized block: a brace touching the backslash that escapes it, content that does not end with `}`, no heading text, an empty block, a block whose `{` is inside a code span, and a block touching the text.

### Setext wrapping

With `heading.kind: setext`, `textWrap: always` and `lineWidth: 10`:

```markdown
# Alpha beta gamma {#id}

# Alpha beta {#id}

### Alpha beta gamma {#id}
```

Output:

```markdown
Alpha beta
gamma {#id}
===========

Alpha
beta {#id}
==========

### Alpha beta gamma {#id}
```

`beta {#id}` does not fit after `Alpha`, so it moves to the next line as a unit. Level 3 stays ATX on one line.

### Setext underline and a block on its own line

With `heading.kind: setext` and `textWrap: maintain`:

```markdown
猫   {#id   .c}
---

Title
{#id}
---
```

Output:

```markdown
猫 {#id .c}
-----------

Title
{#id}
-----
```

The first underline has eleven hyphens because `猫` is two columns wide. The second heading's block is on a line of its own, so it is not recognized and `maintain` keeps the line break.

### Lines CommonMark does not read as one heading

With `heading.kind: atx`, a block on the line after an ATX heading is not part of the heading, because an ATX heading ends at its line. Here it is a paragraph, which the formatter writes after a blank line:

```markdown
### My Heading
{#my-ID}
```

Output:

```markdown
### My Heading

{#my-ID}
```

`###My Heading{#my-ID}` is a paragraph, not a heading, because no space follows the hashes, so it is left as it is.
