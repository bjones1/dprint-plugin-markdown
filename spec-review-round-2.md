# Specification Review - Round 2

**Date**: 2026-11-09
**Reviewer**: Claude 4 Sonnet (code-review skill, medium effort)
**Document**: indented-code-tabs-spec.md (after Round 1 fixes)

## Findings: 3 issues (1 CONFIRMED, 2 PLAUSIBLE)

### 1. Duplicate Example 4 (CONFIRMED, medium severity)
**Location**: Lines 346-362
**Issue**: Example 3 and Example 4 were identical duplicates - both titled "Code Block After Paragraph" with same config, input, and output.

**Resolution**: Replaced duplicate Example 4 with "Example 4: Mixed Indentation Input" showing:
- Input with mixed tabs and spaces (4 spaces, 1 tab, 6 spaces)
- Normalization behavior (all converted to configured style)
- Internal indentation preservation (6 spaces → 1 tab + 2 spaces)

### 2. Oracle test lacks AST comparison criteria (PLAUSIBLE, low severity)
**Location**: Lines 246, 256
**Issue**: Testing strategy mentioned "equivalent ASTs" but didn't specify:
- Which CommonMark parser to use
- Which AST node properties to compare
- What differences to ignore (positions, metadata)

**Resolution**: Enhanced Testing Strategy section with explicit criteria:
- Specified parser: `pulldown-cmark`
- Comparison criteria: node types, code block content (after line ending normalization)
- Ignored fields: source positions, byte offsets, indentation representation metadata
- Added nested container test case (code in lists in block quotes)

### 3. Missing mixed tab/space input example (PLAUSIBLE, low severity)
**Location**: Behavior Specification examples section
**Issue**: No example showed input with both tabs and spaces for code block indentation. Design Decisions stated "input is normalized" but no concrete demonstration.

**Resolution**: Added Example 4 showing mixed indentation:
- 4 spaces → 1 tab
- 1 tab → 1 tab (already correct)
- 6 spaces → 1 tab + 2 spaces (4 for block, 2 internal)

## Changes Summary

**Lines changed**: ~25 lines modified
- Replaced duplicate Example 4 (15 lines)
- Enhanced Testing Strategy with AST comparison details (10 lines)

## Refuted Findings

The following candidate issues were investigated and **refuted** (not actual problems):

1. **Blank line behavior vs CommonMark**: Spec correctly states blank lines have no indentation, consistent with CommonMark's "minus four spaces" rule
2. **Inconsistent "4-column" vs "4-space" terminology**: Terms are used distinctly and correctly (4-space = literal spaces, 4-column = visual width including tabs)
3. **"Top-level" definition mismatch**: Terminology and Scope sections are consistent (top-level includes code in lists/quotes, but only the 4-column code marker is affected)

## Specification Status

**Ready for implementation**: Yes, with high confidence

The specification is now clear, internally consistent, and provides sufficient detail for implementation and testing. Two rounds of review have addressed all major ambiguities, contradictions, and missing examples.

## Round Comparison

| Metric | Round 1 | Round 2 |
|--------|---------|---------|
| Findings (confirmed/plausible) | 5 | 3 |
| Severity HIGH issues | 2 | 0 |
| Lines changed | ~110 | ~25 |
| Meta-commentary removed | Yes | N/A |
| Open questions resolved | 3 | 0 |
