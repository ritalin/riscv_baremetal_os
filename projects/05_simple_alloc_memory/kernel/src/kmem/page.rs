use core::{mem::size_of, ptr::null_mut};

extern "C" {
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
}

const PAGE_ORDER: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_ORDER;

static mut ALLOC_START: usize = 0;

struct MemoryPage {
    status: u8,
}
impl MemoryPage {
    fn clear(&mut self) {
        self.status = PageStatus::Empty.val();
    }

    fn is_empty(&self) -> bool {
        return self.status == PageStatus::Empty.val();
    }

    fn is_used(&self) -> bool {
        return self.status & PageStatus::Used.val() != 0;        
    }

    fn is_last(&self) -> bool {
        return self.status & PageStatus::Last.val() != 0;                
    }

    fn update_status(&mut self, status: PageStatus) {
        self.status |= status.val();
    }
}

#[repr(u8)]
enum PageStatus {
    Empty = 0,
    Used = 1 << 0,
    Last = 1 << 1,
}
impl PageStatus {
    fn val(self) -> u8 {
        return self as u8;
    }
}

struct HeapDiagnostics {
    offset: usize,
    max_pages: usize,
}
impl HeapDiagnostics {
    fn new() -> Self {
        unsafe {
            let max_pages = HEAP_SIZE / PAGE_SIZE;

            return HeapDiagnostics {
                offset: HEAP_START,
                max_pages: max_pages,
            };
        }
    }

    fn offset_as_ptr(&self) -> *const MemoryPage {
        return self.offset as *const MemoryPage;
    }

    fn offset_as_ptr_mut(&self) -> *mut MemoryPage {
        return self.offset as *mut MemoryPage;
    }

    fn limit_as_ptr(&self) -> *const MemoryPage {
        unsafe {
            return self.offset_as_ptr().add(self.max_pages);
        }
    }

    fn skip_empty(&self, offset: *mut MemoryPage, limit: *mut MemoryPage) -> usize {
        let mut n: usize = 0;
        let mut p = offset;

        unsafe {
            while p < limit {
                if (*p).is_empty() { 
                    p = p.add(1);
                    n += 1;
                    continue; 
                }
                break;
            }
        }
        return n;
    }

    fn count_used(&self, offset: *mut MemoryPage, limit: *mut MemoryPage) -> usize {
        let mut n: usize = 0;
        let mut p = offset;

        unsafe {
            while p < limit {
                if ! (*p).is_used() { break; }

                n += 1;
                if (*p).is_last() { break; }
                p = p.add(1);
            }
        }

        return n;
    }

    fn mark_as_used(&self, offset: *mut MemoryPage, pages: usize) -> *mut u8 {
        unsafe {
            for i in 0..pages {
                (*offset.add(i)).update_status(PageStatus::Used);
            }
            (*offset.add(pages-1)).update_status(PageStatus::Last);
        }

        return match self.page_to_address(offset) {
            None => null_mut(),
            Some(address) => address
        }
    }

    fn page_to_address(&self, page: *mut MemoryPage) -> Option<*mut u8> {
        unsafe {
            let address = ALLOC_START + (page as usize - self.offset) * PAGE_SIZE;

            return 
                if address < ALLOC_START || address >= ALLOC_START + self.max_pages * PAGE_SIZE {
                    None
                }
                else {
                    Some (address as *mut u8)
                }
            ;
        }
    }

    fn page_from_address(&self, p: *mut u8) -> Option<*mut MemoryPage> {
        unsafe {
            let page = self.offset + (p as usize - ALLOC_START) / PAGE_SIZE;
            
            return 
                if page < self.offset || page >= self.offset + HEAP_SIZE {
                    None
                }
                else {
                    Some (page as *mut MemoryPage)
                }
            ;
        }
    }

