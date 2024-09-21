import os
import json
from nbt.nbt import *
from packetbuilder import PacketBuilder

def create_damege_type(name, message_id, scaling, exhaustion):
    biome = NBTFile()
    biome.name = name
    
    biome.tags.append(TAG_String(name="message_id", value=message_id))
    biome.tags.append(TAG_String(name="scaling", value=scaling))
    biome.tags.append(TAG_Float(name="exhaustion", value=exhaustion))

    
    return biome

if __name__ == "__main__":
    packet_id = 7 
    builder = PacketBuilder(packet_id)
    builder.write_identifier("minecraft:damage_type")
    i = 0
    biome_folder = '1.21-registry/extracted-from-jar/damage_type'
    
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
                biome = create_damege_type(
                    name=filename.replace('.json',''),
                    message_id=biome_data["message_id"],
                    scaling=biome_data["scaling"],
                    exhaustion=biome_data["exhaustion"]
                )
                builder.write_nbt(biome)
                biome.write_file("1.21-registry/nbts-created/damage_type/"+filename.replace('.json','')+".nbt")
              
    packet = builder.build()
    print("Packet Created Successfully!")
    print(" ")
    print(packet)
    builder.save_to_file("1.21-registry/created-packets/damage_type-registry-packet-1.21.data")
