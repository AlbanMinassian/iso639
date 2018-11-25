extern crate iso639_1;
use iso639_1::{Iso639_1, get_enum};

#[test]
fn to_enum_0001_2chars() {
    assert!(get_enum("fr") == Iso639_1::Fr);
}

#[test]
#[should_panic]
fn to_enum_0002_notexist() {
    assert!(get_enum("XXXX") == Iso639_1::Fr;);
}
