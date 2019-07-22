#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;
use rtfm::{app, Instant};
use stm32f1::stm32f103;

const PERIOD: u32 = 2_000_000;

#[app(device = stm32f1::stm32f103)]
const APP: () = {
	static mut PERIPHERALS: stm32f103::Peripherals = ();

	#[init]
	fn init() -> init::LateResources {
		let p = device;

		let rcc = &p.RCC;
		let gpioc = &p.GPIOC;

		rcc.apb2enr.write(|w| w.iopcen().set_bit());
		gpioc
			.crh
			.write(|w| w.mode13().bits(0b11).cnf13().bits(0b00));
		gpioc.bsrr.write(|w| w.bs13().set_bit());

		init::LateResources { PERIPHERALS: p }
	}

	#[idle(spawn = [turn_on])]
	fn idle() -> ! {
		spawn.turn_on().unwrap();
		loop {}
	}

	#[task(schedule = [turn_on], resources = [PERIPHERALS])]
	fn turn_off() {
		let now = Instant::now();

		let gpioc = &resources.PERIPHERALS.GPIOC;
		gpioc.bsrr.write(|w| w.bs13().set_bit());

		let _ = schedule.turn_on(now + PERIOD.cycles()).unwrap();
	}

	#[task(schedule = [turn_off], resources = [PERIPHERALS])]
	fn turn_on() {
		let now = Instant::now();

		let gpioc = &resources.PERIPHERALS.GPIOC;
		gpioc.brr.write(|w| w.br13().set_bit());

		let _ = schedule.turn_off(now + PERIOD.cycles()).unwrap();
	}

	extern "C" {
		fn TIM2();
	}
};
