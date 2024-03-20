//!
//! Properties file manipulation vaguely following the [Spec](https://docs.oracle.com/cd/E23095_01/Platform.93/ATGProgGuide/html/s0204propertiesfileformat01.html)
//! in rust.
//!

#![deny(missing_docs)]

use std::borrow::{Borrow, Cow};

mod load;

/// A type representing a properties parse error
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// A line did not have proper formatting
    #[error("line {0} is malformed \"{1}\"")]
    MalformedLine(usize, String),

    /// A line found does not contain proper utf-8
    #[error("line {0} is not proper utf-8 \"{1}\"")]
    InvalidUtf8(usize, std::str::Utf8Error),

    /// Invalid escape sequence used
    #[error("line {0} has invalid escape sequence \"{1}\"")]
    InvalidEscape(usize, String),
}

/// The abstract notion of a properties file
///
/// This uses a naive implementation but should be performant enough for most cases
#[derive(Debug, Default)]
pub struct Properties<'bytes> {
    pairs: Vec<(Cow<'bytes, str>, Cow<'bytes, str>)>,
}

impl<'bytes> Properties<'bytes> {
    /// Attempts parsing properties content
    pub fn load(content: &'bytes [u8]) -> Result<Self, Error> {
        load::load(content)
    }

    /// returns how many pairs
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Checks if there are properties
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Get the value for the first first key encountered that matches the key
    pub fn get<'container>(&'container self, key: &str) -> Option<&'bytes str>
    where
        'container: 'bytes,
    {
        for (k, v) in &self.pairs {
            if k == key {
                return Some(v.borrow());
            }
        }

        None
    }

    /// Get all values for a specific key
    pub fn get_all<'container>(&'container self, key: &str) -> Vec<&'bytes str>
    where
        'container: 'bytes,
    {
        let mut res = Vec::new();

        for (k, v) in &self.pairs {
            if k == key {
                res.push(v.borrow());
            }
        }

        res
    }
}
