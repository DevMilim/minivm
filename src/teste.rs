use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Active,
    Inactive,
    Suspended { reason: String },
}

struct User<T> {
    id: u64,
    name: String,
    data: T,
    status: Status,
}

trait Repository<T> {
    fn insert(&mut self, id: u64, value: T);
    fn get(&self, id: u64) -> Option<&T>;
}

struct UserRepository<T> {
    users: HashMap<u64, T>,
}

impl<T> UserRepository<T> {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    fn count(&self) -> usize {
        self.users.len()
    }
}

impl<T> Repository<T> for UserRepository<T> {
    fn insert(&mut self, id: u64, value: T) {
        self.users.insert(id, value);
    }

    fn get(&self, id: u64) -> Option<&T> {
        self.users.get(&id)
    }
}

fn process_user<T: std::fmt::Debug>(user: &User<T>) {
    match &user.status {
        Status::Active => {
            println!("User {} is active", user.name);
        }
        Status::Inactive => {
            println!("User {} is inactive", user.name);
        }
        Status::Suspended { reason } => {
            println!("Suspended: {}", reason);
        }
    }
}

fn main() {
    let mut repository = UserRepository::new();

    let user = User {
        id: 1,
        name: String::from("Milim"),
        data: vec![10, 20, 30],
        status: Status::Active,
    };

    repository.insert(user.id, user);

    if let Some(user) = repository.get(1) {
        process_user(user);
    }

    let numbers: Vec<u32> = (0..10).filter(|n| n % 2 == 0).map(|n| n * 2).collect();

    for number in &numbers {
        println!("Number: {number}");
    }

    let closure = |x: i32, y: i32| -> i32 { x + y };

    println!("{}", closure(10, 20));

    let text = "Raw string with quotes and characters";

    println!("{text}");

    let result: Result<u32, String> = Ok(42);

    match result {
        Ok(value) if value > 0 => println!("Value: {value}"),
        Ok(_) => println!("Zero"),
        Err(error) => eprintln!("Error: {error}"),
    }
}
