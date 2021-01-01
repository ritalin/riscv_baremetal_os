pub mod reg;

#[repr(u8)]
#[allow(dead_code)]
pub enum CpuMode {
    User = 0,
    Supervisor = 1,
    Machine = 3,
}

impl CpuMode {
    pub fn active(self) -> u64 {
        return (self as u64) << 1;
    }

    pub fn higher(self) -> u64 {
        return self.active() << 9;
    }

    #[allow(dead_code)]
    pub fn middle(self) -> u64 {
        return self.active() << 6;
    }

    #[allow(dead_code)]
    pub fn lower(self) -> u64 {
        return self.active() << 3;
    }
}

pub fn cpuid() -> u64 {
    return crate::rv64::reg::Tp::read();
}

pub mod isa {
    #[no_mangle]
    #[inline(always)]
    pub fn wfi() {
        unsafe {
            llvm_asm!("wfi"::::"volatile");
        }
    }

    pub fn mret() {
        unsafe {
            llvm_asm!("mret"::::"volatile");
        }
    }
}
