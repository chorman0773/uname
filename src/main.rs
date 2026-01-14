fn main() {
    let x = uname::uname().unwrap();

    println!("{x:?}");
}
