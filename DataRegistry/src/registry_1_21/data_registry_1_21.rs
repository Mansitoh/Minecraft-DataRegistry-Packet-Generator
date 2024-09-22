

use crate::registry_1_21::trim_material::generate_default_trim_material;
use crate::utils::generate_input;

pub fn create_data_registry(){
    //clear console
    print!("\x1B[2J\x1B[1;1H");
    println!("Welcome to the DataRegistry generator for Minecraft 1.21.");
    println!("First, we will create the `trim_material` data registry.");
    print!("You want a custom data registry for `trim_material`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `trim_material` data registry.");
            generate_default_trim_material();
        },
        _ => {
            println!("Using Minecraft default `trim_material` data registry.");
            generate_default_trim_material();
        }
    }
}


