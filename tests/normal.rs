extern crate leak;
use leak::Leak;

#[test]
fn leak_str() {
    use std::borrow::ToOwned;

    let v = "hi";
    let leaked : &str = {
        let o = v.to_owned();
        o.leak()
    };
    assert_eq!(leaked, v);

    let leaked : &'static str = {
        let o = v.to_owned();
        o.leak()
    };
    assert_eq!(leaked, v);
}

#[test]
fn leak_empty_str() {
    use std::borrow::ToOwned;

    let v = "";
    let leaked : &'static str = {
        let o = v.to_owned();
        o.leak()
    };
    assert_eq!(leaked, v);
}

#[test]
fn leak_vec() {
    let v = vec![3, 5];
    let leaked : &'static [u8] = {
        let o = v.clone();
        o.leak()
    };
    assert_eq!(leaked, &*v);
}

#[test]
fn leak_empty_vec() {
    let v = vec![];
    let leaked : &'static [u8] = {
        let o = v.clone();
        o.leak()
    };
    assert_eq!(leaked, &*v);
}

#[test]
fn leak_box() {
    let v : Box<[&str]> = vec!["hi", "there"].into_boxed_slice();
    let leaked : &'static [&str] = {
        let o = v.clone();
        o.leak()
    };
    assert_eq!(leaked, &*v);
}

#[test]
fn leak_nested() {
    let v : Box<Vec<&str>> = Box::new(vec!["hi", "there"]);
    let leaked : &'static [&str] = {
        let o = v.clone();
        o.leak()
    };
    assert_eq!(leaked, &**v);
}

#[test]
fn leak_mut() {
    let v = vec![1, 3, 4];
    let leaked: &'static mut [u8] = v.leak();
    leaked[1] = 5;
    assert_eq!(leaked, &[1,5,4])
}
