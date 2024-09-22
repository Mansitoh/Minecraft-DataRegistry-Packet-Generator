use std::fs;
use crab_nbt::{nbt, NbtCompound, NbtTag};
use serde_json::{self, json, Value};

use crate::PacketBuilder;

pub fn generate_default_worldgen_biome() {
    let registry_type = "worldgen/biome";
    let data_dir_path = "../Registries/1.21-Registry/extracted-from-jar/".to_owned()+registry_type;
    let packets_dir = "../Registries/1.21-Registry/created-packets/".to_string()+registry_type;
    let jsons_dir = "../Registries/1.21-Registry/jsons-created/".to_string()+registry_type;
    println!("\nWorldGen Biome Data Registry");
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

            let has_precipitation = json["has_precipitation"].as_bool().unwrap();
            let temperature = json["temperature"].as_f64().unwrap() as f32;
            let downfall = json["downfall"].as_f64().unwrap() as f32;
            let effects = json["effects"].clone();

            let nbt = generate_nbt(
                name.clone(),
                has_precipitation,
                temperature,
                downfall,
                effects.clone(), // Clone the effects value here
            );

            add_entry(
                &mut object,
                &("minecraft:".to_string() + &name),
                has_precipitation,
                temperature,
                downfall,
                effects,
                
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
    has_precipitation: bool,
    temperature: f32,
    downfall: f32,
    effects: Value, 
) -> crab_nbt::Nbt {
    let effects_nbt = if effects.is_object() {
        let mut compound = NbtCompound::new();
        for (key, value) in effects.as_object().unwrap() {
            let tag = match value {
                Value::Bool(b) => NbtTag::from(*b),
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        NbtTag::from(i)
                    } else {
                        NbtTag::from(n.as_f64().unwrap_or(0.0))
                    }
                }
                Value::String(s) => NbtTag::from(s.as_str()),
                _ => continue,
            };
            compound.child_tags.insert(key.clone(), tag);
        }
        NbtTag::Compound(compound)
    } else {
        NbtTag::Compound(NbtCompound::new()) 
    };

    let nbt = nbt!(nbt_name, {
        "has_precipitation": has_precipitation,
        "temperature": temperature,
        "downfall": downfall,
        "effects": effects_nbt,
    });
    nbt
}
fn add_entry(
    object: &mut serde_json::Map<String, Value>, 
    identifier: &str,
    has_precipitation: bool,
    temperature: f32,
    downfall: f32,
    effects: serde_json::Value,
) {
    let entry = json!({
        "has_precipitation": has_precipitation,
        "temperature": temperature,
        "downfall": downfall,
        "effects": effects,
    });
    object.insert(identifier.to_string(), entry);
}
