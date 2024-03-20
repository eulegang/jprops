use jprops::Properties;

#[test]
fn default() {
    let props = Properties::default();

    assert_eq!(props.len(), 0);
    assert_eq!(props.is_empty(), true);
}

#[test]
fn insert() {
    let mut props = Properties::default();

    props.insert("hello".to_string(), "world".to_string());

    assert_eq!(props.len(), 1);
    assert_eq!(props.is_empty(), false);
}

#[test]
fn insert_str() {
    let mut props = Properties::default();

    props.insert_str("hallo", "welt");

    assert_eq!(props.len(), 1);
    assert_eq!(props.get("hallo"), Some("welt"));
    assert_eq!(props.get("hello"), None);
}

#[test]
fn delete() {
    let mut props = Properties::default();

    props.insert_str("hallo", "welt");
    props.insert_str("hallo", "welt");
    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");

    assert_eq!(props.len(), 4);

    props.delete("hallo");

    assert_eq!(props.len(), 1);

    assert_eq!(props.get("hello"), Some("world"));
}

#[test]
fn index() {
    let mut props = Properties::default();

    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");

    assert_eq!(&props["hallo"], "welt");
    assert_eq!(&props["hello"], "world");
}

#[test]
#[should_panic]
fn index_panic() {
    let props = Properties::default();

    assert_eq!(&props["hello"], "world");
}

#[test]
fn get_all() {
    let mut props = Properties::default();

    props.insert_str("hallo", "mutter");
    props.insert_str("hallo", "vater");
    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");

    assert_eq!(props.len(), 4);

    assert_eq!(props.get_all("hallo"), vec!["mutter", "vater", "welt",]);
}

#[test]
fn merge() {
    let mut props = Properties::default();
    props.insert_str("hallo", "welt");

    let mut other = Properties::default();
    other.insert_str("hello", "world");

    props.merge(other);

    assert_eq!(props.len(), 2);
}
