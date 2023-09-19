//! A wrapper around PathBuf that represents valid URIs 
//!
//! This is used to index pages and sections in j3sg--
//! values in the global `SECTION_MAP` and `PAGE_MAP` objects
//! are indexed by their corresponding Uri.
//!
//! In addition, a URI also uniquely determines an output file path,
//! given a base output directory, given by.
//!
//! ```
//! out_dir + uri + "index.html"
//! ```
//!
//! where `+` denotes joining paths together

use colored::*;
use std::path::{Path, PathBuf, Component};
use std::hash::{Hash, Hasher};
use std::fmt;
use serde::{Serialize, Serializer};

/// A valid j3sg URI to a page or section. 
///
/// Valid j3sg URIs consist of nonempty segments which may only consist of
///
/// - Alphanumeric characters
/// - Hyphens, underscores, and dots
///
/// Furthermore, segments may never be ".." or "." 
///
/// Internally, a Uri is a non-absolute PathBuf. 
/// (Because it's easier to make a non-absolute path absolute than the other way around)
#[derive(Clone)]
pub struct Uri(PathBuf);
impl Uri {
    /// Helper function which validates if a path would make a valid internal
    /// PathBuf for a Uri
    ///
    /// Specifically, it follows the rules:
    /// * It must be non-absolute
    /// * It must not contain current or parent directory components,
    /// i.e no path segments of the form ".." or "."
    /// * Path segments must use only the following ASCII characters
    ///     * Alphanumeric characters
    ///     * Hyphens, underscores, or periods
    ///
    /// # Arguments
    ///
    /// * `path` - The path to validate
    fn valid_path<P>(path: P) -> bool 
    where
        P: AsRef<Path>
    {
        path.as_ref().components()
            .all(|component| match component {
                Component::RootDir
                | Component::CurDir
                | Component::ParentDir 
                | Component::Prefix(_)
                => false,
                Component::Normal(osstr) => {
                    let Some(s) = osstr.to_str() else { return false };
                    s.chars().all(|ch| ch.is_alphanumeric()
                                  || ch == '-' 
                                  || ch == '_'
                                  || ch == '.')
                }
            })
    }

    /// Creates a new root URI, `/`
    pub fn new() -> Self {
        Self(PathBuf::new())
    }

    /// Generates the path to the output directory which would
    /// contain the .html file this Uri serves
    ///
    /// TODO: fix this unfortunate naming
    ///
    /// # Arguments
    /// * `out_dir` - The path of the output directory
    pub fn out_dir<P>(&self, out_dir: P) -> PathBuf 
    where 
        P: AsRef<Path>
    {
        out_dir.as_ref()
            .join(&self.0)
    }

    /// Generates the path to the output .html file for the given
    /// page provided a specific output directory
    /// 
    /// # Arguments
    /// 
    /// * `out_dir` - The path of the output directory
    pub fn out_path<P>(&self, out_dir: P) -> PathBuf 
    where 
        P: AsRef<Path>
    {
        out_dir.as_ref()
            .join(&self.0)
            .join("index.html")
    }

    /// Returns a new Uri to the parent of this page if it exists
    pub fn parent(&self) -> Option<Uri> {
        self.0.parent().map(|path| Uri(path.to_owned()))
    }
    
    /// Returns a vector of ancestors
    pub fn ancestors(&self) -> Vec<Self> {
        todo!();
    }

    /// Returns a new Uri joined with the given path 
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be joined to this Uri
    pub fn join<P>(&self, path: P) -> Result<Uri, String> 
    where
        P: AsRef<Path>
    {
        let path = path.as_ref();
        if Self::valid_path(path) {
            Ok(Uri(self.0.join(path)))
        } else {
            Err(format!("Tried to join invalid path {:?}", path))
        }
    }

    /// Whether or not this Uri is the root uri, `/`
    pub fn is_root(&self) -> bool {
        let path = self.0.as_path();
        path.parent() == None
    }

    /// Returns the last segment of this Uri
    pub fn file_name(&self) -> String {
        if self.is_root() {
            String::new()
        } else {
            self.0.file_name().unwrap()
                .to_str().unwrap()
                .to_string()
        }
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_root() {
            write!(f, "/")?;
        } else {
            let path = self.0.as_path();
            for segment in path {
                write!(f, "/{}", segment.to_str().unwrap().purple())?;
            }
        }
        Ok(())
    }
}
impl fmt::Debug for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string().purple())
    }
}
impl PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for Uri {}
impl Hash for Uri {
    fn hash<H>(&self, state: &mut H) 
    where 
        H: Hasher
    {
        self.to_string().hash(state);
    }
}
impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where 
        S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn create_new_root_uri() {
        let uri = Uri::new();
        assert_eq!(uri, Uri(PathBuf::new()));
    }

    #[test]
    fn push_segment_to_uri() {
        let uri = Uri::new();
        assert_eq!(uri, Uri(PathBuf::new()));
    }

    #[test]
    fn create_valid_pathbuf() {
        let mut buf = PathBuf::new();
        buf.push("foo");
        buf.push("bar");
        buf.push("baz");

        assert!(Uri::valid_path(buf));
    }

    #[test]
    fn invalid_pathbuf_absolute() {
        let mut buf = PathBuf::new();
        buf.push("/");
        buf.push("absolute");
        buf.push("path");

        assert!(!Uri::valid_path(buf));
    }
    
    #[test]
    fn invalid_pathbuf_non_alphanumeric() {
        let mut buf = PathBuf::new();
        buf.push("@@@");
        buf.push("$$$");
        buf.push("###");

        assert!(!Uri::valid_path(buf));
    }

    #[test]
    fn invalid_pathbuf_parent() {
        let mut buf = PathBuf::new();
        buf.push("foo");
        buf.push("..");
        buf.push("bar");

        assert!(!Uri::valid_path(buf));
    }
}
