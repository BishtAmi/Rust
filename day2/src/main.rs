use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error in input");
    let mut arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid numbers"))
        .collect();

    println!("array elements are x 2: ");
    multipleBY2(&mut arr);
    for x in arr {
        println!("{x} ");
    }
}

fn multipleBY2(arr: &mut Vec<i32>) {
    for x in arr {
        *x *= 2;
    }
}
