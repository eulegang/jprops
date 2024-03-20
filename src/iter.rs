//!
//! Iteration utilities/types
//!

use std::borrow::{Borrow, Cow};

use crate::Properties;

/// Iterate by &str in both key and value
pub struct KVIter<'a, 'bytes>(pub(crate) &'a Properties<'bytes>, pub(crate) usize);

/// Iterate by &str in keys
pub struct KIter<'a, 'bytes>(pub(crate) &'a Properties<'bytes>, pub(crate) usize);

/// Iterate by taking ownership of of key and value
pub struct IntoIter<'bytes>(std::vec::IntoIter<(Cow<'bytes, str>, Cow<'bytes, str>)>);

/// Iterate over references
pub struct Iter<'a, 'bytes>(std::slice::Iter<'a, (Cow<'bytes, str>, Cow<'bytes, str>)>);

/// Iterate over mut references
pub struct IterMut<'a, 'bytes>(std::slice::IterMut<'a, (Cow<'bytes, str>, Cow<'bytes, str>)>);

impl<'a, 'bytes> Iterator for KVIter<'a, 'bytes>
where
    'a: 'bytes,
{
    type Item = (&'bytes str, &'bytes str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.len() {
            return None;
        }

        let idx = self.1;
        self.1 += 1;

        let (k, v) = &self.0.pairs[idx];

        Some((k.borrow(), v.borrow()))
    }
}

impl<'a, 'bytes> Iterator for KIter<'a, 'bytes>
where
    'a: 'bytes,
{
    type Item = &'bytes str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.len() {
            return None;
        }

        let idx = self.1;
        self.1 += 1;

        let (k, _) = &self.0.pairs[idx];

        Some(k.borrow())
    }
}

impl<'bytes> Iterator for IntoIter<'bytes> {
    type Item = (Cow<'bytes, str>, Cow<'bytes, str>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, 'bytes> Iterator for Iter<'a, 'bytes> {
    type Item = &'a (Cow<'bytes, str>, Cow<'bytes, str>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, 'bytes> Iterator for IterMut<'a, 'bytes> {
    type Item = &'a mut (Cow<'bytes, str>, Cow<'bytes, str>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'bytes> IntoIterator for Properties<'bytes> {
    type Item = (Cow<'bytes, str>, Cow<'bytes, str>);

    type IntoIter = IntoIter<'bytes>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.pairs.into_iter())
    }
}

impl<'a, 'bytes> IntoIterator for &'a Properties<'bytes> {
    type Item = &'a (Cow<'bytes, str>, Cow<'bytes, str>);

    type IntoIter = Iter<'a, 'bytes>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.pairs.iter())
    }
}

impl<'a, 'bytes> IntoIterator for &'a mut Properties<'bytes> {
    type Item = &'a mut (Cow<'bytes, str>, Cow<'bytes, str>);

    type IntoIter = IterMut<'a, 'bytes>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.pairs.iter_mut())
    }
}

impl<'bytes> FromIterator<(String, String)> for Properties<'bytes> {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut props = Properties::default();

        for (k, v) in iter {
            props.insert(k, v);
        }

        props
    }
}

impl<'bytes> FromIterator<(&'bytes str, &'bytes str)> for Properties<'bytes> {
    fn from_iter<T: IntoIterator<Item = (&'bytes str, &'bytes str)>>(iter: T) -> Self {
        let mut props = Properties::default();

        for (k, v) in iter {
            props.insert_str(k, v);
        }

        props
    }
}
