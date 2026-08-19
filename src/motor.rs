use esp_hal::gpio::DriveMode;
use esp_hal::gpio::interconnect::PeripheralOutput;
use esp_hal::ledc::channel::Channel;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::channel::Number;
use esp_hal::ledc::timer::Timer;
use esp_hal::ledc::timer::config::Duty;
use esp_hal::ledc::timer::{self, TimerIFace};
use esp_hal::ledc::{LSGlobalClkSource, Ledc, LowSpeed, channel};

use esp_hal::peripherals::LEDC;
use esp_hal::time::Rate;

extern crate alloc;

pub struct Motor<'d> {
    ledc: Ledc<'d>,
    timer: Timer<'d, LowSpeed>,
    channel: Channel<'d, LowSpeed>,
}

impl<'d> Motor<'d> {
    pub fn new(
        instance: LEDC<'d>,
        duty: Duty,
        freq: Rate,
        duty_pct: u8,
        output: impl PeripheralOutput<'d>,
    ) -> Self {
        let mut ledc = Ledc::new(instance);
        ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

        let mut timer = ledc.timer::<LowSpeed>(esp_hal::ledc::timer::Number::Timer1);

        let mut channel = ledc.channel(Number::Channel1, output);

        timer
            .configure(timer::config::Config {
                duty,
                clock_source: timer::LSClockSource::APBClk,
                frequency: freq,
            })
            .unwrap();

        channel
            .configure(channel::config::Config {
                timer: timer.into(),
                duty_pct,
                drive_mode: DriveMode::PushPull,
            })
            .unwrap();

        Self {
            ledc,
            timer,
            channel,
        }
    }

    pub fn set_duty(&mut self, duty_pct: u8) {
        self.channel.set_duty(duty_pct).unwrap()
    }
}
