//!
//! Properties file manipulation vaguely following the [Spec](https://docs.oracle.com/cd/E23095_01/Platform.93/ATGProgGuide/html/s0204propertiesfileformat01.html)
//! in rust.
//!

#![deny(missing_docs)]

use std::borrow::{Borrow, Cow};

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
}

/// The abstract notion of a properties file
///
/// This uses a naive implementation but should be performant enough for most cases
pub struct Properties<'bytes> {
    pairs: Vec<(Cow<'bytes, str>, Cow<'bytes, str>)>,
}

impl<'bytes> Properties<'bytes> {
    /// Attempts parsing properties content
    pub fn load(mut content: &'bytes [u8]) -> Result<Self, Error> {
        let mut pairs = Vec::new();
        let mut line = 0;

        while !content.is_empty() {
            let cur = match memchr::memchr2(b'\n', b'\r', content) {
                Some(br) => {
                    let (cur, next) = content.split_at(br);
                    content = &next[1..];
                    cur
                }

                None => {
                    let next = content;
                    content = &content[content.len()..];
                    next
                }
            };

            line += 1;

            if cur.is_empty() {
                continue;
            }

            let Some(assign) = memchr::memchr2(b'=', b':', cur) else {
                let s = std::str::from_utf8(cur).map_err(|e| Error::InvalidUtf8(line, e))?;

                return Err(Error::MalformedLine(line, s.to_string()));
            };

            let (pre, post) = cur.split_at(assign);

            let key = std::str::from_utf8(pre).map_err(|e| Error::InvalidUtf8(line, e))?;
            let value = std::str::from_utf8(&post[1..]).map_err(|e| Error::InvalidUtf8(line, e))?;

            pairs.push((Cow::Borrowed(key), Cow::Borrowed(value)));
        }

        Ok(Properties { pairs })
    }

    /// returns how many pairs
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Checks if there are properties
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Get all values for a specific key
    pub fn get<'container>(&'container self, key: &str) -> Vec<&'bytes str>
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
