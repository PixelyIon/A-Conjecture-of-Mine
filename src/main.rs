// The following program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

extern crate crossterm;

use std::io::stdin;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::ops::Range;
use crossterm::terminal::{terminal,ClearType};
use crossterm::Screen;

fn main() {
    let screen = Screen::default();
    let prompt = terminal(&screen);

    let mut user_input = String::new();

    println!("This program is a simple test for the following conjecture:
    
Let S: N -> N be the sum of the digits of a positive integer.
For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.
    
What value would you like to test the conjecture for?");

    // Listen for user input
    stdin().read_line(&mut user_input).expect("Did not enter a correct string");
    let input_parsing_result = user_input.trim().parse::<i32>();

    // If the user input is a valid int
    if !input_parsing_result.is_err() {
        println!("\nLOADING...");

        let max = input_parsing_result.unwrap().abs();
        let counterexpls = get_all_countrexpls(max);

        // Print the results
        prompt.clear(ClearType::CurrentLine);
        if counterexpls.len() == 0 {
            println!("The conjecture is proved for all natural numbers smaller or equals to {}!", max);
        } else {
            println!("The conjecture is disproved! Here are the counter examples:");

            for pair in counterexpls {
                println!("{} and {}", pair[0], pair[1]);
            }
        }
    } else {
        println!("'{}' is not an interger!", user_input.trim());
    }
}

fn get_all_countrexpls(max: i32) -> Vec<[i32; 2]> {

    if max > 1000 {
        
        // Thread related variables
        let (coutexpl_sender, coutexpl_reciever): (Sender<Vec<[i32; 2]>>, Receiver<Vec<[i32; 2]>>) = mpsc::channel();
        let mut child_threads = Vec::new();
        let n_threads = 10;
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
