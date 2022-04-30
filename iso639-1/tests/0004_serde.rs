extern crate iso639_1;
extern crate serde;

use iso639_1::Iso639_1;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    lang: Iso639_1,
}

#[test]
fn serialize() {
    let foo = Foo { lang: Iso639_1::Fr };
    let s = serde_json::to_string(&foo).expect("fail to serialize");
    assert_eq!(s, r#"{"lang":"fr"}"#);
}

#[test]
fn deserialize() {
    let s = r#"{"lang":"fr"}"#;
    let foo: Foo = serde_json::from_str(s).expect("fail to deserialize");
    assert_eq!(foo.lang, Iso639_1::Fr);
}
