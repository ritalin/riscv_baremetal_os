use core::mem::size_of;

extern "C" {
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
}

const PAGE_ORDER: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_ORDER;

static mut ALLOC_START: usize = 0;

struct MemoryPage {
    status: PageStatus,
}

#[repr(u8)]
enum PageStatus {
    Empty = 0,
    Used = 1 << 0,
    Last = 1 << 1,
}

pub fn init() {
    let num_pages = max_pages();

    unsafe {
        let p = HEAP_START as *mut MemoryPage;

        for i in 0..num_pages {
            // ダングリングポインタ対策を施す
            (*p.add(i)).clear();
        }

        ALLOC_START = align_roundup(HEAP_START + num_pages * size_of::<MemoryPage,>(), PAGE_ORDER);
    }
}

pub fn print_heap() {
    unsafe {
        let num_pages = max_pages();
        let heap_begin = HEAP_START as *mut MemoryPage;
        let heap_end = heap_begin.add(num_pages);
        let alloc_begin = ALLOC_START;
        let alloc_end = ALLOC_START + num_pages * PAGE_SIZE;

        println!("Page Allocation Table");
        println!("META: {:p} -> {:p}", heap_begin, heap_end);
        println!("PHYS: 0x{:x} -> 0x{:x}", alloc_begin, alloc_end);
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
}

#[inline(always)]
fn max_pages() -> usize {
    unsafe {
        return HEAP_SIZE / PAGE_SIZE;
    }
}

fn align_roundup(v: usize, order: usize) -> usize {
    let o = (1usize << order)-1;

    return (v + o) & !o;
}

impl MemoryPage {
    fn clear(&mut self) {
        self.status = PageStatus::Empty;
    }
}