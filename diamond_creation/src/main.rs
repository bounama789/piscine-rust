use diamond_creation::*;

fn main() {
    println!("{:#?}", get_diamond('B'));
    println!("{:#?}", get_diamond('K'));
    for line in get_diamond('C') {
        println!("{}", line);
    }
}