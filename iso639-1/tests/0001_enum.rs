extern crate iso639_1;
use iso639_1::Iso639_1;

#[test]
fn enum_0001_equal() {
    assert!(Iso639_1::Fr == Iso639_1::Fr);
}

#[test]
fn enum_0002_not_equal() {
    assert!(Iso639_1::Fr != Iso639_1::En);
}

#[test]
fn enum_0003_no_panic() {
    println!("{:?}", Iso639_1::En)
}
