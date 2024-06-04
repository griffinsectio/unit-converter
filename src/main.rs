use dialoguer::{Confirm, Input, Select};
use console::Term;

fn temperature(term: &Term) {
    let units = vec!["Celcius", "Fahrenheit", "Kelvin", "back"];
    let mut first_unit: String;
    let mut first_unit_value: f64;
    let mut second_unit: String;

    loop {
        let first_choice = Select::new()
        .with_prompt("Unit you want to convert from")
        .items(&units)
        .default(0)
        .interact()
        .unwrap();
        
        if first_choice == 3 {
            term.clear_screen().unwrap();
            return
        }

        first_unit = units[first_choice].to_string();
        first_unit_value = Input::new()
        .with_prompt("The value")
        .interact_text()
        .unwrap();
    
        let second_choice = Select::new()
        .with_prompt("Unit you want to convert to")
        .items(&units)
        .default(0)
        .interact()
        .unwrap();

        second_unit = units[second_choice].to_string();
    
        if second_choice == 3 {
            term.clear_screen().unwrap();
            continue
        }
        break
    }

    if first_unit == "Celcius".to_string() {
        if second_unit == "Fahrenheit".to_string() {
            println!("The result is {:.2} F", (first_unit_value as f64 * 1.8) + 32.0)
        } else if second_unit == "Kelvin".to_string() {
            println!("The result is {:.2} K", first_unit_value as f64 + 273.15)
        } else {
            println!("The result is {} C", first_unit_value)
        }
    } else if first_unit == "Fahrenheit".to_string() {
        if second_unit == "Celcius".to_string() {
            println!("The result is {:.2} C", (first_unit_value as f64 - 32.0) / 1.8)
        } else if second_unit == "Kelvin".to_string() {
            println!("The result is {:.2} K", ((first_unit_value as f64 - 32.0) /1.8) + 273.15)
        } else {
            println!("The result is {} F", first_unit_value)
        }
    } else if first_unit == "Kelvin".to_string() {
        if second_unit == "Celcius".to_string() {
            println!("The result is {} C", first_unit_value as f64 - 273.15)
        } else if second_unit == "Fahrenheit".to_string() {
            println!("The result is {} F", ((first_unit_value as f64 + 273.15) * 1.8) + 32.0)
        } else {
            println!("The result is {} K", first_unit_value)
        }
    }
    let confirmation = Confirm::new()
    .with_prompt("Do you want to convert again")
    .default(false)
    .wait_for_newline(true)
    .interact()
    .unwrap();

    if confirmation {
        term.clear_screen().unwrap();
        temperature(term)
    }
    term.clear_screen().unwrap();
}

fn length(term: &Term) {
    let units = vec!["Mile", "Meter", "Feet", "Inch", "back"];
    let mut first_unit: String;
    let mut first_unit_value: f64;
    let mut second_unit: String;

    loop {
        let first_choice = Select::new()
        .with_prompt("Unit you want to convert from")
        .items(&units)
        .default(0)
        .interact()
        .unwrap();
        
        if first_choice == 4 {
            term.clear_screen().unwrap();
            return
        }

        first_unit = units[first_choice].to_string();
        first_unit_value = Input::new()
        .with_prompt("The value")
        .interact_text()
        .unwrap();
    
        let second_choice = Select::new()
        .with_prompt("Unit you want to convert to")
        .items(&units)
        .default(0)
        .interact()
        .unwrap();

        second_unit = units[second_choice].to_string();
    
        if second_choice == 4 {
            term.clear_screen().unwrap();
            continue
        }
        break
    }

    if first_unit == "Mile".to_string() {
        if second_unit == "Meter".to_string() {
            println!("The result is {:.2}m", ((first_unit_value as f64) * 1.6093) * 1000.0)
        } else if second_unit == "Feet".to_string() {
            println!("The result is {:.2}ft", ((first_unit_value as f64) / 1000.0 * 1.6093) * 3.2808)
        } else if second_unit == "Inch".to_string() {
            println!("The result is {:.2}in", (((first_unit_value as f64)/1000.0 * 1.6093) * 3.2808) * 12.0)
        } else {
            println!("The result is {}mi", first_unit_value)
        }
    } else if first_unit == "Meter".to_string() {
        if second_unit == "Mile".to_string() {
            println!("The result is {:.4}mi", (first_unit_value as f64 / 1000.0) / 1.6093)
        } else if second_unit == "Feet".to_string() {
            println!("The result is {:.2}ft", first_unit_value as f64 * 3.2808)
        } else if second_unit == "Inch".to_string() {
            println!("The result is {:.2}in", (first_unit_value as f64 * 3.2808) * 12.0)
        } else {
            println!("The result is {}m", first_unit_value)
        }
    } else if first_unit == "Feet".to_string() {
        if second_unit == "Mile".to_string() {
            println!("The result is {:.2}mi", ((first_unit_value as f64 / 3.2808) / 1000.0) / 1.6093)
        } else if second_unit == "Meter".to_string() {
            println!("The result is {:.2}m", first_unit_value as f64 / 3.2808)
        } else if second_unit == "Inch".to_string() {
            println!("The result is {:.2}in", first_unit_value as f64 * 12.0)
        } else {
            println!("The result is {}ft", first_unit_value)
        }
    } else if first_unit == "Inch".to_string() {
        if second_unit == "Mile".to_string() {
            println!("The result is {}mi", (((first_unit_value as f64 / 12.0) / 3.2808) / 1000.0)/1.6093 )
        } else if second_unit == "Meter".to_string() {
            println!("The result is {}m", (first_unit_value as f64 / 12.0) / 3.2808)
        } else if second_unit == "Feet".to_string() {
            println!("The result is {}ft", first_unit_value as f64 / 12.0)
        } else {
            println!("The result is {}in", first_unit_value)
        }
    }
    let confirmation = Confirm::new()
    .with_prompt("Do you want to convert again")
    .default(false)
    .wait_for_newline(true)
    .interact()
    .unwrap();

    if confirmation {
        term.clear_screen().unwrap();
        length(term)
    }
    term.clear_screen().unwrap();
}

