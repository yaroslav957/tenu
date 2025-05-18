fn main() {
    let args = std::env::args();

    for (n, arg) in args.enumerate() {
        println!("[{n}] {arg}");
    }
}
