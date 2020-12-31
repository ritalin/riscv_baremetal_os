#![no_std]

mod uart;

const UART_BASE_ADDRESS: usize = 0x1000_0000;

use core::fmt::Write;

macro_rules! print {
    ($($args:tt)+) => {
        let _ = write!(crate::uart::Uart::new(UART_BASE_ADDRESS), $($args)+);
    };
}

macro_rules! println {
    () => {
        print!("\r\n");
    };
    ($fmt:expr) => {
        print!(concat!($fmt, "\r\n"));
    };
    ($fmt:expr, $($args:tt)+) => {
        print!(concat!($fmt, "\r\n"), $($args)+);
    };
}

#[no_mangle]
pub extern "C" fn __start() -> ! {
    let mut uart0 = crate::uart::Uart::new(UART_BASE_ADDRESS);
    uart0.init();

    // DONE: 最適化でおかしくなる問題は解消
    // TODO: 複数CPUでポートを取り合う問題は解消せず
    println!("Hello World.");
    println!("This is second line.");
    println!("------------------------");
    println!("END");

    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    // 何もせず、無限ループする
    loop {}
}
