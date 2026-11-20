Project 2 submission
====================

Team name: dprint heading attributes 2

The specification consists of the file
[heading-attributes-2-specification.md](heading-attributes-2-specification.md).


### Alisher Toxanbayev

GitHub userid: AlisherToxanbayev

#### Contributions

Led the preparation of the heading-attributes specification by coordinating repository analysis, defining the project scope and regression-test strategy, and incorporating feedback from the human reviewer, Codex, and Claude.

#### Build and run test

TODO (Alisher): summarize the LLM output for the prompt "Verify that you can build and run the project in the current git repo." The Codex environment used earlier had no `cargo`, so that run could not build the project; repeat it now that Rust is installed.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 0 | A same-context Codex review of an earlier draft reported no actionable findings. |
| 2 | 6 | Claude review: undefined attribute grammar, Setext test-row contradiction, missing ATX-to-Setext check, ambiguous inline attachment, unclear width semantics. |
| 3 | 3 | Codex review: weak regression row fixed; unexecuted tests and incomplete reconciliation of the Claude report noted. |
| 4 | 6 | Claude review: over-broad inline attachment, ordinary text promoted to attributes, separator ambiguity, unspecified recognition stage, semantic comparison gaps. |
| 5 | 3 | Codex review: duplicate regression row corrected; unexecuted tests and incomplete reconciliation noted. |

#### Last specification review

Prompting Codex (reported as GPT-6; the exact build identifier and the reasoning effort were not exposed) with the prompt below produced the following output.

> Review `heading-attributes-2-specification.md` and `submission.md` against the current repository. Check the parser grammar and edge cases against the pinned pulldown-cmark 0.13.4 specification; check source claims about heading parsing, output conversion, wrapping and width; inspect test matrix coverage and arithmetic; and assess whether the semantic test compares the complete document without hiding meaningful differences. Identify remaining actionable inaccuracies, contradictions, omissions, and unsupported execution claims. Do not infer missing Claude materials or user/team identity. Return only findings with concrete corrections.

> 1. Row 15 currently repeats the invalid internally split text from row 8 while describing a valid existing attribute block. Replace it with a complete single-line attribute block so this row tests the converse case.
> 2. Submission review history must not state the Claude round is fully reconciled until the complete report is supplied and checked. The current explicit TODO is appropriate; team member names and individual build outputs also remain required.
> 3. No formatter/parser behavior or proposed fixture output has been executed because `cargo` is unavailable. Keep the row-8 join behavior labeled as an inference and the entire matrix labeled not run.

Finding 1 was applied: row 15 now uses the valid single-line block `Heading {#my-id .foo}`. Findings 2 and 3 were accurate at that time.

### Azamat Ayapbergenov

GitHub userid: azaayapbergenov

#### Contributions

Built and tested the project locally, reproduced the attribute-splitting behavior of the formatter and checked it with pulldown-cmark, and wrote the verified-behavior section of the specification.

#### Build and run test

Prompting GitHub Copilot in VS Code (model: Fusion, effort level: standard) with "Verify that you can build and run the project in the current git repo." produced this result, summarized: the build and run verification passed. `cargo build --all-features` succeeded, and `cargo test --all-targets` succeeded with all 182 reported tests passing. Copilot noted that the project is a Rust library/plugin rather than a standalone executable, so it was exercised through its tests; a linker message in Russian appeared but the build completed, and no source files were changed.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 3 | Added the line-break notation to the verified-behavior table; not applied: two claims about matrix rows 20 and 31 that running pulldown-cmark 0.13.4 showed to be wrong. |
| 2 | 2 | Gave a concrete fixture for row 5 and the exact underline for row 4 of the verified-behavior table. |
| 3 | 1 | Added row 7 to the verified-behavior table after confirming that dropping closing hashes turns the brace text `{#id}` into a heading ID. |

#### Last specification review

Prompting GitHub Copilot in VS Code (model: Fusion, effort level: standard) with the prompt below produced the following output. No finding was rejected, so nothing is struck through.

> Review only the section named "Verified behavior of the current formatter" in heading-attributes-2-specification.md in the current git repo. Read the rest of the document and the source code only to check claims made in that section, and do not review other sections.
>
> Do not edit any file and do not rewrite the section. Do not propose new features, new Markdown dialects, or extra edge cases unless they contradict something already written in that section.
>
> Look for: contradictions within the section or with the rest of the specification, ambiguities where an implementer would have to guess, table rows whose stated output does not follow from the stated input and configuration, claims that conflict with the existing formatter code or with pulldown-cmark 0.13.4 behavior, and statements that cannot be tested.
>
> Report numbered findings in the chat. For each: a one-line title, a severity (high/medium/low), the exact text you are criticising, a short explanation, and a one-sentence minimal fix. End with the number of findings and a one-line list of their titles.
>
> Findings already considered and rejected in earlier rounds, which must not be raised again:
> - Escaped opening brace: pulldown-cmark 0.13.4 gives the class foo for `# \{.foo}` (checked by running it); the row is also outside the section under review.
> - Vertical tab: pulldown-cmark 0.13.4 gives the single class `a\u{b}.b` (checked by running it); the row is also outside the section under review.

1. Closing hashes can be lost and expose a new attribute — Severity: medium
   * Criticised text: `| 19 | #### Heading {#id} #### | ... | Keep block before closing hashes as ordinary text | ...; no ID |`
   * Explanation: The existing ATX parser strips trailing closing hashes into AST metadata before formatting, while the generator emits a heading whose brace text now appears trailing. Pulldown-cmark can then interpret `{#id}` as a valid attribute block, changing the heading semantics from no ID to ID `id`.
   * Minimal fix: Preserve the closing-hash suffix whenever it follows brace text, so the original heading remains `#### Heading {#id} ####`.

1 finding

* Closing hashes can be lost and expose a new attribute

Applied: I ran the finding. With default settings the formatter turns `#### Heading {#id} ####` into `#### Heading {#id}`, and pulldown-cmark 0.13.4 reads no ID in the input but the ID `id` in the output. This is now row 7 of the verified-behavior table. The suggested fix (preserving the closing hashes) is a change to the formatter, not to the specification, so it was not added to the text.

