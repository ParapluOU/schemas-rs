//! NISO Standards Tag Suite (STS) Schemas
//!
//! This crate provides statically embedded NISO STS 1.0 XSD schema files.
//! NISO STS is a standard for the XML encoding of standards documents.
//!
//! # Included Tag Sets
//!
//! - **Interchange**: For exchange between organizations
//! - **Extended**: With additional elements for publishing
//!
//! Both tag sets include MathML 3.0 support.
//!
//! # Example
//!
//! ```
//! use schemas_niso_sts::NisoSts;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in NisoSts::list_paths() {
//!     println!("{}", path);
//! }
//!
//! // Find interchange schemas
//! for file in NisoSts::find_files(|f| f.path.contains("interchange")) {
//!     println!("{}", file.path);
//! }
//! ```
//!
//! # License
//!
//! NISO STS schemas are made openly available for public use.
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, SchemaBundle, SchemaBundleExt, SchemaError, SchemaFile};

// Include the generated schema files
include!(concat!(env!("OUT_DIR"), "/generated_schemas.rs"));

/// NISO STS Schema Bundle
pub struct NisoSts;

impl SchemaBundle for NisoSts {
    const NAME: &'static str = "NISO STS";
    const VERSION: &'static str = "1.0";
    const LICENSE: &'static str = "NISO";

    fn files() -> &'static [SchemaFile] {
        SCHEMA_FILES
    }
}

impl NisoSts {
    /// Get interchange tag set schemas only.
    pub fn interchange_files() -> impl Iterator<Item = &'static SchemaFile> {
        Self::find_files(|f| f.path.contains("interchange"))
    }

    /// Get extended tag set schemas only.
    pub fn extended_files() -> impl Iterator<Item = &'static SchemaFile> {
        Self::find_files(|f| f.path.contains("extended"))
    }

    /// Get MathML schemas.
    pub fn mathml_files() -> impl Iterator<Item = &'static SchemaFile> {
        Self::find_files(|f| f.path.contains("mathml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(NisoSts::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_has_interchange_and_extended() {
        let interchange: Vec<_> = NisoSts::interchange_files().collect();
        let extended: Vec<_> = NisoSts::extended_files().collect();

        assert!(!interchange.is_empty(), "Should have interchange schemas");
        assert!(!extended.is_empty(), "Should have extended schemas");
    }

    #[test]
    fn test_summary() {
        let summary = NisoSts::summary();
        assert_eq!(summary.name, "NISO STS");
        assert_eq!(summary.version, "1.0");
        assert!(summary.file_count > 0);
    }
}
