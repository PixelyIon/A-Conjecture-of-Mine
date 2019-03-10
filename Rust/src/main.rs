// The following program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.

extern crate crossterm;
extern crate num_cpus;
extern crate rand;

use std::{env, thread, time::Instant, sync::mpsc, sync::mpsc::{Sender, Receiver}};
use crossterm::{terminal, input};
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prompt = terminal();

    // Assign the correct number of threads to run the application with
    // The default is the number of cores in the machine
    let n_cores = num_cpus::get_physical();
    let n_threads = if args.len() <= 1 { n_cores } else {
        match args[1].trim().parse::<usize>() {
            Ok(n_arg) => std::cmp::min(n_arg, n_cores),
            Err(_) => n_cores
        }
    };

    println!("\nThis program is a simple test for the following conjecture:\n");
    println!("Let S: N -> N be the sum of the digits of a positive integer.");
    println!("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.");

    // Listen for user input
    let _ = prompt.write("\nWhat value would you like to test the conjecture for? ");
    let user_input = input().read_line().unwrap_or(String::new());

    match user_input.trim().parse::<usize>() {
        Ok(max) => {
            println!("\nLOADING. . .");
            let start_time = Instant::now();
            let counterexpls = get_all_countrexpls(max, n_threads);
            let duration = start_time.elapsed();

            // Print the results
            println!("LOADED. . . in {}s [{} Threads]\n", duration.as_secs(), n_threads);
            if counterexpls.len() == 0 {
                println!("The conjecture is proved for all natural numbers smaller or equals to {}!", max);
            } else {
                println!("The conjecture is disproved! Here are the counter examples:");

                let counterexpls_str: Vec<String> = counterexpls.iter().map(|(a, b)| format!("({}, {})", a, b)).collect();
                println!("{}\n", counterexpls_str.join(", "));
            }
        }, 
        Err(_) => println!("'{}' is not a natural number!", user_input.trim())
    }
}

fn get_all_countrexpls(max: usize, n_threads: usize) -> Vec<(usize, usize)> {
    if max / n_threads > 0 && n_threads > 1 {

        // Thread related variables
        let (coutexpl_sender, coutexpl_reciever): (Sender<Vec<(usize, usize)>>, Receiver<Vec<(usize, usize)>>) = mpsc::channel();
        let mut child_threads = Vec::with_capacity(n_threads);
        let range_lenght = max / n_threads;
        let mut range: Vec<usize> = (0..max).collect();

        // Conjecture related variables
        let mut counterexpls: Vec<(usize, usize)> = Vec::new();

        // Shuffle the values in the range to get an even distribution of
        // calculations across all threads
        range.shuffle(&mut thread_rng());

        // Separate a specific slice of the range and assign it to the thread
        let mut sub_ranges = Vec::with_capacity(n_threads);
        for i in 0..n_threads {
            let start = i * range_lenght;
            let end = start + range_lenght;
            sub_ranges.push(range[(start as usize)..(end as usize)].to_vec());
        }

        // Account for the fact that the maximum number tested may not be
        // a multiple of the numbers of threads used for computations, hence
        // the number of tests performed by each thread may not be constant
        if max % n_threads != 0 {
            let mut rng = thread_rng();
            let end = sub_ranges.len() - 1;
            let mut remainders = range[(max - max % n_threads)..max].to_vec();

            while let Some(val) = remainders.pop() {
                sub_ranges[rng.gen_range(0, end)].push(val);
            }
        }

        for i in 0..n_threads {
            let thread_countr_sd = coutexpl_sender.clone();
            let thread_range = sub_ranges.pop().unwrap();

            let child = thread::spawn(move || {
                thread_countr_sd.send(get_range_countrexpls(thread_range, max))
                    .expect(&format!("Thread n°{} was unable to sent a message trought the channel", i));
            });
            
            child_threads.push(child);
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

fn get_range_countrexpls(range: Vec<usize>, max: usize) -> Vec<(usize, usize)> {
    let mut counterexpls = Vec::new();

    for a in range {
        for b in a..max {
            let difference = sum_digits(a + b) - sum_digits(a) - sum_digits(b);

            if difference % 9 != 0 {
                counterexpls.push((a, b));
            }
        }
    }

    counterexpls
}

fn get_digits(n: usize) -> Vec<usize> {
    if n == 0 {
        vec![0]
    } else {
        let mut output = Vec::with_capacity((n as f64).log(10.0).floor() as usize + 2);
        let mut part = n;

        while part != 0 {
            output.push(part % 10);
            part /= 10;
        }

        output
    }
}

fn sum_digits(n: usize) -> isize {
    get_digits(n).iter().sum::<usize>() as isize
}