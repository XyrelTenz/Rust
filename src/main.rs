mod models;

use models::bankmodel::BankModel;
use std::io;

//TODO: 1. Holy Trinity
//TODO: 2. Data Modeling

fn main() {
    // mut = mutuble variable which means we can change or update the variable
    let mut x = 10;
    println!("{x}");
    x = 5;
    std::println!("{x}");
    const _AGE: &str = "Name";

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

    let is_adult = false;

    let age = if is_adult { 5 } else { 6 };

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

    let _nums = 10;
    let mut _nums: i32 = 10;

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

    // Ownership
    let s1 = String::from("hello");
    let s2 = &s1.clone();

    std::println!("{}", s2);

    let s3 = String::from("Hi");

    takes_ownership(s3);

    let (s2, len) = calculate_length(s1);

    std::println!("The length of {s2} is len{len}");

    let len: usize = calculate(&s2);

    std::println!("{len}");

    // Slice Types
    let mut words = String::from("Xyrel");

    let word = first_word(&words);

    std::println!("{word}");

    words.clear();

    let words: String = String::from("Hello World");

    let hello = &words[0..5];
    let xyrel = &words[6..11];

    std::println!("{hello}");
    std::println!("{xyrel}");

    let mut check_age = String::new();

    io::stdin().read_line(&mut check_age).expect("Did not read");

    let age = 18;

    let is_adult = if age > 18 { "adult" } else { "minor" };

    std::println!("{}", is_adult);

    const TASK: i64 = 50;
    const TIME: i64 = 15 * 60;

    // let total = TASK.parse::<&str>().trim() / TIME.parse::<&str>().trim();

    // std::println!("{}", TOTAL);

    getsize(age.to_string());

    let balance = BankModel::new(100);

    std::println!("{}", balance.balance);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return i;
        }
    }

    s.len()
}

fn calculate(s: &str) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn _giveback(str: String) -> String {
    str
}

fn takes_ownership(string: String) {
    println!("{string}");
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

fn _guessing() {
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read");

    let trimmed = input_text.trim();

    match trimmed.parse::<i32>() {
        Ok(i) => println!("Your integator input: {}", i),
        Err(..) => println!("not an integet: {}", trimmed),
    }

    static ASSETS: String = String::new();
}

// TODO: Fix this tommorow
fn getsize(n: String) -> (String, u64) {
    let length = n.len();

    let convert = length as u64;

    (n, convert)
}