fn weight(term: &Term) {
    let units = vec!["Gram", "Ounce", "Pound", "back"];
    let mut first_unit: String;
    let mut first_unit_value: f64;
    let mut second_unit: String;

    loop {
        let first_choice = Select::new()
        .with_prompt("Unit you want to convert from")
        .items(&units)
        .default(0)
        .interact()
        .unwrap();
        
        if first_choice == 3 {
            term.clear_screen().unwrap();
            return
        }

        first_unit = units[first_choice].to_string();
        first_unit_value = Input::new()
        .with_prompt("The value")
        .interact_text()
        .unwrap();
    
        let second_choice = Select::new()
        .with_prompt("Unit you want to convert to")
        .items(&units)
        .default(0)
        .interact()
        .unwrap();

        second_unit = units[second_choice].to_string();
    
        if second_choice == 3 {
            term.clear_screen().unwrap();
            continue
        }
        break
    }

    if first_unit == "Gram".to_string() {
        if second_unit == "Ounce".to_string() {
            println!("The result is {:.2}oz", first_unit_value as f64 / 28.3494)
        } else if second_unit == "Pound".to_string() {
            println!("The result is {:.2}lb", (first_unit_value as f64 / 1000.0) * 2.2046)
        } else {
            println!("The result is {}g", first_unit_value)
        }
    } else if first_unit == "Ounce".to_string() {
        if second_unit == "Gram".to_string() {
            println!("The result is {:.2}g", first_unit_value as f64 * 28.3494)
        } else if second_unit == "Pound".to_string() {
            println!("The result is {:.2}lb", first_unit_value as f64 / 16.0)
        } else {
            println!("The result is {}oz", first_unit_value)
        }
    } else if first_unit == "Pound".to_string() {
        if second_unit == "Gram".to_string() {
            println!("The result is {:.2}g", (first_unit_value as f64 / 2.2046 ) * 1000.0)
        } else if second_unit == "Ounce".to_string() {
            println!("The result is {:.2}oz", first_unit_value as f64 * 16.0)
        } else {
            println!("The result is {}lb", first_unit_value)
        }
    }
    let confirmation = Confirm::new()
    .with_prompt("Do you want to convert again")
    .default(false)
    .wait_for_newline(true)
    .interact()
    .unwrap();

    if confirmation {
        term.clear_screen().unwrap();
        weight(term)
    }
    term.clear_screen().unwrap();
}

fn main() {
    loop {
        let term = Term::stdout();

        let units = vec!["Temperature", "Length", "Weight", "exit"];
        let main_menu = Select::new()
        .with_prompt("Select kind of unit you want to convert")
        .items(&units)
        .default(0)
        .interact()
        .unwrap();

        term.clear_screen().unwrap();
    
        if main_menu == 0 {
            temperature(&term);
        } else if main_menu == 1 {
            length(&term);
        } else if main_menu == 2 {
            weight(&term)
        } else if main_menu == 3 {
            break;
        }
    }
}