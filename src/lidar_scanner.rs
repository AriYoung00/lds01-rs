use serialport::{TTYPort, SerialPort};
use std::time::Duration;
use std::io::Error;

pub struct LidarScanner {
    angular_min: f64,
    angular_max: f64,
    radial_min: f64,
    radial_max: f64,
    motor_speed: u16,

    baud_rate: u32,
    serial_port: Box<dyn SerialPort>,
    shutting_down: bool,

    range_data: [u16; 360],
    intensity_data: [u16; 360],
    raw_bytes: [u8; 2520]
}

impl LidarScanner {
    pub fn new(port: &str, baud_rate: u32) -> Self {
        let mut port = serialport::new(port, baud_rate.clone())
            .timeout(Duration::from_millis(500))
            .open()
            .expect(&*format!("Unable to open serial port {}", port));
        port.write("b".as_bytes());
        
        LidarScanner{
            angular_min: 0.0,
            angular_max: 360.0,
            radial_min: 0.0,
            radial_max: 4000.0,
            motor_speed: 0,
            baud_rate,
            serial_port: port,
            shutting_down: false,
            range_data: [0; 360],
            intensity_data: [0; 360],
            raw_bytes: [0; 2520]
        }
    }

    pub fn drop(&mut self) {
        self.serial_port.write("e".as_bytes());
    }

    pub fn poll(&mut self) -> [u16; 360] {
        let mut got_scan = false;
        let mut index = 0_usize;
        let mut motor_speed = 0_u32;
        let mut rpms = 0_u16;

        // self.range_data = [0; 360];
        // self.intensity_data = [0; 360];

        while !self.shutting_down && !got_scan {
            // self.serial_port.read_exact(&mut self.raw_bytes[..2]);
            // println!("{:x?}, {:x?}", self.raw_bytes[0], self.raw_bytes[1]);
            self.serial_port.read_exact(&mut self.raw_bytes[0..1]);
            if self.raw_bytes[0] == 0xFA {
                self.serial_port.read_exact(&mut self.raw_bytes[1..2]);
                if self.raw_bytes[1] == 0xA0 {
                    got_scan = true;
                    let res = self.serial_port.read_exact(&mut self.raw_bytes[2..]);
                    match res {
                        Ok(_) => {}
                        Err(e) => {
                            panic!(e)
                        }
                    }

                    for i in (0..self.raw_bytes.len()).step_by(42) {
                        // if self.raw_bytes[i] == 0xFA && self.raw_bytes[i+1] == (0xA0 + (i as u8) / 42) {
                            // motor_speed += ((self.raw_bytes[i + 3] as u32) << 8) + self.raw_bytes[i + 2] as u32;
                            // rpms = ((self.raw_bytes[i + 3] as u16) << 8_u8 | self.raw_bytes[i + 2] as u16) / 10;

                            for j in ((i + 4)..(i + 40)).step_by(6) {
                                index = (6 * (i / 42) + (j - 4 - i) / 6) as usize;
                                let byte0 = self.raw_bytes[j] as u16;
                                let byte1 = self.raw_bytes[j + 1] as u16;
                                let byte2 = self.raw_bytes[j + 2] as u16;
                                let byte3 = self.raw_bytes[j + 3] as u16;

                                println!("i[{}]: {}", index, self.intensity_data[index]);
                                println!("r[{}]: {}", index, self.range_data[index]);

                                self.intensity_data[index as usize] = (byte1 << 8) + byte0;
                                self.range_data[index as usize] = (byte3 << 8) + byte2;
                            }
                        // }
                    }
                }
            }
        }

        self.range_data
    }
}