import os
import json
from nbt.nbt import *
from packetbuilder import PacketBuilder

def create_trim_material(name, asset_name, ingredient, item_model_index, description_color,description_translate):
    biome = NBTFile()
    biome.name = name
    
    biome.tags.append(TAG_String(name="asset_name", value=asset_name))
    biome.tags.append(TAG_String(name="ingredient", value=ingredient))
    biome.tags.append(TAG_Float(name="item_model_index", value=item_model_index))
    #description compound
    description = TAG_Compound(name="description")
    description.tags.append(TAG_String(name="color", value=description_color))
    description.tags.append(TAG_String(name="translate", value=description_translate))
    biome.tags.append(description)



    return biome

if __name__ == "__main__":
    packet_id = 7 
    builder = PacketBuilder(packet_id)
    builder.write_identifier("minecraft:trim_material")
    i = 0
    biome_folder = '1.21-registry/extracted-from-jar/trim_material'
    
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
                biome = create_trim_material(
                    name=filename.replace('.json',''),
                    asset_name=biome_data["asset_name"],
                    ingredient=biome_data["ingredient"],
                    item_model_index=biome_data["item_model_index"],
                    description_color=biome_data["description"]["color"],
                    description_translate=biome_data["description"]["translate"]
                )
                builder.write_nbt(biome)
                biome.write_file("1.21-registry/nbts-created/trim_material/"+filename.replace('.json','')+".nbt")
              
    packet = builder.build()
    print("Packet Created Successfully!")
    print(" ")
    print(packet)
    builder.save_to_file("1.21-registry/created-packets/trim_material-registry-packet-1.21.data")
