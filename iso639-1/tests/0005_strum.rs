extern crate iso639_1;

use iso639_1::Iso639_1;

#[test]
fn strum_iter() {
    let mut iter = Iso639_1::iter();
    assert_eq!(Some(Iso639_1::Aa), iter.next());
    assert_eq!(Some(Iso639_1::Ab), iter.next());
    assert_eq!(Some(Iso639_1::Af), iter.next());
    assert_eq!(Some(Iso639_1::Ak), iter.next());
}
