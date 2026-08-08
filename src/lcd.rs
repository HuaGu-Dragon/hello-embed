use embassy_time::{Duration, Timer};
use esp_hal::{
    Blocking,
    gpio::{Level, Output},
    peripherals::{GPIO11, GPIO12, GPIO13, GPIO21, GPIO40, GPIO47, GPIO48, SPI2},
    spi::master::{Config, Spi},
};

extern crate alloc;

pub struct Lcd<'a> {
    spi: Spi<'a, Blocking>,
    rst: Output<'static>,
    blk: Output<'static>,
    dc: Output<'static>,
    // cs: Output<'static>,
    buf: alloc::vec::Vec<u8>,
}

pub struct Rect {
    pub start_x: u16,
    pub end_x: u16,
    pub start_y: u16,
    pub end_y: u16,
}

impl<'a> Lcd<'a> {
    pub async fn init_spi2(
        spi2: SPI2<'static>,
        mosi: GPIO11<'static>,
        sck: GPIO12<'static>,
        miso: GPIO13<'static>,
        gpio21: GPIO21<'static>,
        gpio40: GPIO40<'static>,
        gpio47: GPIO47<'static>,
        cs: GPIO48<'static>,
    ) -> Self {
        let spi = Spi::new(spi2, Config::default())
            .unwrap()
            .with_cs(cs)
            .with_miso(miso)
            .with_mosi(mosi)
            .with_sck(sck);

        let rst = Output::new(gpio21, Level::High, Default::default());
        let blk = Output::new(gpio40, Level::High, Default::default());
        let dc = Output::new(gpio47, Level::Low, Default::default());
        // let mut gpio48 = Output::new(gpio48, Level::High, Default::default());

        let mut lcd = Self {
            spi,
            rst,
            blk,
            dc,
            // cs: gpio48,
            buf: alloc::vec::Vec::with_capacity(240 * 240 * 2),
        };

        lcd.rst(Level::Low);
        Timer::after(Duration::from_millis(100)).await;
        lcd.rst(Level::High);
        Timer::after(Duration::from_millis(100)).await;

        lcd.blk(Level::High);
        Timer::after(Duration::from_millis(100)).await;

        lcd.write_cmd(&[0x11]);
        Timer::after(Duration::from_millis(120)).await;

        lcd.write_cmd(&[0xB2]);
        lcd.write_data(&[0x0C, 0x0C, 0x00, 0x33, 0x33]);

        lcd.write_cmd(&[0x35]);
        lcd.write_data(&[0x00]);

        lcd.write_cmd(&[0x36]);
        lcd.write_data(&[0x70, 0xA0]);

        lcd.write_cmd(&[0x3A]);
        lcd.write_data(&[0x05]);

        lcd.write_cmd(&[0xB7]);
        lcd.write_data(&[0x35]);

        lcd.write_cmd(&[0xBB]);
        lcd.write_data(&[0x2D]);

        lcd.write_cmd(&[0xC0]);
        lcd.write_data(&[0x2C]);

        lcd.write_cmd(&[0xC2]);
        lcd.write_data(&[0x01]);

        lcd.write_cmd(&[0xC3]);
        lcd.write_data(&[0x15]);

        lcd.write_cmd(&[0xC4]);
        lcd.write_data(&[0x20]);

        lcd.write_cmd(&[0xC6]);
        lcd.write_data(&[0x0F]);

        lcd.write_cmd(&[0xD0]);
        lcd.write_data(&[0xA4, 0xA1]);

        lcd.write_cmd(&[0xD6]);
        lcd.write_data(&[0xA1]);

        lcd.write_cmd(&[0xE0]);
        lcd.write_data(&[
            0x70, 0x05, 0x0A, 0x0B, 0x0A, 0x27, 0x2F, 0x44, 0x47, 0x37, 0x14, 0x14, 0x29, 0x2F,
        ]);

        lcd.write_cmd(&[0xE1]);
        lcd.write_data(&[
            0x70, 0x07, 0x0C, 0x08, 0x08, 0x04, 0x2F, 0x33, 0x46, 0x18, 0x15, 0x15, 0x2B, 0x2D,
        ]);

        lcd.write_cmd(&[0x21, 0x29, 0x2C]);
        lcd.clear(0x0000);

        lcd
    }

    pub fn write_cmd(&mut self, cmd: &[u8]) {
        self.dc(Level::Low);
        self.spi.write(cmd).unwrap();
    }

    fn flush_buf(&mut self) {
        self.dc(Level::High);
        self.spi.write(&self.buf).unwrap();
        self.buf.clear();
    }

    pub fn write_data(&mut self, data: &[u8]) {
        self.dc(Level::High);
        self.spi.write(data).unwrap();
    }

    pub fn draw(&mut self, rect: Rect, color: u16) {
        self.write_cmd(&[0x2a]);
        let start_x = rect.start_x.to_be_bytes();
        let end_x = rect.end_x.to_be_bytes();
        self.write_data(&[start_x[0], start_x[1], end_x[0], end_x[1]]);
        self.write_cmd(&[0x2b]);
        let start_y = rect.start_y.to_be_bytes();
        let end_y = rect.end_y.to_be_bytes();
        self.write_data(&[start_y[0], start_y[1], end_y[0], end_y[1]]);
        self.write_cmd(&[0x2c]);

        let data = color.to_be_bytes();
        let area = (rect.end_x - rect.start_x + 1) * (rect.end_y - rect.start_y + 1);
        let area = area as usize;

        for _ in 0..area {
            self.buf.extend_from_slice(data.as_slice());
        }

        self.flush_buf();
    }

    pub fn clear(&mut self, color: u16) {
        self.draw(
            Rect {
                start_x: 0,
                end_x: 239,
                start_y: 0,
                end_y: 239,
            },
            color,
        );
    }

    fn rst(&mut self, level: Level) {
        self.rst.set_level(level);
    }

    fn dc(&mut self, level: Level) {
        self.dc.set_level(level);
    }

    // fn cs(&mut self, level: Level) {
    //     self.cs.set_level(level);
    // }

    fn blk(&mut self, level: Level) {
        self.blk.set_level(level);
    }
}
