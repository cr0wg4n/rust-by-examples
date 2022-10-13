use std::any::Any;

enum Sex {
    Male,
    Female
}

struct Animal {
    name: String,
    description: String,
    sex: Sex,
    sound: String
}

impl Animal {
    fn new(name: String, description: String, sex: Sex, sound: String) -> Animal {
        Animal {name, description, sex, sound}
    }

    fn make_sound(&self) {
        println!("....{}", self.sound);
        println!("....{}", self.sound);
        println!("My name is {} and this is my personal description: {}", &self.name, &self.description);
    }
}

fn main() {
    let name: String = String::from("fido");
    let description: String = String::from("soy un perro muy tierno");
    let sound: String = String::from("guaw guaw");
    let perro: Animal = Animal::new(name, description, Sex::Male, sound);
    perro.make_sound();
    match perro.sex {
        Sex::Male => {
            println!("is male!");
        },
        Sex::Female => {
            println!("is female!");
        },
    }
}