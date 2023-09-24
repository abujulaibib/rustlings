// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let result = call_me(3);
    println!("{}", result)
}

fn call_me(x: u32) -> u32 {
    x * 3
}
