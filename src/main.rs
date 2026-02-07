use std::io;

//TODO: 1. Holy Trinity
//TODO: 2. Data Modeling

fn main() {
    // mut = mutuble variable which means we can change or update the variable
    let mut x = 10;
    println!("{x}");
    x = 5;
    std::println!("{x}");
    const AGE: &str = "Name";

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

    // guessing();

    println!("1. Balance");
    println!("2. Account");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid Operations");

    let operations = input.trim().parse::<i32>();

    match operations {
        Ok(1) => balance(),
        Ok(2) => account(),
        _ => println!("Invalid Operations"),
    }

    sqr(5, 5);
    let y = {
        let x = 10;
        x + 1
    };

    println!("{}", y);

    let isAdult = false;

    let age = if isAdult { 5 } else { 6 };

    println!("{age}");

    let password = "xyrel";

    let login = if password == "xyrel" {
        "Authorized"
    } else {
        "unAuthorized"
    };

    println!("{login}");

    // i = signed
    // u = unsigned

    // f = float
    // i = integer

    // Shadowing
    let year = "2006";
    let year: i32 = year.parse().unwrap();
    std::println!("{year}");

    let i: u8 = 10;
    let j: u8 = 5;
    std::println!("{}", i);
    std::println!("{}", i & j);
    // std::println!("{}", j | i);
    // std::println!("{}", j ^ i);
    // std::println!("{}", j << i);
    // std::println!("{}", j >> i);

    // Loops
    for i in 1..=5 {
        println!("Number: {}", i);
    }

    for j in 1..100 {
        println!("Starts: {}", j);
    }
}

fn sqr(x: i32, y: i32) -> i32 {
    x * y
}

fn balance() {
    println!("Balance")
}

fn account() {
    println!("Account")
}

fn guessing() {
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read");

    let trimmed = input_text.trim();

    match trimmed.parse::<i32>() {
        Ok(i) => println!("Your integator input: {}", i),
        Err(..) => println!("not an integet: {}", trimmed),
    }
}
