mod temperature;
use temperature::*;
mod length;
use length::*;
mod weight;
use weight::*;

use dialoguer::{Confirm, Input, Select};
use console::Term;

trait Unit {
    fn value(&self) -> f64;
    fn symbol(&self) -> String;
    fn show_value(&self) {
        println!("The value is {:.2}{}", self.value(), self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_to_c() {
        let fahrenheit = Fahrenheit {
            value: 50.5,
            symbol: "F".to_string(),
        };
        let conversion_result = Celsius::from(fahrenheit);
        assert_eq!("10.28C", format!("{:.2}{}", conversion_result.value, conversion_result.symbol))
    }
    
    #[test]
    fn k_to_c() {
        let kelvin = Kelvin {
            value: 325.5,
            symbol: "K".to_string(),
        };
        let conversion_result = Celsius::from(kelvin);
        assert_eq!("52.35C", format!("{:.2}{}", conversion_result.value, conversion_result.symbol))
    }

    #[test]
    fn c_to_f() {
        let celsius = Celsius {
            value: 87.6,
            symbol: "C".to_string(),
        };
        let conversion_result = Fahrenheit::from(celsius);
        assert_eq!("189.68F", format!("{:.2}{}", conversion_result.value, conversion_result.symbol))
    }

    #[test]
    fn k_to_f() {
        let kelvin = Kelvin {
            value: 325.5,
            symbol: "K".to_string(),
        };
        let conversion_result = Fahrenheit::from(kelvin);
        assert_eq!("126.23F", format!("{:.2}{}", conversion_result.value, conversion_result.symbol))
    }

    #[test]
    fn c_to_k() {
        let celsius = Celsius {
            value: 28.15,
            symbol: "C".to_string()
        }; 
        let conversion_result = Kelvin::from(celsius);
        assert_eq!("301.30K", format!("{:.2}{}", conversion_result.value, conversion_result.symbol))
    }

    #[test]
    fn f_to_k() {
        let fahrenheit = Fahrenheit {
            value: 180.23,
            symbol: "F".to_string()
        }; 
        let conversion_result = Kelvin::from(fahrenheit);
        assert_eq!("355.50K", format!("{:.2}{}", conversion_result.value, conversion_result.symbol))
    }
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