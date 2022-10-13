#[derive(Clone, Copy, Debug)]
enum IpKind {
    V4,
    V6
}

impl IpKind {
    fn demo(&self){
        println!("demo");
    }
}

#[derive(Debug)]
struct IpAddress {
    kind: IpKind,
    address: String
}

enum IpAddressV2{
    V6(String),
    V4(String)
}

enum IpAddressV3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn route(route: IpKind) -> u32{
    route.demo();
    10
}


// for match operator
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska
}


fn main(){ 
    let ip_v4 = IpKind::V4;
    let ip_v6 = IpKind::V6;
    route(ip_v4);
    route(ip_v6);
    ip_v6.demo();

    let address = IpAddress{
        kind: ip_v4,
        address: String::from("::1")
    };

    println!("{:?}", address);

    let address_2 = IpAddressV2::V6(String::from("bighead"));
    let address_2 = IpAddressV2::V4(String::from("demo duro"));
    let address_3 = IpAddressV3::V4(127,0,0,1);
    let address_3 = IpAddressV3::V6(String::from("::1"));
    

    // Option Enum

    let some_number = Some(12.0);
    let some_string: Option<String> = Some(String::from("demo"));
    let absent_number: Option<i32> = None;
    let absent_string: Option<String>= None;

    let coin = Coin::Penny;
    let quarter_coin = Coin::Quarter(UsState::Alaska);

    let value = value_in_cents(&coin);
    let value_2 = value_in_cents(&coin);
    let quarter = value_in_cents(&quarter_coin);


        
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    print!("{}", count);
}

fn value_in_cents(coin: &Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("penny");
            1
        },
        Coin::Nickel => {
            return 5;
        },
        Coin::Dime => {
            return 10;
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
