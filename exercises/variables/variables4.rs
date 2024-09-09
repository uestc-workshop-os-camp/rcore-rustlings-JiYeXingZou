// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

//


//x是不可变变量
//思考:为什么:1,为什么不允许二次赋值 2.为什么不允许你修改第11行,如何修改第11行可以编译通过
//继承式可变 和 普通的mut可变 有什么区别
fn main() {
    let mut x = 3; //默认不可变 mut x是一个可变的绑定
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}


//继承式可变




//内部可变性
