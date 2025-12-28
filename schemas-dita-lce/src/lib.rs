//! DITA Learning Content Education (LCE) Schemas
//!
//! This crate provides statically embedded DITA LCE schema files.
//! LCE is a DITA specialization for educational materials and publishers.
//!
//! # Example
//!
//! ```
//! use schemas_dita_lce::DitaLce;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in DitaLce::list_paths() {
//!     println!("{}", path);
//! }
//! ```
//!
//! # License
//!
//! The DITA LCE schemas are licensed under the Apache License 2.0.
//! Copyright 2016 Birgit Strackenbrock (XStructuring) and contributors.

pub use schemas_core::{BundleSummary, SchemaBundle, SchemaBundleExt, SchemaError, SchemaFile};

// Include the generated schema files
include!(concat!(env!("OUT_DIR"), "/generated_schemas.rs"));

/// DITA LCE Schema Bundle
pub struct DitaLce;

impl SchemaBundle for DitaLce {
    const NAME: &'static str = "DITA LCE";
    const VERSION: &'static str = "3.0";
    const LICENSE: &'static str = "Apache-2.0";

    fn files() -> &'static [SchemaFile] {
        SCHEMA_FILES
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(DitaLce::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_summary() {
        let summary = DitaLce::summary();
        assert_eq!(summary.name, "DITA LCE");
        assert!(summary.file_count > 0);
    }
}
