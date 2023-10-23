#![no_std]
#![no_main]

// use dht_sensor::*;
use esp32_hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc, IO,
};
use esp_backtrace as _;
use esp_println::println;
use xtensa_lx_rt::entry;
use dht_sensor::dht22;
use std::thread;
use std::time::Duration;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable watchdog timer
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    //let mut delay = Delay::new(&clocks);
    //let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    //let mut led = io.pins.gpio4.into_push_pull_output();
    //led.set_high().unwrap();

     let mut dht_pin = peripherals.pins.gpio19;
     dht_pin.set_high().ok();
     thread::sleep(Duration::from_millis(1000));

    loop {
        println!("Hello World!");
        // led.toggle().unwrap();

         match dht22::Reading::read(&mut dht_pin) {
             Err(e) => println!("Error {:?}", e),
             Ok(dht22::Reading {
                 temperature,
                 relative_humidity,
             }) => println!("{}°, {}% RH", temperature, relative_humidity),
         }

        thread::sleep(Duration::from_millis(1000));
    }
}
