use std::fs;
use crab_nbt::nbt;
use serde_json::{self, json, Value};

use crate::PacketBuilder;

pub fn generate_default_trim_pattern() {
    let registry_type = "trim_pattern";
    let data_dir_path = "../Registries/1.21-Registry/extracted-from-jar/".to_owned()+registry_type;
    let packets_dir = "../Registries/1.21-Registry/created-packets/".to_string()+registry_type;
    let jsons_dir = "../Registries/1.21-Registry/jsons-created/".to_string()+registry_type;
    println!("\nTrim Pattern Data Registry");
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

            let asset_id = json["asset_id"].as_str().unwrap();
            let template_item = json["template_item"].as_str().unwrap();
            let description_translate = json["description"]["translate"].as_str().unwrap();
            

            let nbt = generate_nbt(
                name.clone(),
                asset_id.to_string(),
                template_item.to_string(),
                description_translate.to_string()
            );

            add_entry(
                &mut object,
                &("minecraft:".to_string() + &name),
                asset_id.to_string(),
                template_item.to_string(),
                description_translate.to_string()
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
    asset_id: String,
    template_item: String,
    description_translate: String
) -> crab_nbt::Nbt {
    let nbt = nbt!(nbt_name, {
        "asset_id": asset_id,
        "template_item": template_item,
        "description": {
            "translate": description_translate,
        }
    });
    nbt
}

fn add_entry(
    object: &mut serde_json::Map<String, Value>, 
    identifier: &str,
    asset_id: String,
    template_item: String,
    description_translate: String
) {
    let entry = json!({
        "asset_id": asset_id,
        "template_item": template_item,
        "description": {
            "translate": description_translate,
        }
    });
    object.insert(identifier.to_string(), entry);
}
