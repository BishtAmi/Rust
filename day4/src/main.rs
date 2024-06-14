struct Dimension(i32, i32);
fn main() {
    let dimension = Dimension(20, 40);
    println!("Area is {}", area(&dimension));
}
fn area(dimension: &Dimension) -> i32 {
    dimension.0 * dimension.1
}
