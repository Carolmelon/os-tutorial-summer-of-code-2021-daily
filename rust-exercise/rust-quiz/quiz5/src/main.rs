trait Trait {
    fn p(self);
}

impl<T> Trait for fn(T) {
    fn p(self) {
        print!("1");
    }
}

impl<T> Trait for fn(&T) {
    fn p(self) {
        print!("2");
    }
}

fn f(_: u8) {}
fn g(_: &u8) {}

fn main() {
    let a: fn(_) = f;   // a: fn(u8)
    let b: fn(_) = g;   // b: fn(&u8)
    let c: fn(&_) = g;  // c: for<'r> fn(&'r u8) 什么玩意儿？Higher-Rank Trait Bounds (HRTBs)
    a.p();
    b.p();
    c.p();

    // let x = (return) || true;    // x推断为bool类型
    // let y = return || true;
    // println!("{}", x);
}
