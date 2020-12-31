#![no_std]

#[no_mangle]
pub extern "C" fn __start() -> ! {
    let uart0 = 0x1000_0000 as *mut u8;

    // TODO: 複数CPUのときに一斉に出力しあってカオスな結果になる
    // TODO: 1CPUでは一見良さそうに見えてしまう
    // TODO: releaseビルドでは、最適化されてちゃんとした出力にならない
    for c in b"Hello World".iter() {
        unsafe { *uart0 = *c as u8; }
    }

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
