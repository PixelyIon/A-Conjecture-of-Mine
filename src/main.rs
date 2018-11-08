// The following program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

extern crate crossterm;

use std::io::{stdin, stdout, prelude::*};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::ops::Range;
use std::env;
use std::process::Command;
use crossterm::terminal::{terminal,ClearType};
use crossterm::Screen;

fn main() {
    let args: Vec<String> = env::args().collect();
    let screen = Screen::default();
    let prompt = terminal(&screen);

    // Assign the correct number of threads to run the application with
    // The default is 10
    let n_threads = if args.len() == 0 { 10 } else {
         match args[0].trim().parse::<i32>() {
             Ok(n) => n,
             Err(_) => 10
         }
    };

    println!("This program is a simple test for the following conjecture:
    
Let S: N -> N be the sum of the digits of a positive integer.
For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.");

    // Listen for user input
    let user_input = ask("\nWhat value would you like to test the conjecture for? ".to_owned());

    match user_input.trim().parse::<i32>() {
        Ok(max) => {
            println!("\nLOADING...");
            let counterexpls = get_all_countrexpls(max, n_threads);

            // Print the results
            prompt.clear(ClearType::All);
            println!("LOADED... 100%\n");
            if counterexpls.len() == 0 {
                println!("The conjecture is proved for all natural numbers smaller or equals to {}!\n", max);
            } else {
                println!("The conjecture is disproved! Here are the counter examples:");

                for pair in counterexpls {
                    println!("{} and {}", pair[0], pair[1]);
                }

                println!("");
            }

            pause_prompt();
        }, 
        Err(_) => println!("'{}' is not an interger!", user_input.trim())
    }
}

fn get_all_countrexpls(max: i32, n_threads: i32) -> Vec<[i32; 2]> {

    if max > 1000 {

        // Thread related variables
        let (coutexpl_sender, coutexpl_reciever): (Sender<Vec<[i32; 2]>>, Receiver<Vec<[i32; 2]>>) = mpsc::channel();
        let mut child_threads = Vec::new();
        let range_lenght = ((max as f32) / n_threads as f32).ceil() as i32;

        // Conjecture related variables
        let mut counterexpls: Vec<[i32; 2]> = Vec::new();

        for i in 0..n_threads {
            let thread_countr_sd = coutexpl_sender.clone();
            let end = std::cmp::min(i * (range_lenght + 1) + range_lenght, max);
            let range = Range {start: i * (range_lenght + 1), end: end};

            let child = thread::spawn(move || {
                thread_countr_sd.send(get_range_countrexpls(range, max))
                    .expect("The thread was unable to sent a message trought the channel");
            });
            child_threads.push(child);
        }

        for _ in 0..n_threads {
            counterexpls.append(&mut coutexpl_reciever.recv().unwrap());
        }

        for child in child_threads {
            child.join().expect("Child thread panicked");
        }

        return counterexpls;

    } else {
        return get_range_countrexpls(0..max, max);
    }
}

fn get_range_countrexpls(range: Range<i32>, max: i32) -> Vec<[i32; 2]> {
    let mut counterexpls = Vec::new();

    for a in range {
        for b in a..max {
            let difference: i32 = sum_digits(a + b) - sum_digits(a) - sum_digits(b);

            if !is_multiple_of_nine(difference) {
                counterexpls.push([a, b]);
            }
        }
    }

    return counterexpls;
}

fn is_multiple_of_nine(n: i32) -> bool {
    let n_float = n as f32;

    return (n_float/9f32) % 1f32 == 0f32;
}

fn get_digits(n: i32) -> Vec<u32> {
    return n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
}

fn sum_digits(n: i32) -> i32 {
    let mut sum = 0;

    for d in get_digits(n) {
        let _d = d as i32;
        sum += _d;
    }

    return sum;
}

fn ask(message: String) -> String {
    // Print the question
    write!(stdout(), "{}", message).unwrap();
    stdout().flush().unwrap();

    // Return the responce
    let mut response = String::new();
    stdin().read_line(&mut response).unwrap();
    return response;
}

fn pause_prompt() {
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}