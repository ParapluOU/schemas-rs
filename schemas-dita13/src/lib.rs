//! OASIS DITA 1.3 XML Schemas
//!
//! This crate provides statically embedded DITA 1.3 XSD schema files
//! that can be accessed at runtime or written to disk.
//!
//! DITA 1.3 includes significant enhancements over 1.2 including:
//! - Troubleshooting topic type
//! - New domains (XML mention, SVG, MathML, release management)
//! - Enhanced key processing and branch filtering
//! - Learning and Training improvements
//!
//! # Example
//!
//! ```
//! use schemas_dita13::Dita13;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in Dita13::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Get a specific file
//! if let Some(file) = Dita13::get_file("base/xsd/topicMod.xsd") {
//!     println!("Content: {} bytes", file.contents().len());
//! }
//!
//! // Find all XSD files
//! for file in Dita13::files_by_extension("xsd") {
//!     println!("{}: {} bytes", file.path().display(), file.contents().len());
//! }
//! ```
//!
//! # License
//!
//! The DITA 1.3 schemas are licensed under Apache-2.0.
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../dita13/schemas");

/// DITA 1.3 Schema Bundle
pub struct Dita13;

impl SchemaBundle for Dita13 {
    const NAME: &'static str = "DITA";
    const VERSION: &'static str = "1.3";
    const LICENSE: &'static str = "Apache-2.0";

    fn dir() -> &'static Dir<'static> {
        &SCHEMA_DIR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(Dita13::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_list_paths() {
        let paths: Vec<_> = Dita13::list_paths().collect();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.extension().is_some_and(|e| e == "xsd")));
    }

    #[test]
    fn test_files_by_extension() {
        let xsd_files: Vec<_> = Dita13::files_by_extension("xsd").collect();
        assert!(!xsd_files.is_empty());
        for file in &xsd_files {
            assert!(file.path().extension().is_some_and(|e| e == "xsd"));
        }
    }

    #[test]
    fn test_summary() {
        let summary = Dita13::summary();
        assert_eq!(summary.name, "DITA");
        assert_eq!(summary.version, "1.3");
        assert!(summary.file_count > 0);
        assert!(summary.total_size > 0);
    }
}
