use brainfuck::brain_fuck;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    brain_fuck(&args[1]);
}
