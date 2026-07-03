//! Guards the machine-readable schema copy against drift from the spec.
//!
//! The normative CDDL lives in the fenced ```cddl block of
//! `docs/draft-nuttycom-zewif.md`; `docs/zewif.cddl` is a verbatim extract of
//! that block for consumption by tooling. This test fails whenever the two
//! fall out of lockstep.

use std::path::Path;

/// Extracts the contents of the sole ```cddl fenced code block from the
/// given Markdown source, preserving it byte-for-byte (each line terminated
/// by a newline, fence lines excluded).
fn extract_cddl_block(markdown: &str) -> String {
    let mut blocks = Vec::new();
    let mut current: Option<String> = None;
    for line in markdown.lines() {
        match current.as_mut() {
            Some(block) => {
                if line.trim_end() == "```" {
                    blocks.push(current.take().expect("block is present"));
                } else {
                    block.push_str(line);
                    block.push('\n');
                }
            }
            None => {
                if line.trim_end() == "```cddl" {
                    current = Some(String::new());
                }
            }
        }
    }
    assert!(
        current.is_none(),
        "unterminated ```cddl fenced block in the spec document"
    );
    assert_eq!(
        blocks.len(),
        1,
        "expected exactly one ```cddl fenced block in the spec document, found {}",
        blocks.len()
    );
    blocks.pop().expect("exactly one block is present")
}

#[test]
fn machine_readable_schema_matches_spec() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec = std::fs::read_to_string(manifest_dir.join("docs/draft-nuttycom-zewif.md"))
        .expect("spec document is readable");
    let machine_copy = std::fs::read_to_string(manifest_dir.join("docs/zewif.cddl"))
        .expect("machine-readable schema is readable");

    assert_eq!(
        extract_cddl_block(&spec),
        machine_copy,
        "docs/zewif.cddl has drifted from the CDDL block in \
         docs/draft-nuttycom-zewif.md; re-extract the fenced block verbatim"
    );
}
