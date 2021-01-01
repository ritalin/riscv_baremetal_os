#![no_std]
#![feature(llvm_asm)]

mod uart;
mod rv64;

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

    // Sモードへの移行を予約する
    {
        let mut status = rv64::reg::MStatus::read();
        status ^= rv64::CpuMode::Machine.higher();
        status |= rv64::CpuMode::Supervisor.higher();
        
        rv64::reg::MStatus::write(status);
    }
    // mepc (M Exception Program Counter)の変更を予約する
    {
        rv64::reg::Mepc::write(kmain as u64);
    }
    // ページングを無効化する
    {
        rv64::reg::SAtp::write(0);
    }
    // 割り込みと例外をSモードに移乗させるよう予約する
    {
        // TODO:
    }
    // タイマー割り込みを有効化する
    {
        // TODO:
    }
    // 現在のCPUを保存する
    {
        let hartid = rv64::reg::MHartId::read();
        rv64::reg::Tp::write(hartid);
    }
    // Sモードに移行する
    {
        rv64::isa::mret();
    }
    loop { rv64::isa::wfi(); }
}

fn kmain() -> ! {
    println!("Transfer to Superviser mode, Success.");
    loop { rv64::isa::wfi(); }
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    abort();
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    // 何もせず、無限ループする
    loop { rv64::isa::wfi(); }
}
