#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

//! The `starship` binary.
//!
//! # Starship
//! 

mod bsp;
mod console;
mod comet;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;

unsafe fn kernel_init() -> ! {
    if let Err(x) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    driver::driver_manager().init_drivers();

    // leave the unsafe world
    init_loader();
}

#[allow(unused)]
fn init_loader() -> ! {
    use console::console;

    comet::set_device(comet::Device::Starship);

    println!("Starting Starship...");
    println!("Running on {}", bsp::board_name());

    console().flush();
    console().clear_rx();

    println!("Requesting binary...");
    comet::request_binary();

    loop {
        if console().read_char() == '#' {
            if console().read_char() == 'C' {
                if console().read_char() == 'O' {
                    if console().read_char() == 'M' {
                        if console().read_char() == 'E' {
                            if console().read_char() == 'T' {
                                if console().read_char() == ':' {
                                    if console().read_char() == comet::Command::SendBinary.as_char() {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Found Comet, receiving binary...");

    let mut size: u32 = u32::from(console().read_char() as u8);
    size |= u32::from(console().read_char() as u8) << 8;
    size |= u32::from(console().read_char() as u8) << 16;
    size |= u32::from(console().read_char() as u8) << 24;

    println!("Binary size: {:.2} KiB", size as f64 / 1024.0);

    // trust the binary size to fit
    // TODO: check if binary size exceeds maximum size

    let kernel_addr: *mut u8 = bsp::memory::board_default_load_addr() as *mut u8;

    unsafe {
        for i in 0..size {
            let data = console().read_char() as u8;
            core::ptr::write_volatile(kernel_addr.offset(i as isize), data);
        }
    }

    println!("Binary received!");
    println!("Jumping to loaded kernel...");

    let kernel: fn() -> ! = unsafe {
        core::mem::transmute(kernel_addr)
    };

    kernel()
}
