#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: i32,
    active: bool
} 

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1
    };
}

// Tuple structs
struct Color (u32, u32, u32);
struct Point (u8, u8);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size }
    }
}
fn main() {
    let mut user = User {
        username: String::from("demo"),
        email: String::from("demo@demo.com"),
        sign_in_count: 12,
        active: true
    };
    user.email = String::from("another_email@demo.com");
    let mut user2 = build_user(String::from("demo_2@gmail.com"), String::from("demo2"));
    user2.email =  String::from("notebook@demo.com");
    
    let mut user3 = User {
        username: String::from("demo3"),
        email: String::from("another@demo.com"),
        ..user2
    };
    let black = Color(0, 0, 0);
    let origin = Point(20, 21);
    println!("{}", origin.1);
    println!("{}", origin.0);
    let rectangle  = (30, 50);
    println!("the area of the rectangles is {} square pixels", area(rectangle));
    let mut rectangle2 = Rectangle{
        width: 30,
        height: 50
    };
    println!("the area of the rectangles is {} square pixels", area_ref(&rectangle2));
    println!("the rectangle is {:#?}", rectangle2);
    rectangle2.height = 10;
    let mut rectangle3 = Rectangle {
        height: 30,
        width: 50
    };
    let area = rectangle3.area();
    println!("area from struct method: {}", &area);
    let hold = rectangle3.can_hold(&rectangle2);
    println!("can hold: {}", hold);
    let square = Rectangle::square(3);
    println!("square {:?}", &square);
}

// fn area (width: u32, height: u32) -> u32 {
//     width * height
// }
fn area (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area_ref (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}