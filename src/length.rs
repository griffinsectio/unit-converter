use super::*;

pub struct Mile {
    pub value: f64,
    pub symbol: String,
}

impl Mile {
    fn new(value: f64, symbol: String) -> Self {
        Mile {
            value,
            symbol
        }
    }
}

impl Unit for Mile {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl From<Kilometer> for Mile {
    fn from(unit: Kilometer) -> Self {
        Mile {
            value: unit.value / 1.6093,
            symbol: "mi".to_string(),
        }
    }
}

impl From<Meter> for Mile {
    fn from(unit: Meter) -> Self {
        Mile {
            value: Kilometer::from(unit).value / 1.6093,
            symbol: "mi".to_string(),
        }
    }
}

impl From<Feet> for Mile {
    fn from(unit: Feet) -> Self {
        Mile {
            value: unit.value / 5280.0,
            symbol: "mi".to_string(),
        }
    }
}

impl From<Inch> for Mile {
    fn from(unit: Inch) -> Self {
        Mile {
            value: unit.value / 63360.0,
            symbol: "mi".to_string(),
        }
    }
}

pub struct Kilometer {
    pub value: f64,
    pub symbol: String,
}

impl Unit for Kilometer {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl Kilometer {
    fn new(value: f64, symbol: String) -> Self {
        Kilometer {
            value,
            symbol
        }
    }
}

impl From<Mile> for Kilometer {
    fn from(unit: Mile) -> Self {
        Kilometer {
            value: unit.value * 1.6093,
            symbol: "km".to_string(),
        }
    }
}

impl From<Meter> for Kilometer {
    fn from(unit: Meter) -> Self {
        Kilometer {
            value: unit.value / 1000.0,
            symbol: "km".to_string(),
        }
    }
}

impl From<Feet> for Kilometer {
    fn from(unit: Feet) -> Self {
        Kilometer {
            value: unit.value / 3281.0,
            symbol: "km".to_string(),
        }
    }
}

impl From<Inch> for Kilometer {
    fn from(unit: Inch) -> Self {
        Kilometer {
            value: unit.value / 39370.0,
            symbol: "km".to_string(),
        }
    }
}

pub struct Meter {
    pub value: f64,
    pub symbol: String,
}

impl Unit for Meter {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl Meter {
    fn new(value: f64, symbol: String) -> Self {
        Meter {
            value,
            symbol
        }
    }
}

impl From<Mile> for Meter {
    fn from(unit: Mile) -> Self {
        Meter {
            value: unit.value * 1609.0,
            symbol: "m".to_string(),
        }
    }
}

impl From<Kilometer> for Meter {
    fn from(unit: Kilometer) -> Self {
        Meter {
            value: unit.value * 1000.0,
            symbol: "m".to_string(),
        }
    }
}

impl From<Feet> for Meter {
    fn from(unit: Feet) -> Self {
        Meter {
            value: unit.value / 3.2808,
            symbol: "m".to_string(),
        }
    }
}

impl From<Inch> for Meter {
    fn from(unit: Inch) -> Self {
        Meter {
            value: unit.value / 39.372,
            symbol: "m".to_string(),
        }
    }
}

pub struct Feet {
    pub value: f64,
    pub symbol: String,
}

impl Unit for Feet {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl Feet {
    fn new(value: f64, symbol: String) -> Self {
        Feet {
            value,
            symbol
        }
    }
}

impl From<Mile> for Feet {
    fn from(unit: Mile) -> Self {
        Feet {
            value: unit.value * 5280.0,
            symbol: "ft".to_string(),
        }
    }
}

impl From<Kilometer> for Feet {
    fn from(unit: Kilometer) -> Self {
        Feet {
            value: unit.value * 3280.84,
            symbol: "ft".to_string(),
        }
    }
}
impl From<Meter> for Feet {
    fn from(unit: Meter) -> Self {
        Feet {
            value: unit.value * 3.2808,
            symbol: "ft".to_string(),
        }
    }
}

impl From<Inch> for Feet {
    fn from(unit: Inch) -> Self {
        Feet {
            value: unit.value / 12.0,
            symbol: "ft".to_string(),
        }
    }
}

pub struct Inch {
    pub value: f64,
    pub symbol: String,
}

impl Unit for Inch {
    fn value(&self) -> f64 {
        self.value
    }
    fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}

impl Inch {
    fn new(value: f64, symbol: String) -> Self {
        Inch {
            value,
            symbol
        }
    }
}

impl From<Mile> for Inch {
    fn from(unit: Mile) -> Self {
        Inch {
            value: unit.value * 63360.0,
            symbol: "in".to_string(),
        }
    }
}

impl From<Kilometer> for Inch {
    fn from(unit: Kilometer) -> Self {
        Inch {
            value: unit.value * 39370.1,
            symbol: "in".to_string(),
        }
    }
}

impl From<Meter> for Inch {
    fn from(unit: Meter) -> Self {
        Inch {
            value: unit.value * 39.37,
            symbol: "in".to_string(),
        }
    }
}

impl From<Feet> for Inch {
    fn from(unit: Feet) -> Self {
        Inch {
            value: unit.value * 12.0,
            symbol: "in".to_string(),
        }
    }
}


pub fn length(term: &Term) {
    let units = vec!["Mile", "Kilometer", "Meter", "Feet", "Inch", "back"];
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

    if first_unit == "Mile".to_string() {
        let mile = Mile::new(unit_value, "mi".to_string());

        if second_unit == "Kilometer".to_string() {
            Kilometer::from(mile).show_value()
        } else if second_unit == "Meter".to_string() {
            Meter::from(mile).show_value()
        } else if second_unit == "Feet".to_string() {
            Feet::from(mile).show_value()
        } else if second_unit == "Inch".to_string() {
            Inch::from(mile).show_value()
        } else {
            mile.show_value()
        }
    } else if first_unit == "Kilometer"{
        let kilometer = Kilometer::new(unit_value, "km".to_string());
        if second_unit == "Mile".to_string() {
            Mile::from(kilometer).show_value()
        } else if second_unit == "Meter".to_string() {
            Meter::from(kilometer).show_value()
        } else if second_unit == "Feet".to_string() {
            Feet::from(kilometer).show_value()
        } else if second_unit == "Inch".to_string() {
            Inch::from(kilometer).show_value()
        } else {
            kilometer.show_value()
        }
    } else if first_unit == "Meter".to_string() {
        let meter = Meter::new(unit_value, "km".to_string());

        if second_unit == "Mile".to_string() {
            Mile::from(meter).show_value()
        } else if second_unit == "Kilometer".to_string() {
            Kilometer::from(meter).show_value()
        } else if second_unit == "Feet".to_string() {
            Feet::from(meter).show_value()
        } else if second_unit == "Inch".to_string() {
            Inch::from(meter).show_value()
        } else {
            meter.show_value()
        }
    } else if first_unit == "Feet".to_string() {
        let feet = Feet::new(unit_value, "km".to_string());

        if second_unit == "Mile".to_string() {
            Mile::from(feet).show_value()
        } else if second_unit == "Kilometer".to_string() {
            Kilometer::from(feet).show_value()
        } else if second_unit == "Meter".to_string() {
            Meter::from(feet).show_value()
        } else if second_unit == "Inch".to_string() {
            Inch::from(feet).show_value()
        } else {
            feet.show_value()
        }
    } else if first_unit == "Inch".to_string() {
        let inch = Inch::new(unit_value, "km".to_string());

        if second_unit == "Mile".to_string() {
            Mile::from(inch).show_value()
        } else if second_unit == "Kilometer".to_string() {
            Kilometer::from(inch).show_value()
        } else if second_unit == "Meter".to_string() {
            Meter::from(inch).show_value()
        } else if second_unit == "Feet".to_string() {
            Feet::from(inch).show_value()
        } else {
            inch.show_value()
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