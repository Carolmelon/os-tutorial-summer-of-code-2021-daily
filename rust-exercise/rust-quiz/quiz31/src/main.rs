trait Or {
    fn f(self);
}

struct T;

impl Or for &T {
    fn f(self) {
        print!("1");
    }
}

impl Or for &&&&T {
    fn f(self) {
        print!("2");
    }
}

fn main() {
    let t = T;
    let wt = &T;
    let wwt = &&T;
    let wwwt = &&&T;
    let wwwwt = &&&&T;
    let wwwwwt = &&&&&T;
    t.f();      // 1    &t
    wt.f();     // 1    wt
    wwt.f();    // 1    *wwt
    wwwt.f();   // 2    &wwwt
    wwwwt.f();  // 2    wwwwt
    wwwwwt.f(); // 2    *wwwwwt
}