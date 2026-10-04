use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
pub struct SdlParseCallback;

impl ParseCallbacks for SdlParseCallback {
    fn process_comment(&self, comment: &str) -> Option<String> {
        let mut out = String::new();
        let mut in_code_block = false;
        let mut in_markdown_fence = false;
        let mut last_was_empty = false;

        for line in comment.lines() {
            let line = line.trim();

            // Handle \code / \endcode blocks
            if line.contains("\\endcode") {
                out.push_str("```\n");
                in_code_block = false;
                continue;
            }
            if line.contains("\\code") {
                if !out.is_empty() && !last_was_empty {
                    out.push('\n');
                }
                out.push_str("```c\n");
                in_code_block = true;
                continue;
            }

            if in_code_block {
                out.push_str(line);
                out.push('\n');
                continue;
            }

            // SDL headers use bare Markdown fences for ASCII art and plain-text
            // tables. Rustdoc compiles untagged fences as Rust doctests, so label
            // them as text to keep them rendering as code blocks instead.
            if line.starts_with("```") {
                if in_markdown_fence {
                    out.push_str("```\n");
                    in_markdown_fence = false;
                } else {
                    let language = line.trim_start_matches('`').trim();
                    if language.is_empty() {
                        out.push_str("```text\n");
                    } else {
                        out.push_str(line);
                        out.push('\n');
                    }
                    in_markdown_fence = true;
                }
                last_was_empty = false;
                continue;
            }

            if line.is_empty() {
                if !last_was_empty && !out.is_empty() {
                    out.push('\n');
                    last_was_empty = true;
                }
                continue;
            }

            // Convert SDL's Doxygen tags into markdown
            let cleaned = line
                .replace("\\brief", "**Brief:**")
                .replace("\\param", "**Parameter:**")
                .replace("\\returns", "**Returns:**")
                .replace("\\return", "**Returns:**")
                .replace("\\since", "**Available Since:**")
                .replace("\\sa", "**See Also:**")
                .replace("\\threadsafety", "**Thread Safety:**")
                .replace("\\note", "> **Note:**")
                .replace("\\warning", "> **Warning:**")
                .replace("\\deprecated", "**Deprecated:**");

            // Ensure spacing before major sections
            if (cleaned.contains("**") || cleaned.contains(">"))
                && !last_was_empty
                && !out.is_empty()
            {
                out.push('\n');
            }

            out.push_str(&cleaned);
            out.push('\n');
            last_was_empty = false;
        }

        if out.is_empty() {
            None
        } else {
            Some(out.trim().to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SdlParseCallback;
    use bindgen::callbacks::ParseCallbacks;

    #[test]
    fn bare_fences_are_tagged_as_text() {
        let comment = "Example:\n\n```\nwave \\_/\\_\n```\n";
        let rendered = SdlParseCallback
            .process_comment(comment)
            .expect("comment should render");

        assert!(
            rendered.contains("```text\nwave"),
            "bare fence should be labelled `text`; got: {rendered}"
        );
        assert!(
            rendered.contains("\n```"),
            "closing fence should remain a bare fence; got: {rendered}"
        );
    }

    #[test]
    fn doxygen_code_blocks_keep_their_language() {
        let comment = "\\code\nint x = 1;\n\\endcode\n";
        let rendered = SdlParseCallback
            .process_comment(comment)
            .expect("comment should render");

        assert!(
            rendered.contains("```c"),
            "doxygen code blocks should stay `c`; got: {rendered}"
        );
    }

    #[test]
    fn fenced_blocks_with_a_language_are_untouched() {
        let comment = "```bash\nninja -C build\n```\n";
        let rendered = SdlParseCallback
            .process_comment(comment)
            .expect("comment should render");

        assert!(
            rendered.contains("```bash"),
            "languaged fences should be preserved; got: {rendered}"
        );
        assert!(
            !rendered.contains("```text"),
            "languaged fences should not be retagged; got: {rendered}"
        );
    }
}
