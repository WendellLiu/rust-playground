// Rust
enum User {
    Consumer {
        last_name: String,
        first_name: String,
    },
    Visitor(String),
}

// impl ToString for Consumer {
//     fn to_string(&self) -> String {
//         format!("{}, {}", self.last_name, self.first_name)
//     }
// }

// impl ToString for Visitor {
//     fn to_string(&self) -> String {
//         format!("{}", self.0)
//     }
// }

impl ToString for User {
    fn to_string(&self) -> String {
        match self {
            User::Consumer {
                last_name,
                first_name,
            } => format!("{}, {}", last_name, first_name),
            User::Visitor(username) => format!("{}", username),
        }
    }
}

fn greeting_text(user: User) -> String {
    match user {
        User::Consumer { .. } => format!("Welcome back, {}", user.to_string()),
        User::Visitor(_) => format!("Hello, {}", user.to_string()),
    }
}

fn main() {
    let consumer = User::Consumer {
        first_name: String::from("Wendell"),
        last_name: String::from("Liu"),
    };

    let visitor = User::Visitor(String::from("Unknown"));

    println!("Consumer: {}", greeting_text(consumer));
    println!("Visitor: {}", greeting_text(visitor))
}
