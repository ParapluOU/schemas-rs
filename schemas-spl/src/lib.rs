//! FDA Structured Product Labeling (SPL) XML Schemas
//!
//! This crate provides statically embedded FDA SPL XSD schema files
//! for pharmaceutical package inserts that can be accessed at runtime
//! or written to disk.
//!
//! SPL is an HL7 standard adopted by the FDA for exchanging drug product
//! labeling information including prescription drug labels, OTC drug labels,
//! and medical device listings.
//!
//! # Example
//!
//! ```
//! use schemas_spl::Spl;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in Spl::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Get the main SPL schema
//! if let Some(file) = Spl::get_file("SPL.xsd") {
//!     println!("Content: {} bytes", file.contents().len());
//! }
//!
//! // Find all XSD files
//! for file in Spl::files_by_extension("xsd") {
//!     println!("{}: {} bytes", file.path().display(), file.contents().len());
//! }
//! ```
//!
//! # License
//!
//! The SPL schemas are licensed under a BSD-3-Clause style license by HL7.
//! See the LICENSE file and schema file headers for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../spl/schemas");

/// FDA Structured Product Labeling Schema Bundle
pub struct Spl;

impl SchemaBundle for Spl {
    const NAME: &'static str = "SPL";
    const VERSION: &'static str = "R2b";
    const LICENSE: &'static str = "BSD-3-Clause";

    fn dir() -> &'static Dir<'static> {
        &SCHEMA_DIR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(Spl::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_list_paths() {
        let paths: Vec<_> = Spl::list_paths().collect();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.extension().is_some_and(|e| e == "xsd")));
    }

    #[test]
    fn test_files_by_extension() {
        let xsd_files: Vec<_> = Spl::files_by_extension("xsd").collect();
        assert!(!xsd_files.is_empty());
        for file in &xsd_files {
            assert!(file.path().extension().is_some_and(|e| e == "xsd"));
        }
    }

    #[test]
    fn test_has_main_spl_schema() {
        assert!(Spl::get_file("SPL.xsd").is_some(), "Should have SPL.xsd");
    }

    #[test]
    fn test_summary() {
        let summary = Spl::summary();
        assert_eq!(summary.name, "SPL");
        assert_eq!(summary.version, "R2b");
        assert!(summary.file_count > 0);
        assert!(summary.total_size > 0);
    }
}
