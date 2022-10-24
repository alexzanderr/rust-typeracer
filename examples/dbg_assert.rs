fn main() {
    let x = 123;
    // this will be gone at runtime in release mode
    // debug_assert!(x != 123);
    let m = module_path!();
    dbg!("{:?}", m);
}
