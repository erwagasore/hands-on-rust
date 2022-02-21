use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Probation,
    Refuse,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alchool to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now probetionary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in", self.name),
        }
    }
}

fn get_user_input() -> String {
    let mut username = String::new();
    stdin().read_line(&mut username).expect("Failed to read line");
    username.trim().to_lowercase()
}

fn main() {
    let mut visitors = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new("steve", VisitorAction::AcceptWithNote {
            note: String::from("Lactose-free milk is in the fridge")
        }, 15),
        Visitor::new("Eugene", VisitorAction::Probation, 36),
        Visitor::new("fred", VisitorAction::Refuse, 30),
    ];

    loop {
         println!("Hello, what is your name? ");
        let username = get_user_input();

        let invited_visitor = visitors.iter().find(|visitor| visitor.name == username);

        match invited_visitor {
            Some(visitor) => visitor.greet(),
            None if username.is_empty() => {
                println!("The final list of visitors:");
                println!("{:#?}", visitors);
                break;
            }
            None => {
                println!("{} is not on the visitor list", username);
                visitors.push(
                    Visitor::new(&username, VisitorAction::AcceptWithNote {
                        note: String::from("Was not invited")
                    }, 0)
                );
            }
        }
   }
}
