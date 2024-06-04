use super::*;

pub struct Kilogram {
    pub value: f64,
    pub symbol: String,
}


impl Kilogram {
    fn new(value: f64, symbol: String) -> Self {
        Kilogram {value, symbol}
    }
}

impl Unit for Kilogram {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl From<Gram> for Kilogram {
    fn from(unit: Gram) -> Self {
        Kilogram::new(unit.value / 1000.0, "Kg".to_string())
    }
}

impl From<Ounce> for Kilogram {
    fn from(unit: Ounce) -> Self {
        Kilogram::new(unit.value / 35.274, "Kg".to_string())
    }
}

impl From<Pound> for Kilogram {
    fn from(unit: Pound) -> Self {
        Kilogram::new(unit.value / 2.205, "Kg".to_string())
    }
}

pub struct Gram {
    pub value: f64,
    pub symbol: String,
}

impl Gram {
    fn new(value: f64, symbol: String) -> Self {
        Gram {value, symbol}
    }
}

impl Unit for Gram {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl From<Kilogram> for Gram {
    fn from(unit: Kilogram) -> Self {
        Gram::new(unit.value * 1000.0, "g".to_string())
    }
}

impl From<Ounce> for Gram {
    fn from(unit: Ounce) -> Self {
        Gram::new(unit.value * 28.35, "g".to_string())
    }
}

impl From<Pound> for Gram {
    fn from(unit: Pound) -> Self {
        Gram::new(unit.value * 453.592, "g".to_string())
    }
}

pub struct Ounce {
    pub value: f64,
    pub symbol: String,
}

impl Ounce {
    fn new(value: f64, symbol: String) -> Self {
        Ounce {value, symbol}
    }
}

impl Unit for Ounce {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl From<Kilogram> for Ounce {
    fn from(unit: Kilogram) -> Self {
        Ounce::new(unit.value * 35.274, "oz".to_string())
    }
}

impl From<Gram> for Ounce {
    fn from(unit: Gram) -> Self {
        Ounce::new(unit.value / 28.35, "oz".to_string())
    }
}

impl From<Pound> for Ounce {
    fn from(unit: Pound) -> Self {
        Ounce::new(unit.value * 16.0, "oz".to_string())
    }
}

pub struct Pound {
    pub value: f64,
    pub symbol: String,
}

impl Pound {
    fn new(value: f64, symbol: String) -> Self {
        Pound {value, symbol }
    }
}

impl Unit for Pound {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl From<Kilogram> for Pound {
    fn from(unit: Kilogram) -> Self {
        Pound::new(unit.value * 2.205, "lb".to_string())
    }
}

impl From<Gram> for Pound {
    fn from(unit: Gram) -> Self {
        Pound::new(unit.value / 453.592, "lb".to_string())
    }
}

impl From<Ounce> for Pound {
    fn from(unit: Ounce) -> Self {
        Pound::new(unit.value / 16.0, "lb".to_string())
    }
}

pub fn weight(term: &Term) {
    let units = vec!["Kilogram", "Gram", "Ounce", "Pound", "back"];
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
        
        if first_choice == units.len() - 1 {
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
    
        if second_choice == units.len() - 1 {
            term.clear_screen().unwrap();
            continue
        }
        break
    }

    if first_unit == "Kilogram".to_string() {
        let kilogram = Kilogram::new(unit_value, "kg".to_string());
        if second_unit == "Gram".to_string() {
            Gram::from(kilogram).show_conversion_result();
        } else if second_unit == "Ounce".to_string() {
            Ounce::from(kilogram).show_conversion_result();
        } else if second_unit == "Pound".to_string() {
            Pound::from(kilogram).show_conversion_result();
        } else {
            println!("The result is {:.2}{}", kilogram.value, kilogram.symbol)
        }
    } else if first_unit == "Gram".to_string() {
        let gram = Gram::new(unit_value, "g".to_string());
        if second_unit == "Kilogram".to_string() {
            Kilogram::from(gram).show_conversion_result();
        } else if second_unit == "Ounce".to_string() {
            Ounce::from(gram).show_conversion_result();
        } else if second_unit == "Pound".to_string() {
            Pound::from(gram).show_conversion_result();
        } else {
            println!("The result is {:.2}{}", gram.value, gram.symbol)
        }
    } else if first_unit == "Ounce".to_string() {
        let ounce = Ounce::new(unit_value, "oz".to_string());
        if second_unit == "Kilogram" {
            Kilogram::from(ounce).show_conversion_result()
        } else if second_unit == "Gram".to_string() {
            Gram::from(ounce).show_conversion_result()
        } else if second_unit == "Pound".to_string() {
            Pound::from(ounce).show_conversion_result()
        } else {
            println!("The result is {:.2}{}", ounce.value, ounce.symbol)
        }
    } else if first_unit == "Pound".to_string() {
        let pound = Pound::new(unit_value, "lb".to_string());
        if second_unit == "Kilogram".to_string() {
            Kilogram::from(pound).show_conversion_result()
        } else if second_unit == "Gram".to_string() {
            Gram::from(pound).show_conversion_result()
        } else if second_unit == "Ounce".to_string() {
            Ounce::from(pound).show_conversion_result()
        } else {
            println!("The result is {}lb", unit_value)
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
