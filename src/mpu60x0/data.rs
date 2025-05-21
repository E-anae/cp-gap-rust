pub struct FifoData {
    pub gyro_data: GyroData,
}

pub struct GyroData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl FifoData {
    pub fn from_buffer(buffer: [u8; 6]) -> Self {
        FifoData {
            gyro_data: GyroData {
                x: i16::from_be_bytes([buffer[0], buffer[1]]),
                y: i16::from_be_bytes([buffer[2], buffer[3]]),
                z: i16::from_be_bytes([buffer[4], buffer[5]]),
            },
        }
    }
}
