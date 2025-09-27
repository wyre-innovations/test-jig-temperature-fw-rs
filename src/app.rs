use crate::lcd::Lcd;
use micromath::F32Ext;
use stm32g0xx_hal::{
    analog::adc::Adc,
    gpio::{Analog, gpioa::*},
    prelude::*,
};

const STEINHART_A: f32 = 0.001129148;
const STEINHART_B: f32 = 0.000234125;
const STEINHART_C: f32 = 8.76741e-8;
const R_FIXED: f32 = 10000.0;

pub struct App {
    adc: Adc,
    adc_pins: (
        PA0<Analog>,
        PA1<Analog>,
        PA2<Analog>,
        PA3<Analog>,
        PA4<Analog>,
        PA5<Analog>,
    ),
    lcd: Lcd,
}

impl App {
    pub fn new(
        adc: Adc,
        adc_pins: (
            PA0<Analog>,
            PA1<Analog>,
            PA2<Analog>,
            PA3<Analog>,
            PA4<Analog>,
            PA5<Analog>,
        ),
        lcd: Lcd,
    ) -> Self {
        Self { adc, adc_pins, lcd }
    }

    #[inline]
    fn calculate_temperature(&self, adc_value: u16) -> f32 {
        const ADC_MAX: f32 = 4095.0;

        // Open-circuit guard: avoids div-by-zero when adc_value == 4095
        if adc_value == 4095 {
            return f32::INFINITY;
        }

        let adc = adc_value as f32;
        let r_therm = R_FIXED * adc / (ADC_MAX - adc);
        let ln_r = r_therm.ln();
        let inv_t = STEINHART_A + STEINHART_B * ln_r + STEINHART_C * ln_r * ln_r * ln_r;
        (1.0 / inv_t) - 273.15
    }

    pub fn setup(&mut self) {
        self.lcd.init();
        self.lcd.print(0, 0, "Wyre Innovations");
        self.lcd.delay_ms(2000u32);
        self.lcd.clear();
    }

    pub fn run(&mut self) -> ! {
        loop {
            let adc_values = [
                self.adc.read(&mut self.adc_pins.0).unwrap(),
                self.adc.read(&mut self.adc_pins.1).unwrap(),
                self.adc.read(&mut self.adc_pins.2).unwrap(),
                self.adc.read(&mut self.adc_pins.3).unwrap(),
                self.adc.read(&mut self.adc_pins.4).unwrap(),
                self.adc.read(&mut self.adc_pins.5).unwrap(),
            ];

            let temperatures: [f32; 6] = [
                self.calculate_temperature(adc_values[0]),
                self.calculate_temperature(adc_values[1]),
                self.calculate_temperature(adc_values[2]),
                self.calculate_temperature(adc_values[3]),
                self.calculate_temperature(adc_values[4]),
                self.calculate_temperature(adc_values[5]),
            ];

            let mut line1 = heapless::String::<17>::new();
            let mut line2 = heapless::String::<17>::new();

            use core::fmt::Write;
            write!(
                line1,
                "T1:{:.1} T2:{:.1} T3:{:.1}",
                temperatures[0], temperatures[1], temperatures[2]
            )
            .ok();
            write!(
                line2,
                "T4:{:.1} T5:{:.1} T6:{:.1}",
                temperatures[3], temperatures[4], temperatures[5]
            )
            .ok();

            self.lcd.clear();
            self.lcd.print(0, 0, line1.as_str());
            self.lcd.print(1, 0, line2.as_str());

            self.lcd.delay_ms(1000u32);
        }
    }
}
