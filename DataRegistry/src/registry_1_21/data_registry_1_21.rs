

use std::fs;

use serde_json::json;

use crate::registry_1_21::banner_pattern::generate_default_banner_pattern;
use crate::registry_1_21::chat_type::generate_default_chat_type;
use crate::registry_1_21::damage_type::generate_default_damage_type;
use crate::registry_1_21::dimension_type::generate_default_dimension_type;
use crate::registry_1_21::painting_variant::generate_default_painting_variant;
use crate::registry_1_21::trim_material::generate_default_trim_material;
use crate::registry_1_21::trim_pattern::generate_default_trim_pattern;
use crate::registry_1_21::wolf_variant::generate_default_wolf_variant;
use crate::registry_1_21::worldgen_biome::generate_default_worldgen_biome;
use crate::utils::{generate_input, read_json_files_from_path};

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
    println!("Data registry created successfully.");
    println!("Now you can find the data registry in the `Registries/1.21-Registry/created-packets/` folder.");
    println!("You can also find the jsons in the `Registries/1.21-Registry/jsons-created/` folder.");
    println!("");
    println!("Now we will create the `trim_pattern` data registry.");
    print!("You want a custom data registry for `trim_pattern`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `trim_pattern` data registry.");
            generate_default_trim_pattern();
        },
        _ => {
            println!("Using Minecraft default `trim_pattern` data registry.");
            generate_default_trim_pattern();
        }
    }
    println!("Data registry created successfully.");
    println!("Now you can find the data registry in the `Registries/1.21-Registry/created-packets/` folder.");
    println!("You can also find the jsons in the `Registries/1.21-Registry/jsons-created/` folder.");
    println!("");
    println!("Now we will create the `banner_pattern` data registry.");
    print!("You want a custom data registry for `banner_pattern`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `banner_pattern` data registry.");
            generate_default_banner_pattern();
        },
        _ => {
            println!("Using Minecraft default `banner_pattern` data registry.");
            generate_default_banner_pattern();
        }
    }
    println!("Data registry created successfully.");
    println!("Now you can find the data registry in the `Registries/1.21-Registry/created-packets/` folder.");
    println!("You can also find the jsons in the `Registries/1.21-Registry/jsons-created/` folder.");
    println!("");
    println!("Now we will create the `worldgen_biome` data registry.");
    print!("You want a custom data registry for `worldgen_biome`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `worldgen_biome` data registry.");
            generate_default_worldgen_biome();
        },
        _ => {
            println!("Using Minecraft default `worldgen_biome` data registry.");
            generate_default_worldgen_biome();
        }
    }
    print!("Data registry created successfully.");
    println!("Now you can find the data registry in the `Registries/1.21-Registry/created-packets/` folder.");
    println!("You can also find the jsons in the `Registries/1.21-Registry/jsons-created/` folder.");
    println!("");
    println!("Now we will create the `chat_type` data registry.");
    print!("You want a custom data registry for `chat_type`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `chat_type` data registry.");
            generate_default_chat_type();
        },
        _ => {
            println!("Using Minecraft default `chat_type` data registry.");
            generate_default_chat_type();
        }
    }
    print!("Data registry created successfully.");
    println!("Now you can find the data registry in the `Registries/1.21-Registry/created-packets/` folder.");
    println!("You can also find the jsons in the `Registries/1.21-Registry/jsons-created/` folder.");
    println!("");
    println!("Now we will create the `damage_type` data registry.");
    print!("You want a custom data registry for `damage_type`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `damage_type` data registry.");
            generate_default_damage_type();
        },
        _ => {
            println!("Using Minecraft default `damage_type` data registry.");
            generate_default_damage_type();
        }
    }
    println!("Data registry created successfully.");
    println!("Now you can find the data registry in the `Registries/1.21-Registry/created-packets/` folder.");
    println!("You can also find the jsons in the `Registries/1.21-Registry/jsons-created/` folder.");
    println!("");
    println!("Now we will create the `dimension_type` data registry.");
    print!("You want a custom data registry for `dimension_type`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `dimension_type` data registry.");
            generate_default_dimension_type();
        },
        _ => {
            println!("Using Minecraft default `dimension_type` data registry.");
            generate_default_dimension_type();
        }
    }
    println!("Data registry created successfully.");
    println!("Now you can find the data registry in the `Registries/1.21-Registry/created-packets/` folder.");
    println!("You can also find the jsons in the `Registries/1.21-Registry/jsons-created/` folder.");
    println!("");
    println!("Now we will create the `wolf_variant` data registry.");
    print!("You want a custom data registry for `wolf_variant`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `wolf_variant` data registry.");
            generate_default_wolf_variant();
        },
        _ => {
            println!("Using Minecraft default `wolf_variant` data registry.");
            generate_default_wolf_variant();
        }
    }
    println!("Data registry created successfully.");
    println!("Now you can find the data registry in the `Registries/1.21-Registry/created-packets/` folder.");
    println!("You can also find the jsons in the `Registries/1.21-Registry/jsons-created/` folder.");
    println!("");
    println!("Now we will create the `painting_variant` data registry.");
    print!("You want a custom data registry for `painting_variant`? (y [custom] / n [minecraft defualt]): ");
    match generate_input().trim().to_lowercase().as_str() {
        "y" => {
            println!("Under construction...");
        },
        "n" => {
            println!("Using Minecraft default `painting_variant` data registry.");
            generate_default_painting_variant();
        },
        _ => {
            println!("Using Minecraft default `painting_variant` data registry.");
            generate_default_painting_variant();
        }
    }
    
    println!("DataRegistry generator finished.");
    println!("Thank you for using the DataRegistry generator for Minecraft 1.21.");
    //for each json file in jsons-created folder, create a single json file with all the jsons
    let mut registry_json = json!({});
    for entry in fs::read_dir("Registries/1.21-Registry/jsons-created/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).unwrap();
            let json: serde_json::Value = serde_json::from_str(&content).unwrap();
            //add json to registry_json
            registry_json.as_object_mut().unwrap().extend(json.as_object().unwrap().clone());
        }
    }
    for entry in fs::read_dir("Registries/1.21-Registry/jsons-created/worldgen").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).unwrap();
            let json: serde_json::Value = serde_json::from_str(&content).unwrap();
            //add json to registry_json
            registry_json.as_object_mut().unwrap().extend(json.as_object().unwrap().clone());
        }
    }
    //write registry_json to a single json file
    let registry_json_str = serde_json::to_string_pretty(&registry_json).unwrap();
    fs::write("Registries/1.21-Registry/jsons-created/registry.json", registry_json_str).unwrap();
}


