use bytes::{BufMut, BytesMut};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub struct PacketBuilder{
    pub buffer: BytesMut,
    pub packet_id: i32,
}

impl PacketBuilder {
    pub fn new(packet_id:i32) -> PacketBuilder {
        let mut builder =PacketBuilder {
            buffer: BytesMut::new(),
            packet_id,
        };
        builder.write_varint(packet_id);
        builder
    }


    pub fn write_string(&mut self, value: &str) -> &mut Self {
        let value_bytes = value.as_bytes();
        let length = value.len() as i32;
        self.write_varint( length);
        self.buffer.extend_from_slice(value_bytes); 
        self
    }
    pub fn write_angle(&mut self, angle: f32) -> &mut Self {
        let encoded_angle = ((angle / 360.0) * 256.0).round() as u8;
        self.buffer.put_u8(encoded_angle);
        self
    }
    pub fn write_identifier(&mut self,identifier: String) -> &mut Self {
        let bytes = identifier.as_bytes();
        self.write_varint(bytes.len() as i32);
        self.buffer.extend_from_slice(bytes);
        self
    }
    pub fn write_short(&mut self, value: i16) -> &mut Self {
        self.buffer.put_i16(value);
        self
    }
    pub fn write_double(&mut self, value: f64) -> &mut Self {
        self.buffer.put_f64(value);
        self
    }
    pub fn write_uuid(&mut self, uuid: Uuid) -> &mut Self {
        let binding = uuid.as_u128().to_be().to_be_bytes();
        let (most_sig_bits, least_sig_bits) = binding.split_at(8);
        self.buffer.extend_from_slice(most_sig_bits);
        self.buffer.extend_from_slice(least_sig_bits);
        self
    }
    pub fn write_byte_array(&mut self, value: &[u8]) -> &mut Self {
        let length = value.len() as i32;
        self.write_varint(length); 
        self.buffer.extend_from_slice(value); 
        self
    }
    pub fn write_nbt(&mut self, nbt:crab_nbt::Nbt) -> &mut Self {
        let nbt_bytes = nbt.write_unnamed();
        self.buffer.extend_from_slice(&nbt_bytes);
        self
    }
    pub fn write_int(&mut self, mut value: i32) -> &mut Self {
        loop {
            if (value & !0x7F) == 0 {
                self.buffer.put_u8(value as u8);
                break;
            } else {
                self.buffer.put_u8((value & 0x7F | 0x80) as u8);
                value >>= 7;
            }
        }
        self
    }
    pub fn write_unsigned_short(&mut self, value: u16) -> &mut Self {
        self.buffer.put_u16(value);
        self
    }
    pub fn write_varint(&mut self, mut value: i32) -> &mut Self {
        while (value & 0xFFFFFF80u32 as i32) != 0 {
            self.buffer.put_u8((value as u8 & 0x7F) | 0x80); 
            value >>= 7;                           
        }
        self.buffer.put_u8(value as u8);                    
        self
    }
    pub fn write_long_be(&mut self, value: i64) -> &mut Self {
        self.buffer.put_i64(value.to_be()); 
        self
    }
    pub fn write_byte(&mut self, value: u8) -> &mut Self {
        self.buffer.put_u8(value);
        self
    }
    pub fn write_boolean(&mut self, value: bool) -> &mut Self {
        self.buffer.put_u8(if value { 1 } else { 0 });
        self
    }

     fn encode_varint(&self, mut value: i32) -> Vec<u8> {
        let mut encoded_bytes = Vec::new();
        loop {
            if (value & !0x7F) == 0 {
                encoded_bytes.push(value as u8);
                break;
            } else {
                encoded_bytes.push((value & 0x7F | 0x80) as u8);
                value >>= 7;
            }
        }
        encoded_bytes
    }
    pub fn build(&mut self) -> BytesMut {
        let mut packet = BytesMut::new();
        
        // Copiamos el buffer actual (que ya contiene el packet_id y los datos)
        let packet_data = self.buffer.split();  // Split para extraer los datos actuales sin modificar self.buffer
    
        // Calculamos la longitud total (packet_id + datos del paquete)
        let total_length = packet_data.len() as i32;
        
        // Escribimos la longitud total del paquete
        packet.put_slice(&self.encode_varint(total_length));
        
        // Luego añadimos el contenido del paquete (packet_id y datos)
        packet.extend_from_slice(&packet_data);
        packet
    }
    

    pub async fn send(&mut self, socket: &mut tokio::net::TcpStream) -> Result<(), String> {
        let packet = self.build();
        socket.write_all(&packet).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}