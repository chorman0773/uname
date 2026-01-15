fn main() {
    let x = sysname::uname().unwrap();

    println!("{x:?}");
}
