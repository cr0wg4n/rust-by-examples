fn main() {
    let name = String::from("demoticon");
    println!("{}", name);
    let mut hello = String::from("Hello");
    hello.push_str(" another demo");
    println!("{}", hello);
    {
        let some_string = "demo string";
        println!("{}", some_string);
    }
    let x = 5;
    let y = x;
    println!("{} {}", y, x); // y is a copy, cause x is an integer with a exact amount of bytes (is in the stack)
    let s1 = String::from("demo");
    let s2 = s1;
    // println!("s1: {}, s2: {}", s1, s2); // rust considers s1 to no longer be valid
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3); 
    let mut string = String::from("another example");
    change_string(&mut string);
    println!("{}", string);
}

fn change_string(some_string: &mut String) {
    some_string.push_str(" with referenciation and mutability");
}
