// The following script is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

fn main() {
    let max : i32 = 1000;
    let counter_examples : Vec<[i32; 2]> = test_conjecture(max);

    // Print the results
    if counter_examples.len() == 0 {
        println!("The conjecture holds up to {}!", max);
    } else {
        println!("The conjecture doesn't hold! Here are the counter examples:");

        for pair in counter_examples {
            println!("{} and {};", pair[0], pair[1]);
        }
    }
}

// Test the conjecture for all values up to max and return the counterexamples
fn test_conjecture(max : i32) -> Vec<[i32; 2]> {
    let mut counter_examples : Vec<[i32; 2]> = Vec::new();
    let mut load_bar = 0f32;

    for a in 0..max {
        for b in a..max {
            let difference : i32 = sum_digits(a + b) - sum_digits(a) - sum_digits(b);

            if !is_multiple_of_nine(difference) {
                counter_examples.push([a, b]);
            }

            // Print the progress on the screen
            let new_load_bar = ((a as f32) * 100f32 / (max as f32)).ceil();

            if new_load_bar != load_bar {
                load_bar = new_load_bar;
                print!("{}[2J", 27 as char);
                println!("LOADING: {}%", new_load_bar);
            }
        }
    }

    return counter_examples;
}

fn is_multiple_of_nine(n : i32) -> bool {
    let n_float = n as f32;

    return (n_float/9f32) % 1f32 == 0f32;
}

fn get_digits(n : i32) -> Vec<u32> {
    return n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
}

fn sum_digits(n : i32) -> i32 {
    let mut sum : i32 = 0i32;

    for d in get_digits(n) {
        let d_ = d as i32;
        sum += d_;
    }

    return sum;
}
