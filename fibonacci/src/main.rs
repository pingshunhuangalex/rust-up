use std::io;

fn main() {
    let mut index: u8;
    let mut output: usize = 0;

    'user_input: loop {
        println!("\nWhich Fibonacci number do you want? (input \"1\" for the first in the sequence)");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your input");

        index = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The number has to be a non-zero positive integer.");
                continue;
            },
        };

        let mut n: u8 = 0;
        let mut last_second_num: usize = 0;
        let mut last_num: usize = 1;

        while n < index {
            if n == 0 {
                output = 0;
            } else if n == 1 {
                output = 1;
            } else {
                if (last_second_num).checked_add(last_num) == None {
                    println!("The Fibonacci number requested is too big. Please try a smaller input.");
                    continue 'user_input;
                } else {
                    output = last_second_num + last_num;
                }

                last_second_num = last_num;
                last_num = output;
            }

            n += 1;
        }

        println!("The Fibonacci number requested is {output}.");
        break;
    };
}
