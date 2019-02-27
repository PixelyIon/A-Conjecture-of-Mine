// The following program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.

extern crate crossterm;
extern crate num_cpus;
extern crate rand;

use std::{env, thread, time::Instant, sync::mpsc, sync::mpsc::{Sender, Receiver}};
use crossterm::{terminal, input, ClearType};
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prompt = terminal();

    // Assign the correct number of threads to run the application with
    // The default is the number of cores in the machine
    let n_cores = num_cpus::get_physical();
    let n_threads = if args.len() == 0 { n_cores } else {
        match args[0].trim().parse::<usize>() {
            Ok(n_arg) => std::cmp::min(n_arg, n_cores),
            Err(_) => n_cores
        }
    };

    println!("This program is a simple test for the following conjecture:\n");
    println!("Let S: N -> N be the sum of the digits of a positive integer.");
    println!("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.");

    // Listen for user input
    let _ = prompt.write("\nWhat value would you like to test the conjecture for? ");
    let user_input = input().read_line().unwrap_or(String::new());

    match user_input.trim().parse::<usize>() {
        Ok(max) => {
            let start_time = Instant::now();
            println!("\nLOADING. . .");
            let counterexpls = get_all_countrexpls(max, n_threads);
            let duration = start_time.elapsed();

            // Print the results
            if let Err(err) = prompt.clear(ClearType::All) { panic!(err); }
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

            if let Err(err) = prompt.write("Press any key to continue. . . ") { panic!(err); }
            let _ = input().read_char();
        }, 
        Err(_) => println!("'{}' is not a natural number!", user_input.trim())
    }
}

fn get_all_countrexpls(max: usize, n_threads: usize) -> Vec<[usize; 2]> {
    if max / n_threads > 0 && n_threads > 1 {

        // Thread related variables
        let (coutexpl_sender, coutexpl_reciever): (Sender<Vec<[usize; 2]>>, Receiver<Vec<[usize; 2]>>) = mpsc::channel();
        let mut child_threads = Vec::new();
        let range_lenght = max / n_threads;
        let mut range: Vec<usize> = (0..max).collect();

        // Conjecture related variables
        let mut counterexpls: Vec<[usize; 2]> = Vec::new();

        // Shuffle the values in the range to get an even distribution of calculations across all threads
        range.shuffle(&mut thread_rng());

        for i in 1..n_threads {
            let thread_countr_sd = coutexpl_sender.clone();

            // Separate a specific slice of the range and assign it to the thread
            let start = (i - 1) * range_lenght;
            let end = start + range_lenght - 1;
            let thread_range = range[(start as usize)..(end as usize)].to_vec();

            let child = thread::spawn(move || {
                thread_countr_sd.send(get_range_countrexpls(thread_range, max))
                    .expect(&format!("Thread n°{} was unable to sent a message trought the channel", i));
            });
            child_threads.push(child);
        }

        for _ in 1..n_threads {
            counterexpls.append(&mut coutexpl_reciever.recv().unwrap());
        }

        for child in child_threads {
            child.join().expect("Child thread panicked");
        }

        counterexpls
    } else {
        get_range_countrexpls((0..max).collect(), max)
    }
}

fn get_range_countrexpls(range: Vec<usize>, max: usize) -> Vec<[usize; 2]> {
    let mut counterexpls = Vec::new();

    for a in range {
        for b in a..max {
            let difference = sum_digits(a + b) - sum_digits(a) - sum_digits(b);

            if !is_multiple_of_nine(difference) {
                counterexpls.push([a, b]);
            }
        }
    }

    counterexpls
}

fn is_multiple_of_nine(n: isize) -> bool {
    let floor = n / 9;
    let neerest_mult = floor * 9;

    n == neerest_mult
}

fn get_digits(n: usize) -> Vec<u32> {
    n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}

fn sum_digits(n: usize) -> isize {
    let mut sum = 0;

    for d in get_digits(n) {
        sum += d as isize;
    }

    sum
}