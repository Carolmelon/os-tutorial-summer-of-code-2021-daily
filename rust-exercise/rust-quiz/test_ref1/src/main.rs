
use std::ops::Deref;

struct X(i32);

trait M { 
    fn m(&self);
}

impl Deref for X {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

// impl M for X {
//     fn m(&self){
//         println!("X::m()");
//     }
// }

// impl M for &X {
//     fn m(&self){
//         println!("&X::m()");
//     }
// }

impl M for i32 {
    fn m(&self){
        println!("i32::m()");
    }
}

impl M for &i32 {
    fn m(&self){
        println!("&i32::m()");
    }
}

// 实现trait语法，impl traitName as typeName
// 完全限定语法，<typeName as traitName>::methodName(..)

// 先加*，再加&
fn main() {
    (X(0)).m(); // X(0) --> *X(0) -> i32 --> &i32
    (&X(0)).m(); // &X(0) -> &i32
    <i32 as M>::m(&X(0)); // 可以验证上述注释
    
    (*(X(0))).m();  // *X(0) -> i32 --> &i32
    (&(*X(0))).m(); // &*X(0) -> &i32
    <i32 as M>::m(&(*X(0)));

    (&&(*X(0))).m(); // &&*X(0) -> &&i32 --> *&&i32 -> &i32
    <&i32 as M>::m(&&(*X(0)));
}
