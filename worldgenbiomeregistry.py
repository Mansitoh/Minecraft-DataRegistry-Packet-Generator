import os
import json
from nbt.nbt import *
from packetbuilder import PacketBuilder

def create_biome(name, has_precipitation, temperature, downfall, effects):
    biome = NBTFile()
    biome.name = name
    
    biome.tags.append(TAG_Byte(name="has_precipitation", value=has_precipitation))
    biome.tags.append(TAG_Float(name="temperature", value=temperature))
    biome.tags.append(TAG_Float(name="downfall", value=downfall))
    
    effects_tag = TAG_Compound(name="effects") 
    for key, value in effects.items():
        effects_tag.tags.append(value)  
    biome.tags.append(effects_tag)
    
    return biome

if __name__ == "__main__":
    packet_id = 7 
    builder = PacketBuilder(packet_id)
    builder.write_identifier("minecraft:worldgen/biome")
    i = 0
    biome_folder = '1.21-registry/extracted-from-jar/worldgen/biome'
    
    for filename in os.listdir(biome_folder):
        if filename.endswith('.json'):
            i += 1
    
    builder.write_varint(0)
    
    for filename in os.listdir(biome_folder):
        if filename.endswith('.json'):
            file_path = os.path.join(biome_folder, filename)
            with open(file_path, 'r') as file:
                biome_data = json.load(file)
                builder.write_identifier("minecraft:"+filename.replace('.json',''))
                builder.write_boolean(True)
                
                effects = {
                    "sky_color": TAG_Int(name="sky_color", value=biome_data["effects"]["sky_color"]),
                    "water_fog_color": TAG_Int(name="water_fog_color", value=biome_data["effects"]["water_fog_color"]),
                    "fog_color": TAG_Int(name="fog_color", value=biome_data["effects"]["fog_color"]),
                    "water_color": TAG_Int(name="water_color", value=biome_data["effects"]["water_color"]),
                }
                
                biome = create_biome(
                    name=filename.replace('.json',''),
                    has_precipitation=biome_data["has_precipitation"],
                    temperature=biome_data["temperature"],
                    downfall=biome_data["downfall"],
                    effects=effects
                )
                builder.write_nbt(biome)
              
    packet = builder.build()
    print("Packet Created Successfully!")
    print(" ")
    print(packet)
    builder.save_to_file("1.21-registry/created-packets/biome-registry-packet-1.21.dat")
