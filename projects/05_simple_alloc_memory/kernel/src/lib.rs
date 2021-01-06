#![no_std]
#![feature(llvm_asm)]

const UART_BASE_ADDRESS: usize = 0x1000_0000;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::uart::Uart::new($crate::UART_BASE_ADDRESS), $($args)+);
    });
}

#[macro_export]
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
    let hartid = rv64::reg::MHartId::read();

    // DONE: 最適化でおかしくなる問題は解消
    // TODO: 複数CPUでポートを取り合う問題は解消せず
    if hartid == 0 {
        crate::uart::Uart::new(UART_BASE_ADDRESS).init();
        println!("Hello World.");
    }

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
        rv64::reg::Tp::write(hartid);
    }
    // Sモードに移行する
    {
        rv64::isa::mret();
    }
    loop { rv64::isa::wfi(); }
}

fn kmain() -> ! {
    let cpuid = rv64::cpuid();
    if cpuid == 0 {
        println!("Transfer to Superviser mode, Success.");
        println!("mhertid: {}", cpuid);
        
        kmem::page::init();

        println!("Allocationg ...");

        let _ = kmem::page::alloc(64);
        let p1 = kmem::page::alloc(1);
        let p2 = kmem::page::alloc(1);
        let _ = kmem::page::alloc(1);

        kmem::page::print_page();

        println!();

        kmem::page::dealloc(p1);
        kmem::page::dealloc(p2);

        kmem::page::print_page();
        println!();

        let p3 = kmem::page::alloc(3);

        kmem::page::print_page();
        println!();
        println!("{:p}", p3);
    }

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

mod rv64;
mod kmem;
mod uart;
