extern crate iso639_2;
use iso639_2::Iso639_2;

#[test]
fn enum_0001_equal() {
    assert!(Iso639_2::Fra == Iso639_2::Fra);
}

#[test]
fn enum_0002_not_equal() {
    assert!(Iso639_2::Fra != Iso639_2::Eng);
}

#[test]
fn enum_0003_no_panic() {
    println!("{:?}", Iso639_2::Eng)
}
