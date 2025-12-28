//! OASIS DITA 1.2 XML Schemas
//!
//! This crate provides statically embedded DITA 1.2 XSD schema files
//! that can be accessed at runtime or written to disk.
//!
//! # Example
//!
//! ```
//! use schemas_dita::Dita12;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in Dita12::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Get a specific file
//! if let Some(file) = Dita12::get_file("xsd1.2/base/xsd/basemap.xsd") {
//!     println!("Content: {} bytes", file.contents().len());
//! }
//!
//! // Find all XSD files
//! for file in Dita12::files_by_extension("xsd") {
//!     println!("{}: {} bytes", file.path().display(), file.contents().len());
//! }
//! ```
//!
//! # License
//!
//! The DITA schemas are licensed under the OASIS IPR Policy.
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../dita/schemas");

/// DITA 1.2 Schema Bundle
pub struct Dita12;

impl SchemaBundle for Dita12 {
    const NAME: &'static str = "DITA";
    const VERSION: &'static str = "1.2";
    const LICENSE: &'static str = "OASIS-IPR";

    fn dir() -> &'static Dir<'static> {
        &SCHEMA_DIR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(Dita12::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_list_paths() {
        let paths: Vec<_> = Dita12::list_paths().collect();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.extension().is_some_and(|e| e == "xsd")));
    }

    #[test]
    fn test_files_by_extension() {
        let xsd_files: Vec<_> = Dita12::files_by_extension("xsd").collect();
        assert!(!xsd_files.is_empty());
        for file in &xsd_files {
            assert!(file.path().extension().is_some_and(|e| e == "xsd"));
        }
    }

    #[test]
    fn test_summary() {
        let summary = Dita12::summary();
        assert_eq!(summary.name, "DITA");
        assert_eq!(summary.version, "1.2");
        assert!(summary.file_count > 0);
        assert!(summary.total_size > 0);
    }
}
