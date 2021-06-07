fn main() {
    let code = match just::run() {
        Err(it) => it,
        _ => return,
    };
    std::process::exit(code);
}
