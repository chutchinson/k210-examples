#![no_std]
#![no_main]

use k210_hal::{prelude::*, fpioa, pac};
use k210_hal::gpio::{Gpio};
use panic_halt as _;

fn sleep(duration: u64) {
    use riscv::register::mcycle;
    let clocks = k210_hal::clock::Clocks::new();
    let freq = clocks.cpu().0 as u64;
    let count = freq * duration / 1000;
    let count = count as usize;
    let cycle = mcycle::read();
    while mcycle::read().wrapping_sub(cycle) < count {
        // spin
    }
}

#[cfg(feature="maixduino")]
#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut sysctl = p.SYSCTL.constrain();
    let fpioa = p.FPIOA.split(&mut sysctl.apb0);
    let gpios = p.GPIO.split(&mut sysctl.apb0);
    
    let led = fpioa.io5.into_function(fpioa::GPIO0);
    let mut led = Gpio::new(gpios.gpio0, led).into_push_pull_output();

    loop {
        led.try_set_low().unwrap();
        sleep(500);
        led.try_set_high().unwrap();
        sleep(500);
    }
}


#[cfg(feature="maix")]
#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut sysctl = p.SYSCTL.constrain();
    let fpioa = p.FPIOA.split(&mut sysctl.apb0);
    let gpios = p.GPIO.split(&mut sysctl.apb0);

    let led = fpioa.io14.into_function(fpioa::GPIO0);
    let mut led = Gpio::new(gpios.gpio0, led).into_push_pull_output();

    loop {
        led.try_set_low().unwrap();
        sleep(500);
        led.try_set_high().unwrap();
        sleep(500);
    }
}
