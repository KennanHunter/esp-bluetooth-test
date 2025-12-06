#![no_std]
#![no_main]

extern crate alloc;

use esp_backtrace as _;
use esp_bluetooth_test::drv2285::{DRV2285, Direction};
use esp_hal::{
    clock::CpuClock,
    efuse,
    gpio::{Level, Output, OutputConfig},
    timer::timg::TimerGroup,
};
use log::info;
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    #[cfg(target_arch = "riscv32")]
    let sw_int = esp_hal::interrupt::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    esp_rtos::start(
        timg0.timer0,
        #[cfg(target_arch = "riscv32")]
        sw_int.software_interrupt0,
    );

    static RADIO: StaticCell<esp_radio::Controller<'static>> = StaticCell::new();
    let radio = RADIO.init(esp_radio::init().unwrap());

    // Get device MAC address
    let _mac_bytes = efuse::Efuse::read_base_mac_address();

    // Initialize stepper motor driver on GPIO pins
    // Configure GPIO pins for stepper control
    let step_pin = Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default());
    let dir_pin = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    // Create DRV2285 driver instance
    let mut motor = DRV2285::new(step_pin, dir_pin);
    motor.set_step_pulse_duration(100); // 10 microsecond pulse width

    // Loop forever: jog forward a few steps
    info!("Starting stepper motor loop...");
    loop {
        motor.jog(Direction::Forward, 250, 5).await;
        embassy_time::Timer::after_millis(100).await;
    }
}
