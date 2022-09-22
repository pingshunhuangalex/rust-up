use std::io;

const CELSIUS_UNIT: &str = "C";
const FAHRENHEIT_UNIT: &str = "F";

fn main() {
    let temperature: f32;
    let temperature_unit: String;

    loop {
        println!("Please input your temperature without unit.");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your temperature");

        temperature = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    };

    loop {
        println!("Please input your temperature unit.\n({CELSIUS_UNIT} for Celsius, {FAHRENHEIT_UNIT} for Fahrenheit)");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your temperature unit");

        let input = input.trim();

        if input == CELSIUS_UNIT || input == FAHRENHEIT_UNIT {
            temperature_unit = input.to_string();
        } else {
            continue;
        };

        break;
    }

    if temperature_unit == CELSIUS_UNIT {
        let temperature = (temperature * 9.0 / 5.0 + 32.0).round();
        println!("The converted temperature is {temperature}{FAHRENHEIT_UNIT}.");
    } else if temperature_unit == FAHRENHEIT_UNIT {
        let temperature = ((temperature - 32.0) * 5.0 / 9.0).round();
        println!("The converted temperature is {temperature}{CELSIUS_UNIT}.");
    } else {
        println!("Nothing was converted.");
    }
}
