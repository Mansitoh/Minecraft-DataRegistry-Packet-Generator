import os
import json
from nbt.nbt import *
from packetbuilder import PacketBuilder

def create_chat_type(name, chat_translation_key,chat_parameters,narration_translation_key,narration_parameters):
    nbt = NBTFile()
    nbt.name = name
    
    chatCompound = TAG_Compound(name="chat")
    chatCompound.tags.append(TAG_String(name="translate", value=chat_translation_key))
    chatCompound.tags.append(TAG_List(name="parameters", type=TAG_String))
    for parameter in chat_parameters:
        chatCompound["parameters"].tags.append(TAG_String(value=parameter))
    nbt.tags.append(chatCompound)

    narrationCompound = TAG_Compound(name="narration")
    narrationCompound.tags.append(TAG_String(name="translate", value=narration_translation_key))
    narrationCompound.tags.append(TAG_List(name="parameters", type=TAG_String))
    for parameter in narration_parameters:
        narrationCompound["parameters"].tags.append(TAG_String(value=parameter))
    nbt.tags.append(narrationCompound)


    return nbt

if __name__ == "__main__":
    packet_id = 7 
    builder = PacketBuilder(packet_id)
    builder.write_identifier("minecraft:chat_type")
    i = 0
    biome_folder = '1.21-registry/extracted-from-jar/chat_type'
    
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
                nbt = create_chat_type(
                    name=filename.replace('.json',''),
                    chat_translation_key=jsonData["chat"]["translation_key"],
                    chat_parameters=jsonData["chat"]["parameters"],
                    narration_translation_key=jsonData["narration"]["translation_key"],
                    narration_parameters=jsonData["narration"]["parameters"]
                )
                builder.write_nbt(nbt)
                nbt.write_file("1.21-registry/nbts-created/chat_type/"+filename.replace('.json','')+".nbt")
              
    packet = builder.build()
    print("Packet Created Successfully!")
    print(" ")
    print(packet)
    builder.save_to_file("1.21-registry/created-packets/chat_type-registry-packet-1.21.data")
