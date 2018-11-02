// The following program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

extern crate kernel32;
extern crate winapi;

use winapi::HANDLE;
use winapi::wincon::CONSOLE_SCREEN_BUFFER_INFO;
use winapi::wincon::COORD;
use winapi::wincon::SMALL_RECT;
use winapi::WORD;
use winapi::DWORD;
use std::io::stdin;

static mut CONSOLE_HANDLE: Option<HANDLE> = None;

fn main() {
    let mut user_input = String::new();

    println!("  This program is a simple test for the following conjecture:
    
  Let S: N -> N be the sum of the digits of a positive integer.
  For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.
    
  What value would you like to test the conjecture for?");

    // Listen for user input
    stdin().read_line(&mut user_input).expect("Did not enter a correct string");
    let input_parsing_result = user_input.trim().parse::<i32>();

    // If the user input is a valid int
    if !input_parsing_result.is_err() {
        let max = input_parsing_result.unwrap().abs();
        let counter_examples = get_counter_expl(max);

        // Print the results
        clear_console();
        if counter_examples.len() == 0 {
            println!("The conjecture is proved for all natural numbers smaller or equals to {}!", max);
        } else {
            println!("The conjecture is disproved! Here are the counter examples:");

            for pair in counter_examples {
                println!("{} and {};", pair[0], pair[1]);
            }
        }
    } else {
        println!("'{}' is not an interger!", user_input.trim());
    }
}

// Test the conjecture for all values up to max and return the counterexamples
fn get_counter_expl(max : i32) -> Vec<[i32; 2]> {
    let mut counter_examples : Vec<[i32; 2]> = Vec::new();
    let mut load_bar = 0;

    for a in 0..max {

        // Print the progress on the screen
        let new_load_bar = a * 100 / max;
        if new_load_bar != load_bar {
            load_bar = new_load_bar;
            clear_console();
            println!("LOADING: {}%", new_load_bar);
        }

        for b in a..max {
            let difference : i32 = sum_digits(a + b) - sum_digits(a) - sum_digits(b);

            if !is_multiple_of_nine(difference) {
                counter_examples.push([a, b]);
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


// Console releted code:

fn get_output_handle() -> HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_HANDLE {
            return handle;
        } else {
            let handle = kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE);
            CONSOLE_HANDLE = Some(handle);
            return handle;
        }
    }
}

fn get_buffer_info() -> winapi::CONSOLE_SCREEN_BUFFER_INFO {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    let mut buffer = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0 as WORD,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
    };
    unsafe {
        kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer);
    }
    buffer
}

fn clear_console() {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }

    let screen_buffer = get_buffer_info();
    let console_size: DWORD = screen_buffer.dwSize.X as u32 * screen_buffer.dwSize.Y as u32;
    let coord_screen = COORD { X: 0, Y: 0 };

    let mut amount_chart_written: DWORD = 0;
    unsafe {
        kernel32::FillConsoleOutputCharacterW(
            handle,
            32 as winapi::WCHAR,
            console_size,
            coord_screen,
            &mut amount_chart_written,
        );
    }
    set_cursor_possition(0, 0);
}

fn set_cursor_possition(y: i16, x: i16) {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    unsafe {
        kernel32::SetConsoleCursorPosition(handle, COORD { X: x, Y: y });
    }
}
