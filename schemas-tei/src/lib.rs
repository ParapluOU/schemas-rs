//! TEI (Text Encoding Initiative) P5 XML Schemas
//!
//! This crate provides statically embedded TEI P5 RelaxNG and XSD schema files
//! for humanities and social sciences text encoding that can be accessed at
//! runtime or written to disk.
//!
//! # Example
//!
//! ```
//! use schemas_tei::TeiP5;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in TeiP5::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Get a specific file
//! if let Some(file) = TeiP5::get_file("tei_all.xsd") {
//!     println!("Content: {} bytes", file.contents().len());
//! }
//!
//! // Find all RNG files
//! for file in TeiP5::files_by_extension("rng") {
//!     println!("{}: {} bytes", file.path().display(), file.contents().len());
//! }
//! ```
//!
//! # License
//!
//! The TEI schemas are licensed under BSD-2-Clause by the TEI Consortium.
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../tei/schemas");

/// TEI P5 Schema Bundle (Text Encoding Initiative)
pub struct TeiP5;

impl SchemaBundle for TeiP5 {
    const NAME: &'static str = "TEI";
    const VERSION: &'static str = "P5";
    const LICENSE: &'static str = "BSD-2-Clause";

    fn dir() -> &'static Dir<'static> {
        &SCHEMA_DIR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(TeiP5::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_list_paths() {
        let paths: Vec<_> = TeiP5::list_paths().collect();
        assert!(!paths.is_empty());
        // TEI has both RNG and XSD
        assert!(paths.iter().any(|p| {
            p.extension().is_some_and(|e| e == "rng" || e == "xsd")
        }));
    }

    #[test]
    fn test_files_by_extension() {
        // TEI has XSD format available
        let xsd_files: Vec<_> = TeiP5::files_by_extension("xsd").collect();
        assert!(!xsd_files.is_empty());
        for file in &xsd_files {
            assert!(file.path().extension().is_some_and(|e| e == "xsd"));
        }
    }

    #[test]
    fn test_summary() {
        let summary = TeiP5::summary();
        assert_eq!(summary.name, "TEI");
        assert_eq!(summary.version, "P5");
        assert!(summary.file_count > 0);
        assert!(summary.total_size > 0);
    }
}
