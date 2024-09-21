import os
import json
from nbt.nbt import *
from packetbuilder import PacketBuilder

def create_data(name, has_skylight,has_ceiling,ultrawarm,natural,coordinate_scale,bed_works,respawn_anchor_works,min_y,height,logical_height,infiniburn,effects,ambient_light,piglin_safe,has_raids,monster_spawn_light_level_type,monster_spawn_light_level_max_inclusive,monster_spawn_light_level_min_inclusive,monster_spawn_block_light_limit):
    nbt = NBTFile()
    nbt.name = name
    
    nbt.tags.append(TAG_Byte(name="has_skylight", value=has_skylight))
    nbt.tags.append(TAG_Byte(name="has_ceiling", value=has_ceiling))
    nbt.tags.append(TAG_Byte(name="ultrawarm", value=ultrawarm))
    nbt.tags.append(TAG_Byte(name="natural", value=natural))
    nbt.tags.append(TAG_Double(name="coordinate_scale", value=coordinate_scale))
    nbt.tags.append(TAG_Byte(name="bed_works", value=bed_works))
    nbt.tags.append(TAG_Byte(name="respawn_anchor_works", value=respawn_anchor_works))
    nbt.tags.append(TAG_Int(name="min_y", value=min_y))
    nbt.tags.append(TAG_Int(name="height", value=height))
    nbt.tags.append(TAG_Int(name="logical_height", value=logical_height))
    nbt.tags.append(TAG_String(name="infiniburn", value=infiniburn))
    nbt.tags.append(TAG_String(name="effects",value=effects))
    nbt.tags.append(TAG_Float(name="ambient_light", value=ambient_light))
    nbt.tags.append(TAG_Byte(name="piglin_safe", value=piglin_safe))
    nbt.tags.append(TAG_Byte(name="has_raids", value=has_raids))
    monster_spawn_light_levelCompbound = TAG_Compound(name="monster_spawn_light_level")
    monster_spawn_light_levelCompbound.tags.append(TAG_String(name="type", value=monster_spawn_light_level_type))
    monster_spawn_light_levelCompbound.tags.append(TAG_Int(name="max_inclusive", value=monster_spawn_light_level_max_inclusive))
    monster_spawn_light_levelCompbound.tags.append(TAG_Int(name="min_inclusive", value=monster_spawn_light_level_min_inclusive))
    nbt.tags.append(monster_spawn_light_levelCompbound)
    nbt.tags.append(TAG_Int(name="monster_spawn_block_light_limit", value=monster_spawn_block_light_limit))
    return nbt

if __name__ == "__main__":
    packet_id = 7 
    builder = PacketBuilder(packet_id)
    builder.write_identifier("minecraft:dimension_type")
    i = 0
    biome_folder = '1.21-registry/extracted-from-jar/dimension_type'
    
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
                    has_skylight=jsonData["has_skylight"],
                    has_ceiling=jsonData["has_ceiling"],
                    ultrawarm=jsonData["ultrawarm"],
                    natural=jsonData["natural"],
                    coordinate_scale=jsonData["coordinate_scale"],
                    bed_works=jsonData["bed_works"],
                    respawn_anchor_works=jsonData["respawn_anchor_works"],
                    min_y=jsonData["min_y"],
                    height=jsonData["height"],
                    logical_height=jsonData["logical_height"],
                    infiniburn=jsonData["infiniburn"],
                    effects=jsonData["effects"],
                    ambient_light=jsonData["ambient_light"],
                    piglin_safe=jsonData["piglin_safe"],
                    has_raids=jsonData["has_raids"],
                    monster_spawn_light_level_type=jsonData["monster_spawn_light_level"]["type"],
                    monster_spawn_light_level_max_inclusive=jsonData["monster_spawn_light_level"]["max_inclusive"],
                    monster_spawn_light_level_min_inclusive=jsonData["monster_spawn_light_level"]["min_inclusive"],
                    monster_spawn_block_light_limit=jsonData["monster_spawn_block_light_limit"]
                )
                builder.write_nbt(nbt)
                nbt.write_file("1.21-registry/nbts-created/dimension_type/"+filename.replace('.json','')+".nbt")
              
    packet = builder.build()
    print("Packet Created Successfully!")
    print(" ")
    print(packet)
    builder.save_to_file("1.21-registry/created-packets/dimension_type-registry-packet-1.21.data")
