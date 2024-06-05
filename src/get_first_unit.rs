use super::*;

pub fn get_first_unit(units: &Vec<&str>) -> String {
    let first_choice = Select::new()
    .with_prompt("Unit you want to convert from")
    .items(&units)
    .default(0)
    .interact()
    .unwrap();
    
    // if first_choice == units.len() - 1 {
    //     term.clear_screen().unwrap();
    // }
    return units[first_choice].to_string();
}