import struct
from uuid import UUID
import io
from nbt import nbt

class PacketBuilder:
    def __init__(self, packet_id: int):
        self.buffer = bytearray()
        self.write_varint(packet_id)

    def write_string(self, value: str):
        value_bytes = value.encode('utf-8')
        self.write_varint(len(value_bytes))
        self.buffer.extend(value_bytes)
        return self

    def write_angle(self, angle: float):
        encoded_angle = round((angle / 360.0) * 256.0) & 0xFF
        self.buffer.append(encoded_angle)
        return self

    def write_identifier(self, identifier: str):
        self.write_string(identifier)
        return self

    def write_short(self, value: int):
        self.buffer.extend(struct.pack('>h', value))
        return self

    def write_double(self, value: float):
        self.buffer.extend(struct.pack('>d', value))
        return self

    def write_uuid(self, uuid: UUID):
        most_sig_bits = uuid.int >> 64
        least_sig_bits = uuid.int & ((1 << 64) - 1)
        self.buffer.extend(struct.pack('>Q', most_sig_bits))
        self.buffer.extend(struct.pack('>Q', least_sig_bits))
        return self

    def write_byte_array(self, value: bytes):
        self.write_varint(len(value))
        self.buffer.extend(value)
        return self

    def write_int(self, value: int):
        self.write_varint(value)
        return self

    def write_unsigned_short(self, value: int):
        self.buffer.extend(struct.pack('>H', value))
        return self
    
    def write_nbt(self, nbt_data: nbt.NBTFile):
        """Escribe un objeto NBT en el buffer utilizando la estructura NBT sin comprimir."""
        temp_buffer = io.BytesIO()
        nbt_data.write_file(buffer=temp_buffer)
        self.buffer.extend(temp_buffer.getvalue())
        
        return self

    def write_varint(self, value: int):
        while True:
            if (value & ~0x7F) == 0:
                self.buffer.append(value)
                break
            else:
                self.buffer.append((value & 0x7F) | 0x80)
                value >>= 7
        return self

    def write_long_be(self, value: int):
        self.buffer.extend(struct.pack('>q', value))
        return self

    def write_byte(self, value: int):
        self.buffer.append(value & 0xFF)
        return self

    def write_boolean(self, value: bool):
        self.buffer.append(1 if value else 0)
        return self

    def build(self):
        packet_data = bytes(self.buffer)
        total_length = len(packet_data)
        length_bytes = self.encode_varint(total_length)
        return length_bytes + packet_data

    def encode_varint(self, value: int):
        encoded_bytes = bytearray()
        while True:
            if (value & ~0x7F) == 0:
                encoded_bytes.append(value)
                break
            else:
                encoded_bytes.append((value & 0x7F) | 0x80)
                value >>= 7
        return encoded_bytes

    async def send(self, writer):
        packet = self.build()
        writer.write(packet)
        await writer.drain()

    def save_to_file(self, filename: str):
        """Guarda el contenido del buffer en un archivo binario."""
        with open(filename, 'wb') as file:
            file.write(self.build())