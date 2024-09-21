import os
import json
from nbt.nbt import *
from packetbuilder import PacketBuilder

def create_data(name,asset_id,height,width):
    nbt = NBTFile()
    nbt.name = name
    
    nbt.tags.append(TAG_String(name="asset_id", value=asset_id))
    nbt.tags.append(TAG_Int(name="height", value=height))
    nbt.tags.append(TAG_Int(name="width", value=width))
    return nbt

if __name__ == "__main__":
    packet_id = 7 
    builder = PacketBuilder(packet_id)
    builder.write_identifier("minecraft:painting_variant")
    i = 0
    biome_folder = '1.21-registry/extracted-from-jar/painting_variant'
    
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
                    asset_id=jsonData["asset_id"],
                    height=jsonData["height"],
                    width=jsonData["width"]
                )
                builder.write_nbt(nbt)
                nbt.write_file("1.21-registry/nbts-created/painting_variant/"+filename.replace('.json','')+".nbt")
              
    packet = builder.build()
    print("Packet Created Successfully!")
    print(" ")
    print(packet)
    builder.save_to_file("1.21-registry/created-packets/painting_variant-registry-packet-1.21.data")
