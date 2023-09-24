// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // by default Rust reserves i32 for integer varibles not explicitly indicated, hence let x = 4 will compile too
    let x: i32 = 4;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
