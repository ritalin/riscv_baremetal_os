pub struct MStatus {}
pub struct Mepc {}
pub struct MHartId {}

pub struct SAtp {}

pub struct Tp {}

impl MStatus {
    pub fn read() -> u64 {
        unsafe {
            let mut v;
            llvm_asm!("csrr $0, mstatus":"=r"(v):::"volatile");
            return v;
        }
    }

    pub fn write(v: u64) {
        unsafe {
            llvm_asm!("csrw mstatus, $0"::"r"(v)::"volatile");
        }
    }
}
impl Mepc {
    pub fn write(v: u64) {
        unsafe {
            llvm_asm!("csrw mepc, $0"::"r"(v)::"volatile");
        }
    }
}
impl MHartId {
    pub fn read() -> u64 {
        unsafe {
            let mut v;
            llvm_asm!("csrr $0, mhartid":"=r"(v):::"volatile");
            return v;
        }
    }   
}
impl SAtp {
    pub fn write(v: u64) {
        unsafe {
            llvm_asm!("csrw satp, $0"::"r"(v)::"volatile");
        }
    }
}
impl Tp {
    pub fn read() -> u64 {
        unsafe {
            let mut v;
            llvm_asm!("mv $0, tp":"=r"(v):::"volatile");
            return v;
        }
    }   
    pub fn write(v: u64) {
        unsafe {
            llvm_asm!("mv tp, $0"::"r"(v):::"volatile");
        }
    }   
}