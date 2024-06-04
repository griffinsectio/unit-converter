use super::*;

pub fn weight(term: &Term) {
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
        
        if first_choice == units.len() - 1 {
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
    
        if second_choice == units.len() - 1 {
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