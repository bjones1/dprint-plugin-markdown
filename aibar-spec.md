# Heading attribute formatting

Keep heading attributes on one line while letting Setext heading text wrap normally. Limit the feature to formatting existing heading attributes in Markdown source under the existing configuration. Validate attribute token syntax only.

CommonMark leaves heading attributes undefined. The formatter needs a syntax to distinguish attributes from literal brace text, so use [Python-Markdown’s `attr_list`](Attribute Lists Extension - Python-Markdown) as the reference for the contents of the braces. The rules below govern placement and formatting in dprint.

## Recognition

- Accept IDs, classes, key/value pairs, quoted values, valueless attributes, and an optional colon after `{`. Validate names as attribute tokens and keep their spelling and duplicates. Treat `-` in `{-}` as a valueless attribute.
- Recognize only the final candidate after nonempty heading text. The block may touch the text, but the opening and closing braces must be on the same source line. Require a complete parse. Treat any unparsed text as a validation failure, even if Python-Markdown would discard it during recovery. For an empty, unclosed, or invalid candidate, format the heading normally. An earlier candidate never takes its place.
- Allow ATX closing hashes before or after the block and complete HTML comments after the block and any closing hashes. Each comment must be on the same source line as the block. Other trailing text prevents recognition.
- Ignore braces inside quoted values when finding the block boundaries. Outside quotes, an odd number of consecutive backslashes escapes either brace. Braces inside code, emphasis, links, images, or HTML stay part of those constructs. Comment-like text inside code or an attribute value stays part of that content.
- For undocumented tokenization details, record the Python-Markdown version used and cover those details with tests.

## Output

- Format and wrap the heading text normally before appending the block to its final line with exactly one space. This also replaces a Setext line break before the block. Keep the block intact and let the final line exceed `lineWidth` if needed. Keep the line breaks chosen for the heading text even when the appended block makes the final line overflow.
- Inside the braces, collapse runs of spaces outside quoted values to one space. Preserve tabs, quoted whitespace, spelling, punctuation, escapes, entity spellings, and attribute order. Treat the contents as attribute syntax, including any Markdown-like text.
- Remove ATX closing hashes. Append each trailing comment unchanged and in order, with one space before it.
- Keep ATX headings on one line. Carry attributes through ATX and Setext conversions. Setext is available for levels 1 and 2 where conversion is safe. Size its underline to the widest output content line, including attributes and comments, using the formatter’s existing display-width and container rules.
- Preserve ordinary formatting for other blocks. Formatting the result again with the same settings must produce identical output.

## Examples

### ATX spacing, comments, and literal braces

Use `heading.kind: atx`. The first example has a tab before `title` and two spaces inside the quoted value. Both are preserved. An unclosed final candidate leaves both brace groups to ordinary formatting.

**Input**

```markdown
# Title{ #id  .note title="a  {b}" }
 
# Title ##   {: checked key=one key=two}   <!-- first -->  <!-- second -->
 
# Title {#id} ## <!-- note -->
 
# A   title\{draft}
 
# A   title { #id  .note } {unfinished
 
#    {#id}
```

**Output**

```markdown
# Title { #id .note title="a  {b}" }
 
# Title {: checked key=one key=two} <!-- first --> <!-- second -->
 
# Title {#id} <!-- note -->
 
# A title\{draft}
 
# A title { #id .note } {unfinished
 
# {#id}
```

### Setext wrapping and overflow

Use `heading.kind: setext`, `textWrap: always`, and `lineWidth: 10`. `Alpha beta` stays on the first line and the block makes the final line 11 columns wide. Level 3 stays ATX.

**Input**

```markdown
# Alpha beta gamma{#id}
 
### Alpha beta gamma{#id}
```

**Output**

```markdown
Alpha beta
gamma {#id}
===========
 
### Alpha beta gamma {#id}
```

### Setext line break before the block

Use `heading.kind: setext` and `textWrap: maintain`. The line break before the block becomes one space even in this mode. The underline has eight hyphens because `猫` takes two columns.

**Input**

```markdown
猫
{#id}
---
```

**Output**

```markdown
猫 {#id}
--------
```

Each Setext result reparses as one heading at its original level.

## CommonMark edge cases

These examples use `heading.kind: atx` and `textWrap: never`. They preserve the block structure defined by [CommonMark’s ATX heading rules](https://spec.commonmark.org/0.31.2/#atx-headings).

### A brace block on the line after an ATX heading

The ATX heading ends at the newline. The formatter separates it from the following paragraph with a blank line.

**Input**

```markdown
### My Heading
{#my-ID}
```

**Expected output**

```markdown
### My Heading
 
{#my-ID}
```

The output reparses as a level 3 heading followed by a paragraph.

### Opening hashes without a space before the title

The opening hashes touch the title, so CommonMark parses the entire line as a paragraph. The braces remain literal paragraph text.

**Input and expected output**

```markdown
###My Heading{#my-ID}
```
