// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    //let maybe_number: Option<Option<()>> = Some(None); // type annotation is optional with () as the value
    let maybe_number = Some(Some(36));
    if let Some(Some(42)) = maybe_number {
        println!("The number is 42");
    } else if let Some(Some(number)) = maybe_number {
        println!("The number is {:?}", number);
    } else {
        println!("No number found");
    }
}




/*

Reflection Questions:

1. What is the purpose of the `if let` construct in Rust? 
How does it differ from using a traditional `if` statement with pattern matching?

Ans: The 'if let' construct in Rust is used to match a single pattern and execute code if the pattern matches.
Just 'if' statement with pattern matching is used to match multiple patterns and execute code based on the matched pattern.

2. In the provided code, the `maybe_number` variable is initially assigned `None`. What would happen if you uncommented the line `//let maybe_number = Some(42);` 
and ran the program? How would the output change? Why?

Ans: If you uncommented the line `//let maybe_number = Some(42);` and ran the program, the output would change to "The number is 42".

Challenge Question:

1. Modify the code to include an additional level of nesting for the `maybe_number` variable. For example, you can assign `Some(Some(42))` to `maybe_number`. 
Update the `if let` construct to handle this additional level of nesting and print the number accordingly.



*/
