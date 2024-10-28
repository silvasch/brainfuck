const PROGRAM: &str = "+[>,.<]";

fn main() {
    if let Err(e) = brainfuck::run(PROGRAM) {
        eprintln!("{}", e);
    };
}
