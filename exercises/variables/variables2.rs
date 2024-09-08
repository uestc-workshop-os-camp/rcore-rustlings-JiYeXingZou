// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

// 

fn main() {
    let x : i32 = 10; //没有初始化,为什么不允许未初始化?出于内存安全的考虑
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
