struct S;

fn main() {
    let [x, y] = &mut [S, S];
    let eq = x as *mut S == y as *mut S;
    print!("{}", eq as u8);

    println!("");
    println!("{}", std::mem::size_of::<[S; 2]>());   // 打印大小 √
    println!("{}", std::mem::size_of::<&str>());
    println!("{}", std::mem::size_of::<&String>());
    println!("{}", std::mem::size_of::<&i32>());
    println!("{}", std::mem::size_of::<&[i32]>());
    println!("{}", std::mem::size_of::<&[&str]>());
}