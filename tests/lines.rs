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

#[test]
pub fn comments() {
    let content = b"# Hello!\nname=value\n\n";

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(props.get("name"), vec![Cow::from("value")]);
}

#[test]
pub fn bang_comments() {
    let content = b"! World!\nname=value\n\n";

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(props.get("name"), vec![Cow::from("value")]);
}

#[test]
pub fn whitespace() {
    let content = b"  name  \t  =    value  \t  ";

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(props.get("name"), vec![Cow::from("value")]);
}

#[test]
pub fn backspace_continue() {
    let content = b"targetCities=\\\nDetroit,\\\n\t   Chicago,\\\n  Los Angeles\n";
    // why does oracle want to target cities?

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(
        props.get("targetCities"),
        vec![Cow::from("Detroit,Chicago,Los Angeles")]
    );
}

#[test]
pub fn escaping() {
    let content = b"name=hello\\tworld\\r\\nhow are you?";

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(
        props.get("name"),
        vec![Cow::from("hello\tworld\r\nhow are you?")]
    );
}

#[test]
pub fn unicode() {
    let content = b"name=hello\\u002c world";

    let props = Properties::load(content).unwrap();

    assert_eq!(props.len(), 1);

    assert_eq!(props.get("name"), vec![Cow::from("hello, world")]);
}
