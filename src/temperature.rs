use super::*;

pub struct Celsius {
    pub value: f64,
    pub symbol: String,
}

impl Celsius {
    fn new(value: f64, symbol: String) -> Self {
        Celsius {
            value,
            symbol
        }
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(unit: Fahrenheit) -> Self {
        Celsius {
            value: ((unit.value as f64 - 32.0) * 5.0) / 9.0,
            symbol: "C".to_string(),
        }
    }
}

impl From<Kelvin> for Celsius {
    fn from(unit: Kelvin) -> Self {
        Celsius {
            value: unit.value as f64 - 273.15,
            symbol: "C".to_string(),
        }
    }
}

pub struct Fahrenheit {
    pub value: f64,
    pub symbol: String,
}

impl Fahrenheit {
    fn new(value: f64, symbol: String) -> Self {
        Fahrenheit {
            value,
            symbol
        }
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(unit: Celsius) -> Self {
        Fahrenheit {
            value: ((unit.value as f64 * 9.0) / 5.0) + 32.0,
            symbol: "F".to_string(),
        }
    }
}

impl From<Kelvin> for Fahrenheit {
    fn from(unit: Kelvin) -> Self {
        Fahrenheit {
            value: (((unit.value as f64 - 273.15) * 9.0) / 5.0) + 32.0,
            symbol: "F".to_string(),
        }
    }
}

pub struct Kelvin {
    pub value: f64,
    pub symbol: String,
}

impl Kelvin {
    fn new(value: f64, symbol: String) -> Self {
        Kelvin {
            value,
            symbol
        }
    }
}

impl From<Celsius> for Kelvin {
    fn from(unit: Celsius) -> Self {
        Kelvin {
            value: unit.value as f64 + 273.15,
            symbol: "K".to_string()
        }
    }
}

impl From<Fahrenheit> for Kelvin {
    fn from(unit: Fahrenheit) -> Self {
        Kelvin {
            value: (((unit.value as f64 - 32.0) * 5.0) / 9.0) + 273.15,
            symbol: "K".to_string()
        }
    }
}

pub fn temperature(term: &Term) {
    let units = vec!["Celsius", "Fahrenheit", "Kelvin", "back"];
    let mut first_unit: String;
    let mut unit_value: f64;
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
        unit_value = Input::new()
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

    if first_unit == "Celsius".to_string() {
        let celsius = Celsius::new(unit_value, "C".to_string());
        if second_unit == "Fahrenheit".to_string() {

            let fahrenheit = Fahrenheit::from(celsius);
            println!("The result is {:.2}{}", fahrenheit.value, fahrenheit.symbol);

        } else if second_unit == "Kelvin".to_string() {

            let kelvin = Kelvin::from(celsius);
            println!("The result is {:.2}{}", kelvin.value, kelvin.symbol)

        } else {
            println!("The result is {}{}", celsius.value, celsius.symbol)
        }
    } else if first_unit == "Fahrenheit".to_string() {
        let fahrenheit = Fahrenheit::new(unit_value, "F".to_string());
        if second_unit == "Celsius".to_string() {

            let celsius = Celsius::from(fahrenheit);
            println!("The result is {:.2}{}", celsius.value, celsius.symbol)

        } else if second_unit == "Kelvin".to_string() {

            let kelvin = Kelvin::from(fahrenheit);
            println!("The result is {:.2}{}", kelvin.value, kelvin.symbol)

        } else {
            println!("The result is {}{}", fahrenheit.value, fahrenheit.symbol)
        }
    } else if first_unit == "Kelvin".to_string() {
        let kelvin = Kelvin::new(unit_value, "K".to_string());
        if second_unit == "Celsius".to_string() {

            let celsius = Celsius::from(kelvin);
            println!("The result is {:.2}{}", celsius.value, celsius.symbol);

        } else if second_unit == "Fahrenheit".to_string() {

            let fahrenheit = Fahrenheit::from(kelvin);
            println!("The result is {:.2}{}", fahrenheit.value, fahrenheit.symbol)

        } else {
            println!("The result is {:.2}{}", kelvin.value, kelvin.symbol)
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