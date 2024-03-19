use jprops::Properties;
use std::borrow::Cow;

#[test]
pub fn basic_eq_assignment() {
    let content = b"name=value";

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(props.get("name"), vec![Cow::from("value")]);
}

#[test]
pub fn basic_colon_assignment() {
    let content = b"name:value";

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(props.get("name"), vec![Cow::from("value")]);
}

#[test]
pub fn blank_lines() {
    let content = b"\nname=value\n\n";

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(props.get("name"), vec![Cow::from("value")]);
}
