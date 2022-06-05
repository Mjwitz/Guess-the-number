use rand::prelude::*;
use std::{thread, time};
use std::io;

fn main() {
    println!("Hello there! You have 3 tries to guess the number other wise we will delete one random file on your computer.");
    thread::sleep(time::Duration::from_secs(5));
    println!("Now whats your first guess.");

    let mut rng = rand::thread_rng();
    let snumber: u8 = rng.gen_range(0..100); // create the randome number 
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("No input found!"); // get the user input

    let mut hold = 100; //hold data that was converted to u8
    let cguess = guess.trim();
    match cguess.parse::<u8>(){
        Ok(i) => {
            println!("Your guess was: {}", i);
            hold = i;
            },
        Err(..) => { 
            println!("You need to wright a number you dunder head!");
            return;
    }
    };

    if hold == snumber {
        println!("You did it on your fist try!!!");
        println!("Good Job!!! You guessed the correct number, {}", hold);
        thread::sleep(time::Duration::from_secs(10));
        return; 
    } else {
        println!("You have failed my young padawan. Try again.");
        if snumber > hold {
            println!("You need to think bigger!");
        } else {
            println!("You need to think smaller!")
        }
        thread::sleep(time::Duration::from_secs(2));
    }


    let mut guess2 = String::new();
    io::stdin().read_line(&mut guess2).expect("No input found!"); // get the user input
    let cguess2 = guess2.trim();
    match cguess2.parse::<u8>(){
        Ok(i) => {
            println!("Your guess was: {}", i);
            hold = i;
            },
        Err(..) => { 
            println!("You need to wright a number you dunder head!");
            return;
    }
    };

    if hold == snumber {
        println!("You did it on your second try!!!");
        println!("Good Job!!! You guessed the correct number, {}", hold);
        thread::sleep(time::Duration::from_secs(10));
        return; 
    } else {
        println!("You have failed my young padawan. Try again. One last try");
        if snumber > hold {
            println!("You need to think bigger!");
        } else {
            println!("You need to think smaller!")
        }
        thread::sleep(time::Duration::from_secs(2));
    }


    let mut guess3 = String::new();
    io::stdin().read_line(&mut guess3).expect("No input found!"); // get the user input
    let cguess3 = guess.trim();
    match cguess3.parse::<u8>(){
        Ok(ii) => {
            println!("Your guess was: {}", ii);
            hold = ii;
            },
        Err(..) => { 
            println!("You need to wright a number you dunder head!");
            return;
    }
    };

    if hold == snumber {
        println!("You did it on your Third try!!!");
        println!("Good Job!!! You guessed the correct number, {}", hold);
        thread::sleep(time::Duration::from_secs(10));
        return; 
    } else {
        println!("You have failed my young padawan. Now we delete one of your files! Bye! The number was, {}", &snumber);
        thread::sleep(time::Duration::from_secs(5));
        return;
    }

}