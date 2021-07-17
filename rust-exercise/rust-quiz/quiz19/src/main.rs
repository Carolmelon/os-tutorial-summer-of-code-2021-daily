struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}
// 21
fn main() {
    let s = S;
    let _ = s;  // 1 不会转移所有权
    print!("2");    // 2
                // 1 
}