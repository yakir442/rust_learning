use std::io;


fn main() {
    println!("Hey this program convert temperatures between Fahrenheit and Celsius");
    println!("the calc works this way:\n first you will give a temperature value\n second you will give unit\n and then the calc will print the complement temperatur.");

    loop {
        let mut temp_value = String::new();  // going to be integer
        let mut temp_unit = String::new();   // need to be char

        println!("Please input the temperature value:");
        io::stdin()
            .read_line(&mut temp_value)
            .expect("Failed to read line");
        
        let temp_value :f64 = match temp_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Please input the temperature unit (write C for celsius and F for fahrenheit):");
        io::stdin()
            .read_line(&mut temp_unit)
            .expect("Failed to read line");
        let temp_unit :char = match temp_unit.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };


        if temp_unit == 'C' {
            let temp_value = (temp_value*1.8)+32.0;
            let temp_unit = 'F';
            println!("the converted temperatur is {temp_value} {temp_unit}");
        } else if temp_unit == 'F' {
            let temp_value = (temp_value-32.0)/1.8;
            let temp_unit = 'C';
            println!("the converted temperature is {temp_value} {temp_unit}");
        } else {
            println!("something went wrong try again.");
            continue;
        }
        loop {
            let mut choise = String::new();   // need to be char
            println!("Do you want to convert another temperature ? (Y/N)");
            io::stdin()
                .read_line(&mut choise)
                .expect("Failed to read line");
            let choise :char = match choise.trim().parse() {
            Ok(char) => char,
            Err(err) => {
                println!("{:?}",err);
                continue;
            },
            };    
            match choise {
                'Y' => break,
                'N' => std::process::exit(0),
                _   => {
                    println!("wrong input please try again");
                    continue; 
                    }
            }    
        }
    }
    
    
}
