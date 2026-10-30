const VALUES: &str = "HELLO";

pub enum Values {
    X(i32),
    Y(u32),
    Pos { x: i32, y: i32 },
    None,
}

pub struct Person {
    name: &'static str,
    age: i32,
}

impl Person {
    pub fn get_age(&self) -> i32 {
        self.age
    }
}

fn main() {
    let x = 12;
    let y = 12;
    let person = Person {
        name: "Adam",
        age: 21234,
    };
    let adam_age = person.get_age();
    let list = [1, 2, 3, 4, 5];
    let mut total = 0;
    for i in list {
        total += i;
    }
    if let Values::X(i) = X(12) {
        i;
    }
}
