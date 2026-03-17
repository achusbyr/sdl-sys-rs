use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
pub struct SdlParseCallback;

impl ParseCallbacks for SdlParseCallback {
    fn process_comment(&self, comment: &str) -> Option<String> {
        let mut out = String::new();
        let mut in_code_block = false;

        for line in comment.lines() {
            // Handle \code / \endcode blocks
            if line.contains("\\endcode") {
                out.push_str("```\n");
                in_code_block = false;
                continue;
            }
            if line.contains("\\code") {
                out.push_str("```c\n");
                in_code_block = true;
                continue;
            }

            if in_code_block {
                out.push_str(line);
                out.push('\n');
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

            out.push_str(&cleaned);
            out.push('\n');
        }
        Some(out)
    }
}
