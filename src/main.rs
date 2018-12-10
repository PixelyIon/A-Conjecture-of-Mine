// The following program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.

extern crate crossterm;
extern crate num_cpus;
extern crate rand;

use std::io::{stdin, stdout, prelude::*};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::env;
use std::process::Command;
use std::time::Instant;
use crossterm::terminal::{terminal,ClearType};
use crossterm::Screen;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let screen = Screen::default();
    let prompt = terminal(&screen);

    // Assign the correct number of threads to run the application with
    // The default is the number of cores in the machine
    let n_cores = num_cpus::get_physical() as i64;
    let n_threads = if args.len() == 0 { n_cores } else {
        match args[0].trim().parse::<i64>() {
            Ok(n) => std::cmp::min(n.abs(), n_cores),
            Err(_) => n_cores
        }
    };

    println!("This program is a simple test for the following conjecture:
    
Let S: N -> N be the sum of the digits of a positive integer.
For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.");

    // Listen for user input
    let user_input = ask("\nWhat value would you like to test the conjecture for?");

    match user_input.trim().parse::<i64>() {
        Ok(max) => {
            let start_time = Instant::now();
            println!("\nLOADING. . .");
            let counterexpls = get_all_countrexpls(max.abs(), n_threads);
            let duration = start_time.elapsed();

            // Print the results
            prompt.clear(ClearType::All);
            println!("LOADED. . . 100% in {}s [{} threads]\n", duration.as_secs(), n_threads);
            if counterexpls.len() == 0 {
                println!("The conjecture is proved for all natural numbers smaller or equals to {}!", max);
            } else {
                println!("The conjecture is disproved! Here are the counter examples:");

                let mut counterexpls_str = String::new();

                for pair in counterexpls {
                    let ordered_pair = format!("({}, {})", pair[0], pair[1]);
                    counterexpls_str = format!("{}, {}", counterexpls_str, ordered_pair);
                }

                println!("{}\n", counterexpls_str);
            }

            pause_prompt();
        }, 
        Err(_) => println!("'{}' is not an integer!", user_input.trim())
    }
}

fn get_all_countrexpls(max: i64, n_threads: i64) -> Vec<[i64; 2]> {
    if n_threads < 1 { panic!("The number {} is not a valid number of threads.", n_threads) }

    if max / n_threads > 0 && n_threads > 1 {

        // Thread related variables
        let (coutexpl_sender, coutexpl_reciever): (Sender<Vec<[i64; 2]>>, Receiver<Vec<[i64; 2]>>) = mpsc::channel();
        let mut child_threads = Vec::new();
        let range_lenght = ((max as f64) / n_threads as f64).ceil() as i64;
        let mut range: Vec<i64> = (0..max).collect();

        // Conjecture related variables
        let mut counterexpls: Vec<[i64; 2]> = Vec::new();

        // Shuffle the values in the range to get an even distribution of calculations across all threads
        range.shuffle(&mut thread_rng());

        for i in 1..n_threads {
            let thread_countr_sd = coutexpl_sender.clone();

            let start = (i - 1) * range_lenght;
            let end = start + range_lenght - 1;
            let thread_range = range[(start as usize)..(end as usize)].to_vec();

            let child = thread::spawn(move || {
                thread_countr_sd.send(get_range_countrexpls(thread_range, max))
                    .expect(&*format!("Thread n°{} was unable to sent a message trought the channel", i));
            });
            child_threads.push(child);
        }

        for _ in 1..n_threads {
            counterexpls.append(&mut coutexpl_reciever.recv().unwrap());
        }

        for child in child_threads {
            child.join().expect("Child thread panicked");
        }

        return counterexpls;

    } else {
        return get_range_countrexpls((0..max).collect(), max);
    }
}

fn get_range_countrexpls(range: Vec<i64>, max: i64) -> Vec<[i64; 2]> {
    let mut counterexpls = Vec::new();

    for a in range {
        for b in a..max {
            let difference: i64 = sum_digits(a + b) - sum_digits(a) - sum_digits(b);

            if !is_multiple_of_nine(difference) {
                counterexpls.push([a, b]);
            }
        }
    }

    return counterexpls;
}

fn is_multiple_of_nine(n: i64) -> bool {
    let floor = n / 9;
    let neerest_mult = floor * 9;

    return n == neerest_mult;
}

fn get_digits(n: i64) -> Vec<i64> {
    return n.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
}

fn sum_digits(n: i64) -> i64 {
    let mut sum = 0;

    for d in get_digits(n) {
        sum += d;
    }

    return sum;
}

fn ask(message: &str) -> String {
    // Print the question
    write!(stdout(), "{} ", message).unwrap();
    stdout().flush().unwrap();

    // Return the responce
    let mut response = String::new();
    stdin().read_line(&mut response).unwrap();
    return response;
}

fn pause_prompt() {
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}