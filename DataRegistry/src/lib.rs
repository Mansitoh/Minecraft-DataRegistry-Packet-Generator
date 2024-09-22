pub mod packet_builder;
pub mod registry_1_21{
    pub mod data_registry_1_21;
    pub mod trim_material;
    pub mod trim_pattern;
    pub mod banner_pattern;
    pub mod worldgen_biome;
    pub mod chat_type;
    pub mod damage_type;
    pub mod dimension_type;
    pub mod wolf_variant;
    pub mod painting_variant;
}
pub mod utils;
pub use packet_builder::PacketBuilder;