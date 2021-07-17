trait Trait {
    fn f(&self);
}

impl Trait for u32 {
    fn f(&self) {
        print!("1");
    }
}

impl<'a> Trait for &'a i32 {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    // let x = &0;
    // x.f();
    // println!("");
    // (&x).f();
    (&0).f();
    <&i32 as Trait>::f(&&0);    // 完全限定语法
    println!("");
    (&&0).f();
}