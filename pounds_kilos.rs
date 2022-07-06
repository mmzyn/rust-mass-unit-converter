use std::io;

fn main() {
    loop {
        println!("What would you like to convert?");
        println!("For pounds to kilograms enter '1'");
        println!("For kilograms to pounds enter '2'");

        let mut conversion_type = String::new();
        io::stdin().read_line(&mut conversion_type)
            .expect("Failed to read line");
        let conversion_type = conversion_type.trim();
        let conversion_type = match conversion_type {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("Please select either 1 or 2.");
                continue;
            }
        };

        println!("Please input the pounds//kilograms to convert: ");
        let mut mass = String::new();
        io::stdin().read_line(&mut mass)
            .expect("Failed to read line");
        let mass: f64 = match mass.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid mass");
                continue;
            }
        };

        let converted_mass = if conversion_type == 1 {
            mass * 0.45359237
        } else {
            mass / 0.45359237
        };

        println!("The converted mass is {}", converted_mass);

        break;

    }
} 
