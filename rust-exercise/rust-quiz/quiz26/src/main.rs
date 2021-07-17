// 这代码在stable-x86_64-pc-windows-gnu上编译不通过，切换到stable-x86_64-pc-windows-msvc能编译过

fn main() {
    let input = vec![1, 2, 3];

    let parity = input
        .iter()
        .map(|x| {
            print!("{}", x);
            x % 2
        });

    for p in parity {
        print!("{}", p);
    }

    // 1 1 2 0 3 1
}