    fn mark_as_free(&self, offset: *mut MemoryPage) {
        unsafe {
            let mut p = offset.add(0);
            
            println!("gegin --->");
            while (*p).is_used() && (! (*p).is_last()) {
                (*p).clear(); 
                    
                p = p.add(1);
            }
            println!("<--- end");
            assert!((*p).is_last(), "double-free detected !");            
            
            (*p).clear(); 
        }
    }
}

pub fn init() {
    let h = HeapDiagnostics::new();

    unsafe {
        let p = h.offset_as_ptr_mut();

        for i in 0..h.max_pages {
            // ダングリングポインタ対策を施す
            (*p.add(i)).clear();
        }
        
        // ページ管理領域後から割り当てを開始する
        ALLOC_START = align_roundup(h.offset + h.max_pages * size_of::<MemoryPage,>(), PAGE_ORDER);
    }
}

pub fn alloc(pages: usize) -> *mut u8 {
    assert!(pages > 0);

    let heap = HeapDiagnostics::new();

    return match search_page(pages, &heap) {
        None => null_mut(),
        Some (p) => heap.mark_as_used(p, pages)
    };
}

fn search_page(pages: usize, h: &HeapDiagnostics) -> Option<*mut MemoryPage> {
    let offset = h.offset_as_ptr_mut();

    unsafe {
        for i in 0..(h.max_pages - pages) {
            let p = offset.add(i);

            if (*p).is_empty() && page_available(p, pages) {
                return Some(p);
            }
        }
    }
    return None;
}

fn page_available(from: *mut MemoryPage, pages: usize) -> bool {
    for j in 0..pages {
        unsafe { 
            if (*from.add(j)).is_used() { return false; }
        }
    }

    return true;
}

pub fn dealloc(p: *mut u8)  {
    assert!(! p.is_null());

    let h = HeapDiagnostics::new();

    dealloc_internal(p, &h);
}

fn dealloc_internal(p: *mut u8, h: &HeapDiagnostics)  {
    return match h.page_from_address(p) {
        None => assert!(false, "Page not found !"),
        Some (page_from) => h.mark_as_free(page_from)
    }
}

pub fn print_page() {
    let h = HeapDiagnostics::new();

    print_heap(&h);

    let n = print_page_alloc(&h);

    print_usage(n, h.max_pages);
}

fn print_heap(heap: &HeapDiagnostics) {
    unsafe {
        let alloc_begin = ALLOC_START;
        let alloc_end = ALLOC_START + heap.max_pages * PAGE_SIZE;

        println!("Page Allocation Table");
        println!("META: {:p} -> {:p}", heap.offset_as_ptr(), heap.limit_as_ptr());
        println!("PHYS: 0x{:x} -> 0x{:x}", alloc_begin, alloc_end);
    }
}

fn print_page_alloc(heap: &HeapDiagnostics) -> usize {
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let mut n = 0;

    unsafe {
        let mut offset: usize = 0;
        let mut p = heap.offset_as_ptr_mut();
        let limit = heap.limit_as_ptr() as *mut MemoryPage;

        while p < limit {
            let skipped = heap.skip_empty(p, limit);

            offset += skipped;
            p = p.add(skipped);

            if p >= limit { break; }

            let used = heap.count_used(p, limit);
            if used > 0 {
                // 結果をダンプする
                println!("0x{:x}: {:>3} page(s)", ALLOC_START + PAGE_SIZE * offset, used);

                p = p.add(used);
                n += used;
                offset += used;
            }          
        }
    }

    if n == 0 {
        println!("(unused)");
    }

    return n;
}

fn print_usage(n: usize, max_pages: usize) {
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Allocated: {:>6} pages ({:>10} bytes)", n, n * PAGE_SIZE);
    println!("Free:      {:>6} pages ({:>10} bytes)", max_pages - n, (max_pages - n) * PAGE_SIZE);
}

fn align_roundup(v: usize, order: usize) -> usize {
    let o = (1usize << order)-1;

    return (v + o) & !o;
}
