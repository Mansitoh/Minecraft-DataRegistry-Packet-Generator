use std::fs;
use crab_nbt::{nbt, NbtTag};
use serde_json::{self, json, Value};

use crate::PacketBuilder;

pub fn generate_default_chat_type() {
    let registry_type = "chat_type";
    let data_dir_path = "../Registries/1.21-Registry/extracted-from-jar/".to_owned()+registry_type;
    let packets_dir = "../Registries/1.21-Registry/created-packets/".to_string()+registry_type;
    let jsons_dir = "../Registries/1.21-Registry/jsons-created/".to_string()+registry_type;
    println!("\nChatType Data Registry");
    println!("Generating default `{}` data registry...", registry_type);
    
    let mut packet = PacketBuilder::new(0x07);
    println!("Writing packet identifier...");
    packet.write_identifier("minecraft:".to_string()+registry_type);

    let number_of_entries = fs::read_dir(data_dir_path.clone())
        .unwrap()
        .count() as i32; 

    println!("Writing number of entries...");
    packet.write_varint(number_of_entries);
    println!("Writing entries...");
    
    let mut registry_json = json!({
        "minecraft:".to_owned()+registry_type: {} 
    });

    let mut object = registry_json["minecraft:".to_owned()+registry_type].as_object_mut().unwrap();

    for entry in fs::read_dir(data_dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).unwrap();
            let json: serde_json::Value = serde_json::from_str(&content).unwrap();
            let name = entry.file_name().into_string().unwrap().replace(".json", "");

            println!("  Writing entry: minecraft:{}", name);
            packet.write_identifier("minecraft:".to_string() + &name);
            println!("      Adding data to entry");
            packet.write_boolean(true);

            let chat_tanslate = json["chat"]["translation_key"].as_str().unwrap();
            let chat_parameters: Vec<String> = json["chat"]["parameters"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
            let narration_tanslate = json["narration"]["translation_key"].as_str().unwrap();
            let narration_parameters: Vec<String> = json["narration"]["parameters"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
            let nbt = generate_nbt(
                name.clone(),
                chat_tanslate.to_string(),
                chat_parameters.clone(),
                narration_tanslate.to_string(),
                narration_parameters.clone()
            );

            add_entry(
                &mut object,
                &("minecraft:".to_string() + &name),
                chat_tanslate.to_string(),
                chat_parameters,
                narration_tanslate.to_string(),
                narration_parameters
            );

            println!("      Writing NBT data...");
            packet.write_nbt(nbt);
            println!("      Successfully wrote entry: minecraft:{}", name);
        }
    }

    println!("Successfully wrote `{}` data registry.", registry_type);
    let packet_created = packet.build();
    println!("Writing packet to file...");
    fs::write(packets_dir+".data", packet_created).unwrap();
    println!("Successfully wrote `{}` data registry to file.",registry_type);
    fs::write(jsons_dir+".json", serde_json::to_string_pretty(&registry_json).unwrap()).unwrap();
}

pub fn generate_nbt(
    nbt_name: String,
    chat_tanslate: String,
    chat_parameters: Vec<String>,
    narration_tanslate: String,
    narration_parameters: Vec<String>
) -> crab_nbt::Nbt {
    let chat_parameters_tag: Vec<NbtTag> = chat_parameters.iter().map(|p| NbtTag::String(p.clone())).collect();
    let narration_parameters_tag: Vec<NbtTag> = narration_parameters.iter().map(|p| NbtTag::String(p.clone())).collect();
    let nbt = nbt!(nbt_name, {
        "chat": {
            "translation_key": chat_tanslate,
            "parameters": chat_parameters_tag
        },
        "narration": {
            "translation_key": narration_tanslate,
            "parameters": narration_parameters_tag
        }
    });
    nbt
}

fn add_entry(
    object: &mut serde_json::Map<String, Value>, 
    identifier: &str,
    chat_tanslate: String,
    chat_parameters: Vec<String>,
    narration_tanslate: String,
    narration_parameters: Vec<String>
) {
    let entry = json!({
        "chat": {
            "translation_key": chat_tanslate,
            "parameters": chat_parameters
        },
        "narration": {
            "translation_key": narration_tanslate,
            "parameters": narration_parameters
        }
    });
    object.insert(identifier.to_string(), entry);
}
