fn main() {
    let num = 3;
    println!("The square of {} is {}", num, square(num));
}

fn square(n: i32) -> i32 {
    return n * n;
}