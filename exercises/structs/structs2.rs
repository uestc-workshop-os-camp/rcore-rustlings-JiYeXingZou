// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.

//

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!
        let your_order = Order{
            name: String::from("Hacker in Rust"),
            year: 2019,
            made_by_phone: false,
            made_by_mobile: false,
            made_by_email: true,
            item_number: 123,
            count: 1,
        };
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
/*
String 与字符串字面量的区别：

字符串字面量（例如 "Hacker in Rust"）是一个不可变的、固定大小的字符串，它的类型是 &str。这种字符串存储在程序的只读内存中，无法动态改变长度。
String 是 Rust 中的可变字符串类型，存储在堆上，它可以在运行时动态地调整大小。String 是一个拥有字符串数据的类型，具有更多的功能和灵活性，例如可以扩展、修改等。
为什么要使用 String::from()：

结构体 Order 中的 name 字段类型是 String，而字面量 "Hacker in Rust" 是 &str。为了将 &str 转换为 String 类型，使用了 String::from()。
String::from() 方法的作用是将 &str 转换为堆上分配的 String，从而使得 name 字段可以保存并拥有这个字符串。
直接使用字符串字面量的问题：

如果直接将 "Hacker in Rust" 赋给 name，会出现类型不匹配错误，因为 "Hacker in Rust" 的类型是 &str，而 name 需要的是 String。
*/