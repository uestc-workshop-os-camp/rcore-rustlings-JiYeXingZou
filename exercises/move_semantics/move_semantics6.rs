// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

//

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
/*
get_char 函数：

这个函数的目的是获取 data 的最后一个字符，但是不需要对 data 进行修改，也不需要拿走 data 的所有权。
因此，这里用的是 借用（borrow），即通过 &data 来传递引用，而不是直接传递 data 的所有权。
这符合 Rust 的所有权规则，使得调用这个函数之后，data 依然属于调用它的代码块，可以继续被使用。
string_uppercase 函数：

这个函数的目的是将 data 中的字符串全部转为大写，并且函数内部对 data 进行修改（通过 to_uppercase）。
因为这个函数需要修改 data，并且这里并不希望继续使用修改后的 data，所以需要 获取所有权。
当 data 被传递给 string_uppercase 后，data 的所有权会从调用方转移到 string_uppercase 函数内部，因此调用 string_uppercase 后，main 中的 data 将不再有效（即 Rust 中的移动语义）。
*/