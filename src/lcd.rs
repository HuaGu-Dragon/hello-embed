use embassy_time::{Duration, Timer};
use esp_hal::{
    Blocking,
    gpio::{Level, Output},
    peripherals::{GPIO11, GPIO12, GPIO13, GPIO21, GPIO40, GPIO47, GPIO48, Peripherals, SPI2},
    spi::master::{Config, Spi},
};

pub struct Lcd<'a> {
    spi: Spi<'a, Blocking>,
    rst: Output<'static>,
    blk: Output<'static>,
    dc: Output<'static>,
    // cs: Output<'static>,
}

impl<'a> Lcd<'a> {
    pub async fn init_spi2(
        spi2: SPI2<'static>,
        gpio11: GPIO11<'static>,
        gpio12: GPIO12<'static>,
        gpio13: GPIO13<'static>,
        gpio21: GPIO21<'static>,
        gpio40: GPIO40<'static>,
        gpio47: GPIO47<'static>,
        gpio48: GPIO48<'static>,
    ) -> Self {
        let spi = Spi::new(spi2, Config::default())
            .unwrap()
            .with_cs(gpio48)
            .with_miso(gpio13)
            .with_mosi(gpio11)
            .with_sck(gpio12);

        let mut rst = Output::new(gpio21, Level::High, Default::default());
        let mut blk = Output::new(gpio40, Level::High, Default::default());
        let mut dc = Output::new(gpio47, Level::Low, Default::default());
        // let mut gpio48 = Output::new(gpio48, Level::High, Default::default());
        let mut lcd = Self {
            spi,
            rst,
            blk,
            dc,
            // cs: gpio48,
        };

        // lcd.rst(Level::High);
        // lcd.dc(Level::High);
        // lcd.blk(Level::Low);

        lcd.rst(Level::Low);
        Timer::after(Duration::from_millis(100)).await;
        lcd.rst(Level::High);
        Timer::after(Duration::from_millis(100)).await;

        lcd.blk(Level::High);
        Timer::after(Duration::from_millis(100)).await;

        lcd.write_cmd(&[0x11]);
        Timer::after(Duration::from_millis(120)).await;

        lcd.write_cmd(&[0xB2]);
        lcd.write_data(&[0x0C]);
        lcd.write_data(&[0x0C]);
        lcd.write_data(&[0x00]);
        lcd.write_data(&[0x33]);
        lcd.write_data(&[0x33]);

        lcd.write_cmd(&[0x35]);
        lcd.write_data(&[0x00]);

        lcd.write_cmd(&[0x36]);
        lcd.write_data(&[0x70]);
        lcd.write_data(&[0xA0]);

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
        lcd.write_data(&[0xA4]);
        lcd.write_data(&[0xA1]);

        lcd.write_cmd(&[0xD6]);
        lcd.write_data(&[0xA1]);

        lcd.write_cmd(&[0xE0]);
        lcd.write_data(&[0x70]);
        lcd.write_data(&[0x05]);
        lcd.write_data(&[0x0A]);
        lcd.write_data(&[0x0B]);
        lcd.write_data(&[0x0A]);
        lcd.write_data(&[0x27]);
        lcd.write_data(&[0x2F]);
        lcd.write_data(&[0x44]);
        lcd.write_data(&[0x47]);
        lcd.write_data(&[0x37]);
        lcd.write_data(&[0x14]);
        lcd.write_data(&[0x14]);
        lcd.write_data(&[0x29]);
        lcd.write_data(&[0x2F]);

        lcd.write_cmd(&[0xE1]);
        lcd.write_data(&[0x70]);
        lcd.write_data(&[0x07]);
        lcd.write_data(&[0x0C]);
        lcd.write_data(&[0x08]);
        lcd.write_data(&[0x08]);
        lcd.write_data(&[0x04]);
        lcd.write_data(&[0x2F]);
        lcd.write_data(&[0x33]);
        lcd.write_data(&[0x46]);
        lcd.write_data(&[0x18]);
        lcd.write_data(&[0x15]);
        lcd.write_data(&[0x15]);
        lcd.write_data(&[0x2B]);
        lcd.write_data(&[0x2D]);

        lcd.write_cmd(&[0x21]);
        lcd.write_cmd(&[0x29]);
        lcd.write_cmd(&[0x2C]);

        lcd
    }

    pub fn write_cmd(&mut self, cmd: &[u8]) {
        self.dc(Level::Low);
        self.spi.write(cmd).unwrap();
    }

    pub fn write_data(&mut self, data: &[u8]) {
        self.dc(Level::High);
        self.spi.write(data).unwrap();
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
