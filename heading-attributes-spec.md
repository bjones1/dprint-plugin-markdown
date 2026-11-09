# Heading attributes

A heading may end with an attribute block, as in `# Title {#id .class key="value"}`. The formatter keeps the block intact and attached to the heading while it formats the heading text as it already does. CommonMark does not define attribute blocks, so the syntax of the block's contents follows [Python-Markdown's `attr_list`](https://python-markdown.github.io/extensions/attr_list/), with the stricter grammar below. This adds no configuration options and does not check what the attributes mean (for example, duplicate ids are kept).

Everything not described here is unchanged: a heading without a recognized block, including every case below that fails recognition, is formatted exactly as it is today, with its braces treated as ordinary text.

## Recognition

A heading has an attribute block when all of these hold:

1. The heading content ends with a block, ignoring trailing spaces and tabs. The heading content is what CommonMark reads as the heading's text, so an ATX closing sequence after the block (`# Title {#id} ##`) is already gone, while hashes before the block (`# Title ## {#id}`) are heading text.
2. The block's opening `{` is not escaped (preceded by an odd number of backslashes) and is not inside a code span, link, image, emphasis, inline HTML, math, or autolink that starts before it. Both braces are on the same source line.
3. Heading text with at least one non-whitespace character comes before the block. The text may touch the `{`. Between the text and the block there may be spaces, tabs, or, in a Setext heading, the line break that ends a line of the text. A hard line break right before the block prevents recognition.

The block grammar, where `SP` is one space character:

```
block  = "{" [":"] *SP attr *(1*SP attr) *SP "}"
attr   = "#" name / "." name / name "=" value / name
value  = DQUOTE *not-DQUOTE DQUOTE / "'" *not-"'" "'" / 1*char
name   = 1*char, not starting with "#" or "."
char   = any character except whitespace, "{", "}", "=", DQUOTE, "'", "\"
```

A quoted value ends at the next quote of the same kind. A backslash inside it has no special meaning, as in Python-Markdown. A quoted value may contain any other character of the line, including braces, tabs, and Markdown syntax. Outside quoted values, only single spaces separate attributes, so a tab or any other whitespace there makes the block invalid.

Only the block that ends the heading content is considered. When it is invalid or unclosed, no earlier block is used. Examples of invalid blocks: `{}`, `{:}`, `{#}`, `{.}`, `{key=}`, `{a="b}`, `{a=b=c}`, `{x\y}`, and `{#id .c}` written with a tab instead of the space.

## Output

- The heading text is formatted as it is today. It is followed by exactly one space and the block. The space replaces whatever separated the text from the block, including the line break of a Setext heading, in every `textWrap` mode.
- Inside the block, each run of spaces outside quoted values becomes one space. Everything else is written exactly as in the source: the optional colon, attribute order, duplicates, quoted values with their whitespace, and any Markdown-like text, which is not escaped or reformatted.
- The block is never broken across lines. When the heading text wraps, the last word of the text and the block are one unbreakable unit: they move to the next line together and stay together even when they exceed `lineWidth`.
- ATX headings stay on one line, as today. Level 1 and 2 headings become Setext only where the formatter already converts them, and the underline is sized with the existing rule, counting the block as part of its line.
- Formatting the output again with the same configuration produces the same output.

## Examples

### ATX

With `heading.kind: atx`:

```markdown
# Title{#id}

# Title {  #id   .note   title="a  {b}"  }

# Title {:  checked  key='x'} ##

# Title ## {#id}

# Title {t="*a*"}

# A   title\{draft}

# A   title {#id} {unfinished

#    {#id}

# Title {}

# `code {#id}`
```

Output:

```markdown
# Title {#id}

# Title { #id .note title="a  {b}" }

# Title {: checked key='x'}

# Title ## {#id}

# Title {t="*a*"}

# A title\{draft}

# A title {#id} {unfinished

# {#id}

# Title {}

# `code {#id}`
```

The last five headings have no recognized block: an escaped brace, an unclosed final candidate, no heading text, an empty block, and a block inside a code span.

### Setext wrapping

With `heading.kind: setext`, `textWrap: always` and `lineWidth: 10`:

```markdown
# Alpha beta gamma{#id}

# Alpha beta{#id}

### Alpha beta gamma{#id}
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

### Setext line break before the block

With `heading.kind: setext` and `textWrap: maintain`:

```markdown
猫
{#id}
---
```

Output:

```markdown
猫 {#id}
--------
```

The line break becomes one space even though `maintain` keeps line breaks. The underline has eight hyphens because `猫` is two columns wide.

### Lines CommonMark does not read as one heading

With `heading.kind: atx`, a block on the line after an ATX heading is a separate paragraph, because an ATX heading ends at its line. The formatter writes the paragraph after a blank line:

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
