use mcp3208::{Channel, Mcp3208};
use std::io::{stdout, Write};

fn main() {
    if let Ok(mut mcp3208) = Mcp3208::new("/dev/spidev0.0") {
        loop {
            let x = mcp3208.read_adc_single(Channel::Ch0).unwrap();
            let y = mcp3208.read_adc_single(Channel::Ch1).unwrap();
            let z = mcp3208.read_adc_single(Channel::Ch2).unwrap();

            print!("\rx: {x}({x:012b}), y: {y}({y:012b}), z: {z}({z:012b})",);
            stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
