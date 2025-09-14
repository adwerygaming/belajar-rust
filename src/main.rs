use text_io::read;

// fn name() {
//     const NAME: &str = "MasDepan";
//     const LANGUAGE: &str = "Rust";

//     println!("Hello, my name is {0} and i'm learning {1}", NAME, LANGUAGE)
// }

// fn test_input() {
//     print!("Enter your input: ");
//     let input: String = read!("{}\n");

//     println!("Your input is: {0}", input)
// }

// fn addition() {
//     print!("Masukan angka 1: ");
//     let input1: i32 = read!();

//     print!("Masukan angka 2: ");
//     let input2: i32 = read!();

//     let sum: i32 = input1 + input2;

//     println!("Result is: {0}", sum);
// }

fn menu() {
    println!("Welcome to main menu.");
    print!("\n[1] Menu C");
    print!("\n[2] Menu A");
    print!("\n[3] Menu B");
    println!("");

    print!("Enter option => ");
    let input: i8 = read!();

    if input == 1 {
        menu_a();
    } else if input == 2 {
        menu_b();
    } else if input == 3 {
        menu_c();
    } else {
        println!("Invalid input!")
    }
}

fn menu_a() {
    println!("You're on Menu A.");
    println!("\n[0] Back to main menu");
    println!("");

    print!("Enter option => ");
    let input: i8 = read!();

    if input == 0 {
        menu();
    } else {
        println!("Invalid option, back to main.");
        main();
    }
}

fn menu_b() {
    println!("You're on Menu B.");
    println!("\n[0] Back to main menu");
    println!("");

    print!("Enter option => ");
    let input: i8 = read!();

    if input == 0 {
        menu();
    } else {
        println!("Invalid option, back to main.");
        main();
    }
}

fn menu_c() {
    println!("You're on Menu C.");
    println!("\n[0] Back to main menu");
    println!("");

    print!("Enter option => ");
    let input: i8 = read!();

    if input == 0 {
        menu();
    } else {
        println!("Invalid option, back to main.");
        main();
    }
}

fn main() {
    // name();
    // test_input();
    // addition();
    menu()
}

