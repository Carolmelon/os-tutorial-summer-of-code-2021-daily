#![allow(unused_parens)]

trait Trait {
    fn f(&self);
}

impl<F: FnOnce() -> bool> Trait for F {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for () {
    fn f(&self) {
        print!("2");
    }
}


fn main() {
    
    let x = || { (return) || true; };   // 2
    x().f();

    let x = loop { (break) || true; };  // 2
    x.f();

    let x = || { return (|| true); };   // 1
    x().f();

    let x = loop { break (|| true); };  // 2(x) 1
    x.f();

    let x = || { return || true; };     // 2(x) 1
    x().f();

    let x = loop { break || true; };    // 2(x) 1
    x.f();
}
