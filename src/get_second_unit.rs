use super::*;

pub fn get_second_unit(units: &Vec<&str>) -> String {
    let second_choice = Select::new()
    .with_prompt("Unit you want to convert to")
    .items(units)
    .default(0)
    .interact()
    .unwrap();

    // if second_choice == units.len() - 1 {
    //     term.clear_screen().unwrap();
    // }

    return units[second_choice].to_string();
}