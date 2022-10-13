
use rand::Rng;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::distributions::Standard;
use rand::distributions::Alphanumeric;
use rand_distr::{Normal};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (randx, randy) = rng.gen();
        Point {
            x: randx,
            y: randy
        }
    }
}

fn main() {
    random_without_ranges();
    println!();
    random_with_ranges();
    println!();
    // uniform distribution aproach is very fast when you need iterations
    random_with_uniform_distribution(); 
    // custom distributions
    random_with_custom_distributions();
    // random in custom types
    random_values_in_custom_types();
    // randomize string 
    random_password();
    // randomize string from a charset
    random_password_from_charset();
}

fn random_without_ranges() {
    let mut random = rand::thread_rng();
    let number_a: u8 = random.gen();
    let number_b: u16 = random.gen();
    println!("Random u8: {}", number_a);
    println!("Random u16: {}", number_b);
    println!("Random float: {}", random.gen::<f32>());
}

fn random_with_ranges() {
    let mut random = rand::thread_rng();
    println!("Interger: {}", random.gen_range(0..100));
    println!("Float: {}", random.gen_range(0.1..12.2));
}

fn random_with_uniform_distribution() {
    let mut random = rand::thread_rng();
    let dice: Uniform<u32> = Uniform::from(1..7);
    loop {
        let throw = dice.sample(&mut random);
        println!("Roll the dice: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn random_with_custom_distributions() {
    let mut random = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut random);
    println!("{} is from a N(2,9) distributio", v);
}

fn random_values_in_custom_types() {
    let mut random = rand::thread_rng();
    let rand_tuple = random.gen::<(i32,bool,f64)>();
    let rand_point: Point = random.gen();
    println!("Random tumple: {:?}", rand_tuple);
    println!("Random tumple: {:?}", rand_point);
}

fn random_password() {
    let random_string: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();
    println!("{}", random_string);
}

fn random_password_from_charset() {
    let mut random = rand::thread_rng();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let password: String = (0..PASSWORD_LEN)
        .map(|_|{
            let idx = random.gen_range(0..CHARSET.len());
            return CHARSET[idx] as char;
        })
        .collect();

    println!("{}", password);
}