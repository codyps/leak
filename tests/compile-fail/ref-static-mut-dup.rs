fn main() {
    fn dupe(data: &'static mut u8) -> (&'static mut u8, &'static mut u8) {
            (data, data) //~ ERROR cannot borrow
    }

    let _ = dupe;
}
