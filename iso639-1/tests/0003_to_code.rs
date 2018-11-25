extern crate iso639_1;
use iso639_1::{Iso639_1, get_code_iso639_3};

#[test]
fn get_iso639v3_0001_match() {
    assert!(get_code_iso639_3("fr") == "fra");
}

#[test]
#[should_panic]
fn get_iso639v3_0002_not_match() {
    assert!(get_code_iso639_3("xxxx") == "fra");
}
