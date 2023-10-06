#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{init_node, DeviceIdentifier, LifeTimeType, NodeConfig};
use panic_halt as _;

mod mesh_lib;

use mesh_lib::millis::{millis, ms};

use mesh_lib::NodeString;
use ufmt::uwrite;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut mesh_node = init_node(NodeConfig {
        device_identifier: DeviceIdentifier(2),
        listen_period: 500 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_send_time: ms = millis();
    let mut now_time: ms;
    let mut packet_counter: u32 = 0;
    let mut led_pin = pins.d13.into_output();

    loop {
        let _ = mesh_node.update();

        if let Some(message) = mesh_node.receive() {
            led_pin.toggle();
            serial_println!("\nMsg");
            for byte in message.data {
                serial_write_byte!(byte).unwrap_or({});
            }
        }

        now_time = millis();
        if now_time > (last_send_time + 1000 as ms) {
            last_send_time = now_time;

            let mut message = NodeString::new();
            uwrite!(&mut message, "Packet #: {}", packet_counter).unwrap();

            if let Ok(_) = mesh_node.send_ping_pong(
                message.into_bytes(),
                DeviceIdentifier(1),
                10 as LifeTimeType,
                true,
                5000 as ms,
            ) {
                led_pin.toggle();
                serial_println!("Transaction done!");
            } else {
                serial_println!("Transaction failed!");
            }
            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
