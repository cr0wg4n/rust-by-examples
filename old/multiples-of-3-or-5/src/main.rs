// ProjectEuler.net problem: https://projecteuler.net/problem=1
fn main() {
    let n: i32 = 1000;
    let mut sum: i32 = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }
    println!("the sum is: {}", sum);
}
