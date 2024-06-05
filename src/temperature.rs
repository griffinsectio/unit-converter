use super::*;

pub struct Celsius {
    pub value: f64,
    pub symbol: String,
}

impl Celsius {
    fn new(value: f64, symbol: String) -> Self {
        Celsius {value, symbol}
    }
}

impl Unit for Celsius {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(unit: Fahrenheit) -> Self {
        Celsius {value: ((unit.value as f64 - 32.0) * 5.0) / 9.0, symbol: "C".to_string()}
    }
}

impl From<Kelvin> for Celsius {
    fn from(unit: Kelvin) -> Self {
        Celsius {value: unit.value as f64 - 273.15, symbol: "C".to_string()}
    }
}

pub struct Fahrenheit {
    pub value: f64,
    pub symbol: String,
}

impl Unit for Fahrenheit {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl Fahrenheit {
    fn new(value: f64, symbol: String) -> Self {
        Fahrenheit {value, symbol}
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(unit: Celsius) -> Self {
        Fahrenheit {value: ((unit.value as f64 * 9.0) / 5.0) + 32.0, symbol: "F".to_string()}
    }
}

impl From<Kelvin> for Fahrenheit {
    fn from(unit: Kelvin) -> Self {
        Fahrenheit {value: (((unit.value as f64 - 273.15) * 9.0) / 5.0) + 32.0, symbol: "F".to_string()}
    }
}

pub struct Kelvin {
    pub value: f64,
    pub symbol: String,
}

impl Unit for Kelvin {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl Kelvin {
    fn new(value: f64, symbol: String) -> Self {
        Kelvin {value, symbol}
    }
}

impl From<Celsius> for Kelvin {
    fn from(unit: Celsius) -> Self {
        Kelvin {value: unit.value as f64 + 273.15, symbol: "K".to_string()}
    }
}

impl From<Fahrenheit> for Kelvin {
    fn from(unit: Fahrenheit) -> Self {
        Kelvin {value: (((unit.value as f64 - 32.0) * 5.0) / 9.0) + 273.15, symbol: "K".to_string()}
    }
}

pub fn temperature(term: &Term) {
    let units = vec!["Celsius", "Fahrenheit", "Kelvin", "back"];
    let mut first_unit: String;
    let mut unit_value: f64;
    let mut second_unit: String;

    loop {
        first_unit = get_first_unit(&units);

        // If the user chose to go back, return to the main menu
        if first_unit == "back" {
            term.clear_screen().unwrap();
            return
        }

        // get value we want to convert from of the unit
        unit_value = get_value();

        second_unit = get_second_unit(&units);
    
        // If instead of unit, the user chose to back, then clear the terminal screen and then
        // prompt the user for the first unit again
        if second_unit == "back".to_string() {
            term.clear_screen().unwrap();
            continue
        }
        break
    }

    if first_unit == "Celsius".to_string() {
        let celsius = Celsius::new(unit_value, "C".to_string());

        if second_unit == "Fahrenheit".to_string() {
            Fahrenheit::from(celsius).show_value()
        } else if second_unit == "Kelvin".to_string() {
            Kelvin::from(celsius).show_value()
        } else {
            celsius.show_value()
        }
    } else if first_unit == "Fahrenheit".to_string() {
        let fahrenheit = Fahrenheit::new(unit_value, "F".to_string());

        if second_unit == "Celsius".to_string() {
            Celsius::from(fahrenheit).show_value()
        } else if second_unit == "Kelvin".to_string() {
            Kelvin::from(fahrenheit).show_value()
        } else {
            fahrenheit.show_value()
        }
    } else if first_unit == "Kelvin".to_string() {
        let kelvin = Kelvin::new(unit_value, "K".to_string());

        if second_unit == "Celsius".to_string() {
            Celsius::from(kelvin).show_value()
        } else if second_unit == "Fahrenheit".to_string() {
            Fahrenheit::from(kelvin).show_value()
        } else {
            kelvin.show_value()
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