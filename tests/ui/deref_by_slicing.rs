// run-rustfix

#![warn(clippy::deref_by_slicing)]

fn main() {
    let mut vec = vec![0];
    let _ = &vec[..];
    let _ = &mut vec[..];

    let ref_vec = &mut vec;
    let _ = &ref_vec[..];
    let _ = &mut ref_vec[..];

    let s = String::new();
    let _ = &s[..];
}
