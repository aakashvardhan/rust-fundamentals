use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    // make name case insensitive
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let name = name.trim();

    // use of match expression to pattern match against variable "name"
    match name.to_lowercase().as_str() {
        "good morning" => println!("Good morning!"),
        "good evening" => println!("Good evening!"),
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        "how are you?" => println!("I'm doing well!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}

/*
Refletion Questions:

1. What is the purpose of the `match` control flow statement in Rust? 
How does it differ from using multiple `if` statements or `if-else` chains?

Ans: The 'match' control flow statement in Rust is used to compare a value against a series of patterns and then 
execute code based on the matched pattern.

2. In the provided code, what would happen if you added another pattern to the `match` expression, such as "How are you?" => println!("I'm doing well!") ?
 How would the program output change when you enter that specific greeting?

Ans: If you added another pattern to the `match` expression, such as "How are you?" => println!("I'm doing well!"),

Challenge Question:

1. Extend the program by adding more patterns to the `match` expression. For example, you can handle additional greetings like `"Good morning"`, `"Good evening"`, 
or any other greetings you can think of. Print appropriate messages for each pattern.




*/

