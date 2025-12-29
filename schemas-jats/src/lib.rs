//! JATS (Journal Article Tag Suite) 1.4 XML Schemas
//!
//! This crate provides statically embedded JATS 1.4 XSD schema files
//! with MathML 3 support that can be accessed at runtime or written to disk.
//!
//! # Example
//!
//! ```
//! use schemas_jats::Jats14;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in Jats14::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Get a specific file
//! if let Some(file) = Jats14::get_file("JATS-journalpublishing1-4-mathml3.xsd") {
//!     println!("Content: {} bytes", file.contents().len());
//! }
//!
//! // Find all XSD files
//! for file in Jats14::files_by_extension("xsd") {
//!     println!("{}: {} bytes", file.path().display(), file.contents().len());
//! }
//! ```
//!
//! # License
//!
//! The JATS schemas are in the public domain (NLM/NIH).
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../jats/schemas");

/// JATS 1.4 Schema Bundle (Journal Article Tag Suite)
pub struct Jats14;

impl SchemaBundle for Jats14 {
    const NAME: &'static str = "JATS";
    const VERSION: &'static str = "1.4";
    const LICENSE: &'static str = "Public Domain";

    fn dir() -> &'static Dir<'static> {
        &SCHEMA_DIR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(Jats14::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_list_paths() {
        let paths: Vec<_> = Jats14::list_paths().collect();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.extension().is_some_and(|e| e == "xsd")));
    }

    #[test]
    fn test_files_by_extension() {
        let xsd_files: Vec<_> = Jats14::files_by_extension("xsd").collect();
        assert!(!xsd_files.is_empty());
        for file in &xsd_files {
            assert!(file.path().extension().is_some_and(|e| e == "xsd"));
        }
    }

    #[test]
    fn test_summary() {
        let summary = Jats14::summary();
        assert_eq!(summary.name, "JATS");
        assert_eq!(summary.version, "1.4");
        assert!(summary.file_count > 0);
        assert!(summary.total_size > 0);
    }
}
