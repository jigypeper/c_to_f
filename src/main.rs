use std::io;

fn main() {
    println!("Welcome to the celsius to fahrenheit converter!");

    loop {
        println!("Please enter the temperature you would like to convert.");

        let mut temp: String = String::new();


        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temp_f = (temp * (9.0/5.0)) + 32.0;

        println!("The result it: {temp_f} deg F");
        break;
    }
    
}
