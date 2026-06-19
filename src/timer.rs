// use atsamd_hal::{clock::GenericClockController, fugit::ExtU32, pac::{Pm, Tc3}, timer::TimerCounter, timer_traits::InterruptDrivenTimer};
// use portable_atomic::{AtomicU32, Ordering};
// use crate::pac::interrupt;

// static COUNT: AtomicU32 = AtomicU32::new(0);

// pub fn set_up(clocks: &mut GenericClockController, tc3: Tc3, pm: &mut Pm) {
//     let glck0 = &clocks.gclk0();

//     let mut tc3 = TimerCounter::tc3_(&clocks.tcc2_tc3(glck0).unwrap(), tc3, pm);
//     tc3.start(1u32.millis());
//     tc3.enable_interrupt();
// }

// #[interrupt]
// fn TC3() {
//     COUNT.fetch_add(1, Ordering::Relaxed);
//     unsafe { Tc3::steal() }.count16().intflag().write(|w| w.ovf().set_bit());
// }

defmt::timestamp!("{=u64}.{=u64:03}s", embassy_time::Instant::now().as_millis() / 1000, embassy_time::Instant::now().as_millis() % 1000);