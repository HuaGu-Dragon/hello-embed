#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use bt_hci::controller::ExternalController;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, TICK_HZ, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::mcpwm::operator::PwmPinConfig;
use esp_hal::mcpwm::timer::PwmWorkingMode;
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::ble::controller::BleConnector;
use hello_embed::lcd::Lcd;
use panic_rtt_target as _;
use trouble_host::prelude::*;

extern crate alloc;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 1;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32s3 -o unstable-hal -o alloc -o embassy -o wifi -o ble-trouble -o probe-rs -o defmt -o panic-rtt-target -o embedded-test -o ci -o zed -o 1.95.0-x86_64-pc-windows-msvc

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let clock_cfg = PeripheralClockConfig::with_frequency(Rate::from_mhz(40)).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);

    // connect operator0 to timer0
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    // connect operator0 to pin
    let mut _pwm_pin = mcpwm
        .operator0
        .with_pin_a(peripherals.GPIO38, PwmPinConfig::UP_ACTIVE_HIGH);

    // start timer with timestamp values in the range of 0..=99 and a frequency
    // of 20 kHz
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(99, PwmWorkingMode::Increase, Rate::from_khz(20))
        .unwrap();
    mcpwm.timer0.start(timer_clock_cfg);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 128 * 1024);
    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");
    // find more examples https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    let transport = BleConnector::new(peripherals.BT, Default::default()).unwrap();
    let ble_controller = ExternalController::<_, 1>::new(transport);
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();
    let _stack = trouble_host::new(ble_controller, &mut resources);

    let mut lcd = Lcd::init_spi2(
        peripherals.SPI2,
        peripherals.GPIO11,
        peripherals.GPIO12,
        peripherals.GPIO13,
        peripherals.GPIO21,
        peripherals.GPIO40,
        peripherals.GPIO47,
        peripherals.GPIO48,
    )
    .await;

    let time = Instant::now();

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        hello_embed::renderer::render(&mut lcd, time.elapsed().as_ticks() as f32 / TICK_HZ as f32)
            .await;
        // for timestamp in 0..=100 {
        //     pwm_pin.set_timestamp(timestamp);
        //     Timer::after(Duration::from_millis(20)).await;
        // }
        // for timestamp in 0..=100 {
        //     pwm_pin.set_timestamp(100 - timestamp);
        //     Timer::after(Duration::from_millis(20)).await;
        // }
        // channel.start_duty_fade(1, 12, 1000).unwrap();
        // while channel.is_duty_fade_running() {}
        // channel.start_duty_fade(12, 1, 1000).unwrap();
        // while channel.is_duty_fade_running() {}
        // info!("Hello world!");
        // Timer::after(Duration::from_secs(1)).await;
        // lcd.draw(
        //     Rect {
        //         start_x: 0,
        //         start_y: 0,
        //         end_x: 100,
        //         end_y: 100,
        //     },
        //     0xFFFF,
        // );
        // led.set_high();
        // Timer::after(Duration::from_secs(1)).await;
        // lcd.draw(
        //     Rect {
        //         start_x: 0,
        //         start_y: 0,
        //         end_x: 100,
        //         end_y: 100,
        //     },
        //     0x0000,
        // );
        // led.set_low();
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
