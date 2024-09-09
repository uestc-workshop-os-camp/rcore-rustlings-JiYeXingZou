// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE 
//栈内存  vs  堆内存

//栈 -> 编译期识别类型的大小，默认分配的空间
//堆 -> 运行时动态分配
fn main() {
    let vec0 = Vec::new(); // 动态增长

    let mut vec1 = fill_vec(vec0);  

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
