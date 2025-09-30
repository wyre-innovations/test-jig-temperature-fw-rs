#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod app;
mod lcd;

use cortex_m_rt::entry;
use panic_halt as _;
use stm32g0xx_hal::{
    analog::adc::{AdcExt, OversamplingRatio, Precision, SampleTime},
    gpio::*,
    prelude::*,
    rcc::{Config, PLLSrc, PllConfig},
    stm32,
};

use cortex_m::delay::Delay as SysDelay;

use app::App;
use lcd::Lcd;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");

    let mut rcc = dp.RCC.constrain();

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);

    let config = Config::pll()
        .pll_cfg(PllConfig {
            mux: PLLSrc::HSI,
            m: 1,
            n: 8,
            r: 2,
            q: None,
            p: Some(2),
        })
        .ahb_psc(stm32g0xx_hal::rcc::Prescaler::NotDivided)
        .apb_psc(stm32g0xx_hal::rcc::Prescaler::NotDivided);

    let mut rcc = rcc.freeze(config);

    let lcd_backlight = gpioc.pc13.into_push_pull_output();

    let lcd_rs = gpiob.pb8.into_push_pull_output();
    let lcd_en = gpiob.pb9.into_push_pull_output();
    let lcd_d4 = gpiob.pb4.into_push_pull_output();
    let lcd_d5 = gpiob.pb5.into_push_pull_output();
    let lcd_d6 = gpiob.pb6.into_push_pull_output();
    let lcd_d7 = gpiob.pb7.into_push_pull_output();

    let adc_pins = (
        gpioa.pa0.into_analog(),
        gpioa.pa1.into_analog(),
        gpioa.pa2.into_analog(),
        gpioa.pa3.into_analog(),
        gpioa.pa4.into_analog(),
        gpioa.pa5.into_analog(),
    );

    let mut adc = dp.ADC.constrain(&mut rcc);
    adc.set_sample_time(SampleTime::T_160);
    adc.set_precision(Precision::B_12);
    adc.set_oversampling_ratio(OversamplingRatio::X_256);
    adc.set_oversampling_shift(8);
    adc.oversampling_enable(true);

    let core_hz: u32 = rcc.clocks.core_clk.raw();
    let delay = SysDelay::new(cp.SYST, core_hz);

    let lcd = Lcd::new(
        lcd_rs,
        lcd_en,
        lcd_d4,
        lcd_d5,
        lcd_d6,
        lcd_d7,
        lcd_backlight,
        delay,
    );

    let mut app = App::new(adc, adc_pins, lcd);

    app.setup();
    app.run()
}
