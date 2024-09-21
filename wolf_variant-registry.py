import os
import json
from nbt.nbt import *
from packetbuilder import PacketBuilder

def create_data(name,wild_texture,tame_texture,angry_texture,biomes):
    nbt = NBTFile()
    nbt.name = name
    
    nbt.tags.append(TAG_String(name="wild_texture", value=wild_texture))
    nbt.tags.append(TAG_String(name="tame_texture", value=tame_texture))
    nbt.tags.append(TAG_String(name="angry_texture", value=angry_texture))
    nbt.tags.append(TAG_String(name="biomes", value=biomes))
    return nbt

if __name__ == "__main__":
    packet_id = 7 
    builder = PacketBuilder(packet_id)
    builder.write_identifier("minecraft:wolf_variant")
    i = 0
    biome_folder = '1.21-registry/extracted-from-jar/wolf_variant'
    
    for filename in os.listdir(biome_folder):
        if filename.endswith('.json'):
            i += 1
    
    builder.write_varint(0)
    
    for filename in os.listdir(biome_folder):
        if filename.endswith('.json'):
            file_path = os.path.join(biome_folder, filename)
            with open(file_path, 'r') as file:
                jsonData = json.load(file)
                builder.write_identifier("minecraft:"+filename.replace('.json',''))
                builder.write_boolean(True)
                nbt = create_data(
                    name=filename.replace('.json',''),
                    wild_texture=jsonData["wild_texture"],
                    tame_texture=jsonData["tame_texture"],
                    angry_texture=jsonData["angry_texture"],
                    biomes=jsonData["biomes"]
                )
                builder.write_nbt(nbt)
                nbt.write_file("1.21-registry/nbts-created/wolf_variant/"+filename.replace('.json','')+".nbt")
              
    packet = builder.build()
    print("Packet Created Successfully!")
    print(" ")
    print(packet)
    builder.save_to_file("1.21-registry/created-packets/wolf_variant-registry-packet-1.21.data")
