pub mod isa {
    #[no_mangle]
    #[inline(always)]
    pub fn wfi() {
        unsafe {
            llvm_asm!("wfi"::::"volatile");
        }
    }
}

pub mod reg 