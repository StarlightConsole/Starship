use super::memory::map::mmio;
use crate::{bsp::device_driver, console, driver as generic_driver};
use core::sync::atomic::{AtomicBool, Ordering};

static PL011_UART: device_driver::PL011Uart = unsafe {
    device_driver::PL011Uart::new(mmio::PL011_UART_START)
};

static GPIO: device_driver::GPIO = unsafe {
    device_driver::GPIO::new(mmio::GPIO_START)
};

fn post_init_uart() -> Result<(), &'static str> {
    console::register_console(&PL011_UART);
    Ok(())
}

fn post_init_gpio() -> Result<(), &'static str> {
    GPIO.map_pl011_uart();
    Ok(())
}

fn init_driver_uart() -> Result<(), &'static str> {
    let uart_descriptor = generic_driver::DeviceDriverDescriptor::new(&PL011_UART, Some(post_init_uart));
    generic_driver::driver_manager().register_driver(uart_descriptor);

    Ok(())
}

fn init_driver_gpio() -> Result<(), &'static str> {
    let gpio_descriptor = generic_driver::DeviceDriverDescriptor::new(&GPIO, Some(post_init_gpio));
    generic_driver::driver_manager().register_driver(gpio_descriptor);

    Ok(())
}

pub unsafe fn init() -> Result<(), &'static str> {
    static INIT_DONE: AtomicBool = AtomicBool::new(false);
    if INIT_DONE.load(Ordering::Relaxed) {
        return Err("Init already done");
    }

    init_driver_uart()?;
    init_driver_gpio()?;

    INIT_DONE.store(true, Ordering::Relaxed);
    Ok(())
}
