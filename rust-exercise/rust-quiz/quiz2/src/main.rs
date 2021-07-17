struct S(i32);

impl std::ops::BitAnd<S> for () {
    type Output = ();

    fn bitand(self, rhs: S) {
        print!("{}", rhs.0);
    }
}

fn main() {
    let f = || (() & S(1));
    let g = || () & S(2);
    let h = || ({} & S(3));
    let i = || {
        {}
        &S(4);
    };
    f(); //  1
    g(); //  2...
    h();
    i();
    // () & S(4);
}
