//! This example shows how to use UART (Universal asynchronous receiver-transmitter) in the RP2040 chip.
//!
//! Test TX-only and RX-only on two different UARTs. You need to connect GPIO0 to GPIO5 for
//! this to work
//! The Raspberry Pi Debug Probe (https://www.raspberrypi.com/products/debug-probe/) could be used
//! with its UART port.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::UART1;
use embassy_rp::uart::{Async, Config, InterruptHandler, UartRx, UartTx};
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pin};
use embassy_time::{Timer, Delay};
use {defmt_rtt as _, panic_probe as _};

use dht_sensor::{dht11};

bind_interrupts!(struct Irqs {
    UART1_IRQ => InterruptHandler<UART1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut uart_tx = UartTx::new(p.UART0, p.PIN_0, p.DMA_CH0, Config::default());
    let uart_rx = UartRx::new(p.UART1, p.PIN_5, Irqs, p.DMA_CH1, Config::default());

    unwrap!(spawner.spawn(reader(uart_rx)));

    unwrap!(spawner.spawn(read_sensor(p.PIN_27.degrade())));

    info!("Writing...");
    loop {
        // let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
        // info!("TX {:?}", data);
        // uart_tx.write(&data).await.unwrap();
        // Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn reader(mut rx: UartRx<'static, UART1, Async>) {
    info!("Reading...");
    loop {
        // read a total of 4 transmissions (32 / 8) and then print the result
        let mut buf = [0; 32];
        let res = rx.read(&mut buf).await;
        info!("Read {:?}", res);
        if let Ok(()) = res {
            info!("RX {:?}", &buf[..]);
        }
    }
}


#[embassy_executor::task]
async fn read_sensor(mut dht_pin: AnyPin) {
    // let mut pin = gpio::InOutPin::new(pins.gpio27);

    let mut output_pin = Output::new(dht_pin, Level::Low);

    let _ = output_pin.set_high();

    // let mut dht_pin = dht_pin.degrade();

    // let mut input_pin = Input::new(dht_pin, Pull::Up);
    // Perform a sensor reading
    let measurement = dht11::read(&mut Delay, &mut output_pin);
}