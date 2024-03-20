use jprops::Properties;
use std::borrow::Cow;

#[test]
fn iter_take() {
    let mut props = Properties::default();
    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");

    let mut it = props.into_iter();

    assert_eq!(
        it.next(),
        Some((Cow::Borrowed("hallo"), Cow::Borrowed("welt")))
    );
    assert_eq!(
        it.next(),
        Some((Cow::Borrowed("hello"), Cow::Borrowed("world")))
    );
    assert_eq!(it.next(), None);
}

#[test]
fn iter_ref() {
    let mut props = Properties::default();
    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");

    let mut it = (&props).into_iter();

    assert_eq!(
        it.next(),
        Some(&(Cow::Borrowed("hallo"), Cow::Borrowed("welt")))
    );
    assert_eq!(
        it.next(),
        Some(&(Cow::Borrowed("hello"), Cow::Borrowed("world")))
    );
    assert_eq!(it.next(), None);
}

#[test]
fn iter_mut() {
    let mut props = Properties::default();
    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");

    let mut it = (&mut props).into_iter();

    assert_eq!(
        it.next(),
        Some(&mut (Cow::Borrowed("hallo"), Cow::Borrowed("welt")))
    );
    assert_eq!(
        it.next(),
        Some(&mut (Cow::Borrowed("hello"), Cow::Borrowed("world")))
    );
    assert_eq!(it.next(), None);
}

#[test]
fn key_values() {
    let mut props = Properties::default();
    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");

    let mut it = props.key_values();

    assert_eq!(it.next(), Some(("hallo", "welt")));
    assert_eq!(it.next(), Some(("hello", "world")));
    assert_eq!(it.next(), None);
}

#[test]
fn keys() {
    let mut props = Properties::default();
    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");

    let mut it = props.keys();

    assert_eq!(it.next(), Some("hallo"));
    assert_eq!(it.next(), Some("hello"));
    assert_eq!(it.next(), None);
}
