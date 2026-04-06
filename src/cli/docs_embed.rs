//! Embedded tutorial content using include_dir
//!
//! Compiles docs/src/ markdown files into the binary at compile time.
//! No runtime file I/O required.

use include_dir::{include_dir, Dir};

/// Embed the entire docs/src/ directory at compile time
pub const DOCS_DIR: Dir = include_dir!("docs/src");

/// Get the content of a tutorial markdown file
///
/// # Arguments
/// * `doc_path` - Relative path from docs/src/ (e.g., "basic/ownership.md")
///
/// # Returns
/// * `Some(&'static str)` - The markdown content
/// * `None` - File not found
pub fn get_tutorial_content(doc_path: &str) -> Option<&'static str> {
    DOCS_DIR
        .get_file(doc_path)
        .map(|f| f.contents_utf8().unwrap_or(""))
}

/// Display tutorial content with formatting
///
/// # Arguments
/// * `doc_path` - Relative path from docs/src/
///
/// # Returns
/// * `true` - Tutorial was found and displayed
/// * `false` - Tutorial not found
pub fn display_tutorial(doc_path: &str) -> bool {
    if let Some(content) = get_tutorial_content(doc_path) {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("{}", content);
        true
    } else {
        false
    }
}

/// List all markdown files in a category directory
///
/// # Arguments
/// * `category` - Category name (e.g., "basic", "advance")
///
/// # Returns
/// Iterator over file paths
pub fn list_category_docs(category: &str) -> Vec<String> {
    let mut docs = Vec::new();

    if let Some(dir) = DOCS_DIR.get_dir(category) {
        for entry in dir.entries() {
            if let Some(file) = entry.as_file() {
                if file.path().extension().map_or(false, |ext| ext == "md") {
                    if let Some(path) = file.path().to_str() {
                        docs.push(path.to_string());
                    }
                }
            }
        }
    }

    docs
}
