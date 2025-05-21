pub struct FifoData {
    pub gyro_data: GyroData,
}

pub struct GyroData {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl FifoData {
    pub fn from_buffer(buffer: [u8; 6]) -> Self {
        FifoData {
            gyro_data: GyroData {
                x: u16::from_be_bytes([buffer[0], buffer[1]]),
                y: u16::from_be_bytes([buffer[2], buffer[3]]),
                z: u16::from_be_bytes([buffer[4], buffer[5]]),
            },
        }
    }
}
