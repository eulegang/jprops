use jprops::Properties;

#[test]
fn single_dump() {
    let mut props = Properties::default();

    props.insert_str("hello", "world");

    assert_eq!(props.to_string(), "hello=world\n");
}

#[test]
fn multi_dump() {
    let mut props = Properties::default();

    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");
    assert_eq!(props.to_string(), "hallo=welt\nhello=world\n");
}

#[test]
fn duplicate_dump() {
    let mut props = Properties::default();

    props.insert_str("hallo", "mutter");
    props.insert_str("hallo", "vater");
    props.insert_str("hallo", "welt");
    props.insert_str("hello", "world");
    assert_eq!(
        props.to_string(),
        "hallo=mutter\nhallo=vater\nhallo=welt\nhello=world\n"
    );
}
