use std::io;

fn main() {
    'choose_type: loop {
        println!("Choose the type of conversion: ");
        println!("[1] °F to °C");
        println!("[2] °C to °F");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line!");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        loop {
            if option == 1 {
                println!("Enter the temperature value in °F: ");
                let mut temp_in_f = String::new();
                io::stdin()
                    .read_line(&mut temp_in_f)
                    .expect("Failed to read line!");
                // let temp_in_f: f64 = temp_in_f.trim().parse().expect("Must be a number!");
                let temp_in_f: f64 = match temp_in_f.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temp_in_c: f64 = (temp_in_f - 32.0) / 1.8;
                let unit: char = 'C';
                println!("The temperature is {:.2}°{}.", temp_in_c, unit);
                break 'choose_type;
            } else if option == 2 {
                println!("Enter the temperature value in °C: ");
                let mut temp_in_c = String::new();
                io::stdin()
                    .read_line(&mut temp_in_c)
                    .expect("Failed to read line!");
                // let temp_in_c: f64 = temp_in_c.trim().parse().expect("Must be a number!");
                let temp_in_c: f64 = match temp_in_c.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temp_in_f: f64 = temp_in_c * 1.8 + 32.0;
                let unit: char = 'F';
                println!("The temperature is {:.2}°{}.", temp_in_f, unit);
                break 'choose_type;
            } else {
                println!("Please pick either [1] or [2]!");
                continue 'choose_type;
            }
        }
    }
}
