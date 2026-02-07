use std::io;

fn main() {
    // mut = mutuble variable which means we can change or update the variable
    let mut x = 10;
    println!("{x}");
    x = 5;
    std::println!("{x}");

    // Scope
    {
        let x = x * 3;
        println!("{x}");
    }

    // Guessing Game
    // {
    //     println!("Enter Name: ");
    //
    //     let mut guess = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     println!("You guess: {guess}");
    // }

    {
        const NAME: &str = "Xyrel";

        std::println!("{NAME}");
    }
}
