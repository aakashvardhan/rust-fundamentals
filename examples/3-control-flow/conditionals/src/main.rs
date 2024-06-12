fn main() {
    let proceed = true;
    // proceed = false;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    if height > 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }

    let age = 25;

    if age < 20 {
        println!("Teenager");
    } else if age < 13 {
        println!("Child");
    } else {
        println!("Adult");
    }

}


/*
Reflection Questions:
How does the `if` statement work in Rust? How is it similar or different from other programming languages you've used?

Ans: The `if` statement in Rust is similar to other programming languages in terms of syntax and structure.
However, Rust enforces a strict rule that the condition must be a boolean expression.
This means that the condition must evaluate to either `true` or `false`, and you cannot use non-boolean values directly in the condition.
This helps prevent common programming errors where non-boolean values are mistakenly used in conditions at compile time.

What would happen if you change the value of the `proceed` variable to `true`? How would the program output change?

Ans: If you change the value of the `proceed` variable to `true`, the program would output "Proceeding" instead of "Not proceeding".

Challenge Questions:
Modify the code to include an additional condition. For example, you could add a variable `age` and print different messages based on the age range (e.g., "Child", "Teenager", "Adult"). Experiment with different values to test the program's behavior.

Extend the program to handle nested if-else statements. For instance, you can add a condition based on a user's gender and age range to print specific messages. How would you structure the nested if-else statements to achieve the desired behavior?


*/
