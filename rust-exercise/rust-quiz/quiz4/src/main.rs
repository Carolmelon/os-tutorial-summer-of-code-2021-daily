fn main() {
    let (.., x, y) = (0, 1, ..);
    let a = b"066";
    println!("x: {:?}, y: {:?}", x, y);
    println!("{} {} {}", a[0], a[1], a[2]);
    print!("{}", a[y][x]);
}