use stm32g0xx_hal::{
    gpio::{Output, PushPull, gpiob::*},
    prelude::*,
};

use cortex_m::delay::Delay as SysDelay;

pub struct Lcd {
    rs: PB8<Output<PushPull>>,
    en: PB9<Output<PushPull>>,
    d4: PB4<Output<PushPull>>,
    d5: PB5<Output<PushPull>>,
    d6: PB6<Output<PushPull>>,
    d7: PB7<Output<PushPull>>,
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
        delay: SysDelay,
    ) -> Self {
        Self {
            rs,
            en,
            d4,
            d5,
            d6,
            d7,
            delay,
        }
    }

    pub fn init(&mut self) {
        self.delay.delay_ms(20u32);

        self.write_4bits(0x03);
        self.delay.delay_ms(5u32);
        self.write_4bits(0x03);
        self.delay.delay_ms(1u32);
        self.write_4bits(0x03);
        self.write_4bits(0x02);

        self.command(0x28);
        self.command(0x0C);
        self.command(0x01);
        self.command(0x06);
    }

    pub fn command(&mut self, cmd: u8) {
        self.rs.set_low().ok();
        self.write_4bits(cmd >> 4);
        self.write_4bits(cmd & 0x0F);
    }

    fn write_4bits(&mut self, data: u8) {
        if (data >> 0) & 0x01 != 0 {
            self.d4.set_high().ok();
        } else {
            self.d4.set_low().ok();
        }

        if (data >> 1) & 0x01 != 0 {
            self.d5.set_high().ok();
        } else {
            self.d5.set_low().ok();
        }

        if (data >> 2) & 0x01 != 0 {
            self.d6.set_high().ok();
        } else {
            self.d6.set_low().ok();
        }

        if (data >> 3) & 0x01 != 0 {
            self.d7.set_high().ok();
        } else {
            self.d7.set_low().ok();
        }

        self.pulse_enable();
    }

    fn pulse_enable(&mut self) {
        self.en.set_high().ok();
        self.delay.delay_ms(1u32);
        self.en.set_low().ok();
        self.delay.delay_ms(1u32);
    }

    fn send_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.data(byte);
        }
    }

    fn data(&mut self, data: u8) {
        self.rs.set_high().ok();
        self.write_4bits(data >> 4);
        self.write_4bits(data & 0x0F);
    }

    pub fn print(&mut self, pos_x: u8, pos_y: u8, message: &str) {
        let pos_y = if pos_y > 15 { 15 } else { pos_y };

        let cmd = match pos_x {
            0 => 0x80 + pos_y,
            1 => 0xC0 + pos_y,
            _ => return,
        };

        self.command(cmd);
        self.send_string(message);
    }

    pub fn clear(&mut self) {
        self.command(0x01);
        self.delay.delay_ms(2u32);
    }

    pub fn delay_ms(&mut self, ms: u32) {
        self.delay.delay_ms(ms);
    }
}
