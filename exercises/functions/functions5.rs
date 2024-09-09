// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// 

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
//分号 后面如果没有东西， 返回的是一个(),所以需要移除这个分号
//分号有什么意义?分号语法的意义
