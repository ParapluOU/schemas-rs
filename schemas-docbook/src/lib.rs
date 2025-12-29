//! DocBook 5.1 XML Schemas
//!
//! This crate provides statically embedded DocBook 5.1 RelaxNG and Schematron
//! schema files that can be accessed at runtime or written to disk.
//!
//! # Example
//!
//! ```
//! use schemas_docbook::DocBook51;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in DocBook51::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Get a specific file
//! if let Some(file) = DocBook51::get_file("rng/docbook.rng") {
//!     println!("Content: {} bytes", file.contents().len());
//! }
//!
//! // Find all RNG files
//! for file in DocBook51::files_by_extension("rng") {
//!     println!("{}: {} bytes", file.path().display(), file.contents().len());
//! }
//! ```
//!
//! # License
//!
//! The DocBook schemas are licensed under BSD-2-Clause by the OASIS DocBook TC.
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../docbook/schemas");

/// DocBook 5.1 Schema Bundle
pub struct DocBook51;

impl SchemaBundle for DocBook51 {
    const NAME: &'static str = "DocBook";
    const VERSION: &'static str = "5.1";
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
        assert!(DocBook51::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_list_paths() {
        let paths: Vec<_> = DocBook51::list_paths().collect();
        assert!(!paths.is_empty());
        // DocBook uses RNG format
        assert!(paths.iter().any(|p| p.extension().is_some_and(|e| e == "rng")));
    }

    #[test]
    fn test_files_by_extension() {
        let rng_files: Vec<_> = DocBook51::files_by_extension("rng").collect();
        assert!(!rng_files.is_empty());
        for file in &rng_files {
            assert!(file.path().extension().is_some_and(|e| e == "rng"));
        }
    }

    #[test]
    fn test_summary() {
        let summary = DocBook51::summary();
        assert_eq!(summary.name, "DocBook");
        assert_eq!(summary.version, "5.1");
        assert!(summary.file_count > 0);
        assert!(summary.total_size > 0);
    }
}
