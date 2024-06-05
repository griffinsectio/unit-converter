use super::*;

pub fn get_value() -> f64 {
    loop {
        let unit_value: f64 = Input::new()
                    .with_prompt("The value")
                    .interact_text()
                    .unwrap();
                
        if unit_value > 0.0 {
            return unit_value;
        } else {
            println!("Please enter non-zero or non-negative value!")
        }
    }
}