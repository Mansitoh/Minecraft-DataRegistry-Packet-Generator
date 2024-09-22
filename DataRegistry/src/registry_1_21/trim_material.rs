use std::{fs, path::Path};

use crate::PacketBuilder;

pub fn generate_default_trim_material(){
    println!("");
    println!("Trim Material Data Registry");
    println!("Generating default `trim_material` data registry...");
    let mut packet = PacketBuilder::new(0x07);
    packet.write_identifier("minecraft:trim_material".to_string());
    
    let path = Path::new("../registries");
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if dir_name == "trim_material" {
                println!("`trim_material` data registry already exists.");
                return;
            }
        }
        println!("path: {:?}", path);
    }
}