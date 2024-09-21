import os
import json
from nbt.nbt import *
from packetbuilder import PacketBuilder

def create_trim_pattern(name, asset_id, template_item,description_translate,decal):
    nbt = NBTFile()
    nbt.name = name
    
    nbt.tags.append(TAG_String(name="asset_id", value=asset_id))
    nbt.tags.append(TAG_String(name="template_item", value=template_item))
    
    descriptionCompound = TAG_Compound(name="description")
    descriptionCompound.tags.append(TAG_String(name="translate", value=description_translate))
    nbt.tags.append(descriptionCompound)

    nbt.tags.append(TAG_Byte(name="decal", value=decal))


    return nbt

if __name__ == "__main__":
    packet_id = 7 
    builder = PacketBuilder(packet_id)
    builder.write_identifier("minecraft:trim_pattern")
    i = 0
    biome_folder = '1.21-registry/extracted-from-jar/trim_pattern'
    
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
                nbt = create_trim_pattern(
                    name=filename.replace('.json',''),
                    asset_id=jsonData["asset_id"],
                    template_item=jsonData["template_item"],
                    description_translate=jsonData["description"]["translate"],
                    decal=jsonData["decal"]
                )
                builder.write_nbt(nbt)
                nbt.write_file("1.21-registry/nbts-created/trim_pattern/"+filename.replace('.json','')+".nbt")
              
    packet = builder.build()
    print("Packet Created Successfully!")
    print(" ")
    print(packet)
    builder.save_to_file("1.21-registry/created-packets/trim_pattern-registry-packet-1.21.data")
