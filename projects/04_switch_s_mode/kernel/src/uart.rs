use core::fmt::Write;
use core::fmt::Error;

pub struct Uart {
    base_addr: usize,
}

impl Write for Uart {
    fn write_str(&mut self, str: &str) -> Result<(), Error> {
        for c in str.bytes() {
            self.put(c);
        }
        return Ok(());
    }
}

impl Uart {
    pub fn new(base_addr: usize) -> Self {
        Uart { base_addr: base_addr }
    }

    pub fn init(&mut self) {
        let p = self.base_addr as *mut u8;
        unsafe {
            let lcr = 1 << 0 | 1 << 1;

            // LCR (offset = 3)のWSL0(bit = 0)とWSL1(bit = 1)を初期化する
            p.add(3).write_volatile(lcr);
            // FCR (offset = 2)のFIFI enable (bit = 0)を有効化する
            p.add(2).write_volatile(1 << 0);
            // IER (offset = 1)のREceive Interruptを有効化する
            p.add(1).write_volatile(1 << 0);

            // LCRのDLABビットを立てる
            p.add(3).write_volatile(lcr | 1 << 7);

            use core::convert::TryInto;
            let div: u16 = 592;
            let div_least: u8 = (div & 0xFF).try_into().unwrap();
            let div_most: u8 = (div >> 8).try_into().unwrap();

            // DLLのBit0とDLMのBit8を初期化する
            p.add(0).write_volatile(div_least);
            p.add(1).write_volatile(div_most);

            // LCRのDLABビットをおろす
            p.add(3).write_volatile(lcr);
        }
    }

    pub fn put(&mut self, c: u8) {
        let p = self.base_addr as *mut u8;
        unsafe {
            p.add(0).write_volatile(c);
        }
    }
}