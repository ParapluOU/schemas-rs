//! Akoma Ntoso 3.0 XML Schemas
//!
//! This crate provides statically embedded Akoma Ntoso 3.0 XSD schema files
//! for legal document interchange that can be accessed at runtime or written to disk.
//!
//! # Example
//!
//! ```
//! use schemas_akoma_ntoso::AkomaNtoso30;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in AkomaNtoso30::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Get a specific file
//! if let Some(file) = AkomaNtoso30::get_file("akomantoso30.xsd") {
//!     println!("Content: {} bytes", file.contents().len());
//! }
//!
//! // Find all XSD files
//! for file in AkomaNtoso30::files_by_extension("xsd") {
//!     println!("{}: {} bytes", file.path().display(), file.contents().len());
//! }
//! ```
//!
//! # License
//!
//! The Akoma Ntoso schemas are licensed under CC-BY-4.0 by OASIS Open.
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../akoma-ntoso/schemas");

/// Akoma Ntoso 3.0 Schema Bundle (Legal Documents)
pub struct AkomaNtoso30;

impl SchemaBundle for AkomaNtoso30 {
    const NAME: &'static str = "Akoma Ntoso";
    const VERSION: &'static str = "3.0";
    const LICENSE: &'static str = "CC-BY-4.0";

    fn dir() -> &'static Dir<'static> {
        &SCHEMA_DIR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(AkomaNtoso30::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_list_paths() {
        let paths: Vec<_> = AkomaNtoso30::list_paths().collect();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.extension().is_some_and(|e| e == "xsd")));
    }

    #[test]
    fn test_files_by_extension() {
        let xsd_files: Vec<_> = AkomaNtoso30::files_by_extension("xsd").collect();
        assert!(!xsd_files.is_empty());
        for file in &xsd_files {
            assert!(file.path().extension().is_some_and(|e| e == "xsd"));
        }
    }

    #[test]
    fn test_summary() {
        let summary = AkomaNtoso30::summary();
        assert_eq!(summary.name, "Akoma Ntoso");
        assert_eq!(summary.version, "3.0");
        assert!(summary.file_count > 0);
        assert!(summary.total_size > 0);
    }
}
