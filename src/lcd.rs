use cortex_m::delay::Delay as SysDelay;
use stm32g0xx_hal::{
    gpio::{Output, PushPull, gpiob::*, gpioc::*},
    prelude::*,
};

pub struct Lcd {
    rs: PB8<Output<PushPull>>,
    en: PB9<Output<PushPull>>,
    d4: PB4<Output<PushPull>>,
    d5: PB5<Output<PushPull>>,
    d6: PB6<Output<PushPull>>,
    d7: PB7<Output<PushPull>>,
    backlight: PC13<Output<PushPull>>,
    delay: SysDelay,
}

impl Lcd {
    pub fn new(
        rs: PB8<Output<PushPull>>,
        en: PB9<Output<PushPull>>,
        d4: PB4<Output<PushPull>>,
        d5: PB5<Output<PushPull>>,
        d6: PB6<Output<PushPull>>,
        d7: PB7<Output<PushPull>>,
        backlight: PC13<Output<PushPull>>,
        delay: SysDelay,
    ) -> Self {
        let mut lcd = Self {
            rs,
            en,
            d4,
            d5,
            d6,
            d7,
            backlight,
            delay,
        };
        // Known idle levels
        let _ = lcd.rs.set_low();
        let _ = lcd.en.set_low();
        let _ = lcd.d4.set_low();
        let _ = lcd.d5.set_low();
        let _ = lcd.d6.set_low();
        let _ = lcd.d7.set_low();
        // Backlight off initially (we'll turn it on after init)
        let _ = lcd.backlight.set_low();
        lcd
    }

    /// Mirror the C init exactly:
    ///  - 20ms power-up wait
    ///  - 0x03, 5ms, 0x03, 1ms, 0x03, then 0x02 (4-bit)
    ///  - 0x28, 0x0C, 0x01, 0x06
    pub fn init(&mut self) {
        self.delay.delay_ms(20u32);

        self.write_4bits(0x03);
        self.delay.delay_ms(5u32);
        self.write_4bits(0x03);
        self.delay.delay_ms(1u32);
        self.write_4bits(0x03);
        self.write_4bits(0x02);

        self.command(0x28); // Function set: 4-bit, 2 line, 5x8
        self.command(0x0C); // Display ON, cursor OFF, blink OFF
        self.command(0x01); // Clear display
        self.command(0x06); // Entry mode: increment, no shift

        // Backlight on (active-high like your C code)
        let _ = self.backlight.set_high();
    }

    #[inline]
    pub fn command(&mut self, cmd: u8) {
        let _ = self.rs.set_low();
        self.write_4bits(cmd >> 4);
        self.write_4bits(cmd & 0x0F);

        // Your C code only waits explicitly after clear; we’ll keep that behavior.
        if cmd == 0x01 {
            self.delay.delay_ms(2u32); // >=1.52ms for clear
        }
    }

    #[inline]
    pub fn data(&mut self, data: u8) {
        let _ = self.rs.set_high();
        self.write_4bits(data >> 4);
        self.write_4bits(data & 0x0F);
    }

    /// Match C: set D4..D7 from nibble, then pulse E with 1ms high, 1ms low
    #[inline]
    fn write_4bits(&mut self, nibble: u8) {
        if (nibble & 0x01) != 0 {
            let _ = self.d4.set_high();
        } else {
            let _ = self.d4.set_low();
        }
        if (nibble & 0x02) != 0 {
            let _ = self.d5.set_high();
        } else {
            let _ = self.d5.set_low();
        }
        if (nibble & 0x04) != 0 {
            let _ = self.d6.set_high();
        } else {
            let _ = self.d6.set_low();
        }
        if (nibble & 0x08) != 0 {
            let _ = self.d7.set_high();
        } else {
            let _ = self.d7.set_low();
        }
        self.pulse_enable();
    }

    /// Exact pulse as C: E=1; delay 1ms; E=0; delay 1ms
    #[inline]
    fn pulse_enable(&mut self) {
        let _ = self.en.set_high();
        self.delay.delay_ms(1u32);
        let _ = self.en.set_low();
        self.delay.delay_ms(1u32);
    }

    pub fn send_string(&mut self, s: &str) {
        // Keep simple like C; assume ASCII
        for b in s.bytes() {
            self.data(b);
        }
    }

    pub fn print(&mut self, pos_x: u8, pos_y: u8, message: &str) {
        let pos_y = pos_y.min(15);
        let cmd = if pos_x == 0 {
            0x80 + pos_y
        } else if pos_x == 1 {
            0xC0 + pos_y
        } else {
            return;
        };
        self.command(cmd);
        self.send_string(message);
    }

    pub fn clear(&mut self) {
        self.command(0x01);
        // 2ms handled in command()
    }

    pub fn delay_ms(&mut self, delay: u32) {
        self.delay.delay_ms(delay * 16);
    }
}
