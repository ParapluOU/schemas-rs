//! Core traits and types for XML schema bundles.
//!
//! This crate provides the [`SchemaBundle`] trait that all schema crates implement,
//! allowing uniform access to statically embedded schema files.

use std::path::Path;

/// Error types for schema operations.
#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    /// The requested file was not found in the bundle.
    #[error("schema file not found: {0}")]
    FileNotFound(String),

    /// Failed to write a file.
    #[error("failed to write file {path}: {source}")]
    WriteError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// Failed to create a directory.
    #[error("failed to create directory {path}: {source}")]
    CreateDirError {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

/// A single schema file with its relative path and content.
#[derive(Debug, Clone, Copy)]
pub struct SchemaFile {
    /// Relative path within the schema bundle (e.g., "xsd1.2/base/xsd/basemap.xsd")
    pub path: &'static str,
    /// Raw file content as bytes
    pub content: &'static [u8],
}

impl SchemaFile {
    /// Create a new schema file entry.
    pub const fn new(path: &'static str, content: &'static [u8]) -> Self {
        Self { path, content }
    }

    /// Get the content as a UTF-8 string (if valid).
    pub fn content_str(&self) -> Result<&'static str, std::str::Utf8Error> {
        std::str::from_utf8(self.content)
    }

    /// Get the file extension (e.g., "xsd", "dtd").
    pub fn extension(&self) -> Option<&str> {
        Path::new(self.path).extension().and_then(|e| e.to_str())
    }

    /// Get just the file name without directory.
    pub fn file_name(&self) -> Option<&str> {
        Path::new(self.path).file_name().and_then(|n| n.to_str())
    }
}

/// A bundle of schema files that can be accessed and extracted.
///
/// All schema crates implement this trait to provide access to their
/// statically embedded schema files.
pub trait SchemaBundle {
    /// Human-readable name of the schema (e.g., "DITA 1.2", "NISO STS 1.0")
    const NAME: &'static str;

    /// Version string of the schema
    const VERSION: &'static str;

    /// License identifier (e.g., "OASIS-IPR", "Apache-2.0")
    const LICENSE: &'static str;

    /// Get all files in this bundle.
    fn files() -> &'static [SchemaFile];

    /// Get the total number of files in the bundle.
    fn file_count() -> usize {
        Self::files().len()
    }

    /// Find a file by its exact relative path.
    fn get_file(path: &str) -> Option<&'static SchemaFile> {
        Self::files().iter().find(|f| f.path == path)
    }

    /// Find all files matching a predicate.
    fn find_files<F>(predicate: F) -> impl Iterator<Item = &'static SchemaFile>
    where
        F: Fn(&SchemaFile) -> bool,
    {
        Self::files().iter().filter(move |f| predicate(f))
    }

    /// Find all files with a specific extension (e.g., "xsd").
    fn files_by_extension(ext: &str) -> impl Iterator<Item = &'static SchemaFile> {
        Self::files()
            .iter()
            .filter(move |f| f.extension() == Some(ext))
    }

    /// List all file paths in the bundle.
    fn list_paths() -> impl Iterator<Item = &'static str> {
        Self::files().iter().map(|f| f.path)
    }

    /// Write all schema files to the given base directory.
    ///
    /// This creates the directory structure and writes all files,
    /// preserving the relative paths from the bundle.
    fn write_to_directory(base_path: &Path) -> Result<usize, SchemaError> {
        let mut count = 0;

        for file in Self::files() {
            let full_path = base_path.join(file.path);

            // Create parent directories
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| SchemaError::CreateDirError {
                    path: parent.display().to_string(),
                    source: e,
                })?;
            }

            // Write the file
            std::fs::write(&full_path, file.content).map_err(|e| SchemaError::WriteError {
                path: full_path.display().to_string(),
                source: e,
            })?;

            count += 1;
        }

        Ok(count)
    }

    /// Calculate total size in bytes of all schema files.
    fn total_size() -> usize {
        Self::files().iter().map(|f| f.content.len()).sum()
    }
}

/// Extension trait for iterating over multiple schema bundles.
pub trait SchemaBundleExt: SchemaBundle {
    /// Get a summary of this bundle.
    fn summary() -> BundleSummary {
        BundleSummary {
            name: Self::NAME,
            version: Self::VERSION,
            license: Self::LICENSE,
            file_count: Self::file_count(),
            total_size: Self::total_size(),
        }
    }
}

impl<T: SchemaBundle> SchemaBundleExt for T {}

/// Summary information about a schema bundle.
#[derive(Debug, Clone)]
pub struct BundleSummary {
    pub name: &'static str,
    pub version: &'static str,
    pub license: &'static str,
    pub file_count: usize,
    pub total_size: usize,
}

impl std::fmt::Display for BundleSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} v{} ({}) - {} files, {} bytes",
            self.name, self.version, self.license, self.file_count, self.total_size
        )
    }
}
