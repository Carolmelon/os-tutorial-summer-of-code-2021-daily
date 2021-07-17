use std::ops::RangeFull;

trait Trait {
    fn method(&self) -> fn();
}

impl Trait for RangeFull {
    fn method(&self) -> fn() {
        print!("1");
        || print!("3")
    }
}

impl<F: FnOnce() -> T, T> Trait for F {
    fn method(&self) -> fn() {
        print!("2");
        || print!("4")
    }
}

// trait Fn 的父trait是 trait FnMut, trait FnMut 的父trait是 trait FnOnce,不能重复实现
// impl<F: Fn() -> T, T> Trait for F {
//     fn method(&self) -> fn() {
//         print!("5");
//         || print!("6")
//     }
// }

fn main() {
    (|| .. .method())();    // 结合优先级，..先跟||结合
    println!("");
    (|| RangeFull .method())();  // 结合优先级，RangeFull先跟后面的.结合
    println!("");
    (|| (..) .method())();  // 和上面等价
    assert_eq!(.., RangeFull);
}
