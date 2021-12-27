
fn main() {
    let max: i32 = 4_000_000;
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut sum: i32 = 0;
    while a < max {
        println!("fibonacci number: {}", a);
        if a % 2 == 0 {
            sum += a;
        }
        let c = a + b;
        a = b;
        b = c;
    }
    println!("the sum is: {}", sum);
}