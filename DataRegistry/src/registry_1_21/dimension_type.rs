use std::fs;
use crab_nbt::nbt;
use serde_json::{self, json, Value};

use crate::PacketBuilder;

pub fn generate_default_dimension_type() {
    let registry_type = "dimension_type";
    let data_dir_path = "../Registries/1.21-Registry/extracted-from-jar/".to_owned()+registry_type;
    let packets_dir = "../Registries/1.21-Registry/created-packets/".to_string()+registry_type;
    let jsons_dir = "../Registries/1.21-Registry/jsons-created/".to_string()+registry_type;
    println!("\nDimensionType Data Registry");
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


            let has_skylight = if json["has_skylight"].as_bool().unwrap(){ 1 } else { 0 };
            let has_ceiling = if json["has_ceiling"].as_bool().unwrap(){ 1 } else { 0 };
            let ultrawarm = if json["ultrawarm"].as_bool().unwrap() { 1 } else { 0 };
            let coordinate_scale = json["coordinate_scale"].as_f64().unwrap();
            let bed_works =if json["bed_works"].as_bool().unwrap(){ 1 } else { 0 };
            let respan_anchor_works = if json["respawn_anchor_works"].as_bool().unwrap(){ 1 } else { 0 };
            let min_y = json["min_y"].as_i64().unwrap() as i32;
            let height = json["height"].as_i64().unwrap() as i32;
            let logical_height = json["logical_height"].as_i64().unwrap() as i32;
            let infiniburn = json["infiniburn"].as_str().unwrap();
            let effects = json["effects"].as_str().unwrap();
            let ambient_light = json["ambient_light"].as_f64().unwrap() as f32;
            let piglin_safe = if json["piglin_safe"].as_bool().unwrap(){ 1 } else { 0 };
            let has_raids = if json["has_raids"].as_bool().unwrap(){ 1 } else { 0 };
            let monster_spawn_light_level_type = json["monster_spawn_light_level"]["type"].as_str().unwrap();
            let monster_spawn_light_level_max_inclusie = json["monster_spawn_light_level"]["max_inclusive"].as_i64().unwrap() as i32;
            let monster_spawn_light_level_min_inclusive = json["monster_spawn_light_level"]["min_inclusive"].as_i64().unwrap() as i32;
            let monster_spawn_block_light_limit = json["monster_spawn_block_light_limit"].as_i64().unwrap() as i32;


            let nbt = generate_nbt(
                name.clone(),
                has_skylight ,
                has_ceiling,
                ultrawarm,
                coordinate_scale,
                bed_works,
                respan_anchor_works,
                min_y,
                height,
                logical_height,
                infiniburn.to_string(),
                effects.to_string(),
                ambient_light,
                piglin_safe,
                has_raids,
                monster_spawn_light_level_type.to_string(),
                monster_spawn_light_level_max_inclusie,
                monster_spawn_light_level_min_inclusive,
                monster_spawn_block_light_limit
            );

            add_entry(
                &mut object,
                &("minecraft:".to_string() + &name),
                has_skylight,
                has_ceiling,
                ultrawarm,
                coordinate_scale,
                bed_works,
                respan_anchor_works,
                min_y,
                height,
                logical_height,
                infiniburn.to_string(),
                effects.to_string(),
                ambient_light,
                piglin_safe,
                has_raids,
                monster_spawn_light_level_type.to_string(),
                monster_spawn_light_level_max_inclusie,
                monster_spawn_light_level_min_inclusive,
                monster_spawn_block_light_limit
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
    has_skylight: i8,
    has_ceiling: i8,
    ultrawarm: i8,
    coordinate_scale: f64,
    bed_works: i8,
    respan_anchor_works: i8,
    min_y: i32,
    height: i32,
    logical_height: i32,
    infiniburn: String,
    effects: String,
    ambient_light: f32,
    piglin_safe: i8,
    has_raids: i8,
    monster_spawn_light_level_type: String,
    monster_spawn_light_level_max_inclusie: i32,
    monster_spawn_light_level_min_inclusive: i32,
    monster_spawn_block_light_limit: i32,
) -> crab_nbt::Nbt {
    let nbt = nbt!(nbt_name, {
        "has_skylight": has_skylight,
        "has_ceiling": has_ceiling,
        "ultrawarm": ultrawarm,
        "coordinate_scale": coordinate_scale,
        "bed_works": bed_works,
        "respawn_anchor_works": respan_anchor_works,
        "min_y": min_y,
        "height": height,
        "logical_height": logical_height,
        "infiniburn": infiniburn,
        "effects": effects,
        "ambient_light": ambient_light,
        "piglin_safe": piglin_safe,
        "has_raids": has_raids,
        "monster_spawn_light_level": {
            "type": monster_spawn_light_level_type,
            "max_inclusive": monster_spawn_light_level_max_inclusie,
            "min_inclusive": monster_spawn_light_level_min_inclusive,
        },
        "monster_spawn_block_light_limit": monster_spawn_block_light_limit,
    });
    nbt
}

fn add_entry(
    object: &mut serde_json::Map<String, Value>, 
    identifier: &str,

    has_skylight: i8,
    has_ceiling: i8,
    ultrawarm: i8,
    coordinate_scale: f64,
    bed_works: i8,
    respan_anchor_works: i8,
    min_y: i32,
    height: i32,
    logical_height: i32,
    infiniburn: String,
    effects: String,
    ambient_light: f32,
    piglin_safe: i8,
    has_raids: i8,
    monster_spawn_light_level_type: String,
    monster_spawn_light_level_max_inclusie: i32,
    monster_spawn_light_level_min_inclusive: i32,
    monster_spawn_block_light_limit: i32,
) {
    let entry = json!({
        "has_skylight": has_skylight,
        "has_ceiling": has_ceiling,
        "ultrawarm": ultrawarm,
        "coordinate_scale": coordinate_scale,
        "bed_works": bed_works,
        "respawn_anchor_works": respan_anchor_works,
        "min_y": min_y,
        "height": height,
        "logical_height": logical_height,
        "infiniburn": infiniburn,
        "effects": effects,
        "ambient_light": ambient_light,
        "piglin_safe": piglin_safe,
        "has_raids": has_raids,
        "monster_spawn_light_level": {
            "type": monster_spawn_light_level_type,
            "max_inclusive": monster_spawn_light_level_max_inclusie,
            "min_inclusive": monster_spawn_light_level_min_inclusive,
        },
        "monster_spawn_block_light_limit": monster_spawn_block_light_limit,
    });
    object.insert(identifier.to_string(), entry);
}
