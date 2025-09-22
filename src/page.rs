use core::arch::{asm, global_asm};

use crate::{
    alloc::{HEAP_SIZE, HEAP_START, PAGE_SIZE, align_val, dealloc, zalloc},
    kmem, println,
};

global_asm!(
    ".section .rodata
.global TEXT_START
TEXT_START: .dword __text_start

.global TEXT_END
TEXT_END: .dword __text_end

.global RODATA_START
RODATA_START: .dword __rodata_start

.global RODATA_END
RODATA_END: .dword __rodata_end

.global DATA_START
DATA_START: .dword __data_start

.global DATA_END
DATA_END: .dword __data_end

.global BSS_START
BSS_START: .dword __bss_start

.global BSS_END
BSS_END: .dword __bss_end

.global STACK_START
STACK_START: .dword __stack_start

.global STACK_END
STACK_END: .dword __stack_end
"
);

unsafe extern "C" {
    pub static TEXT_START: usize;
    pub static TEXT_END: usize;
    pub static RODATA_START: usize;
    pub static RODATA_END: usize;
    pub static DATA_START: usize;
    pub static DATA_END: usize;
    pub static BSS_START: usize;
    pub static BSS_END: usize;
    pub static STACK_START: usize;
    pub static STACK_END: usize;
}

pub fn init() {
    let root_ptr = kmem::get_page_table();
    let root_u = root_ptr as usize;
    let mut root = unsafe { root_ptr.as_mut().unwrap() };
    let kheap_head = kmem::get_head() as usize;
    let total_pages = kmem::get_num_allocations();
    println!();
    println!();
    unsafe {
        println!("TEXT:   0x{:x} -> 0x{:x}", TEXT_START, TEXT_END);
        println!("RODATA: 0x{:x} -> 0x{:x}", RODATA_START, RODATA_END);
        println!("DATA:   0x{:x} -> 0x{:x}", DATA_START, DATA_END);
        println!("BSS:    0x{:x} -> 0x{:x}", BSS_START, BSS_END);
        println!("STACK:  0x{:x} -> 0x{:x}", STACK_START, STACK_END);
        println!(
            "HEAP:   0x{:x} -> 0x{:x}",
            kheap_head,
            kheap_head + total_pages * 4096
        );
    }
    id_map_range(
        &mut root,
        kheap_head,
        kheap_head + total_pages * 4096,
        EntryBits::ReadWrite as i64,
    );
    unsafe {
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        id_map_range(
            root,
            HEAP_START,
            HEAP_START + num_pages,
            EntryBits::ReadWrite as i64,
        );
        id_map_range(root, TEXT_START, TEXT_END, EntryBits::ReadExecute as i64);
        id_map_range(
            root,
            RODATA_START,
            RODATA_END,
            EntryBits::ReadExecute as i64,
        );

        id_map_range(root, DATA_START, DATA_END, EntryBits::ReadWrite as i64);
        id_map_range(root, BSS_START, BSS_END, EntryBits::ReadWrite as i64);
        id_map_range(root, STACK_START, STACK_END, EntryBits::ReadWrite as i64);
    }

    map(
        root,
        0x1000_0000,
        0x1000_0000,
        EntryBits::ReadWrite as i64,
        0,
    );

    let root_ppn = root_u >> 12;
    let satp_val = 8 << 60 | root_ppn;
    unsafe {
        asm!("csrw satp, {}", in(reg) satp_val);
    }
}

#[repr(i64)]
pub enum EntryBits {
    Valid = 1 << 0,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    Accessed = 1 << 6,
    Dirty = 1 << 7,

    ReadWrite = Self::Read as i64 | Self::Write as i64,
    ReadExecute = Self::Read as i64 | Self::Execute as i64,
}
pub struct Entry {
    pub entry: i64,
}

impl Entry {
    pub fn is_valid(&self) -> bool {
        self.get_entry() & EntryBits::Valid as i64 != 0
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    pub fn is_leaf(&self) -> bool {
        self.get_entry() & 0xe != 0
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    pub fn set_entry(&mut self, entry: i64) {
        self.entry = entry;
    }

    pub fn get_entry(&self) -> i64 {
        self.entry
    }
}

pub struct Table {
    pub entries: [Entry; 512],
}

impl Table {
    pub fn len() -> usize {
        512
    }
}

pub fn map(root: &mut Table, vaddr: usize, paddr: usize, bits: i64, level: usize) {
    assert!(bits & 0xe != 0);

    let vpn = [
        (vaddr >> 12) & 0x1ff,
        (vaddr >> 22) & 0x1ff,
        (vaddr >> 30) & 0x1ff,
    ];

    let ppn = [
        (paddr >> 12) & 0x1ff,
        (paddr >> 22) & 0x1ff,
        (paddr >> 30) & 0x1ff,
    ];

    let mut v = &mut root.entries[vpn[2]];

    for i in (level..2).rev() {
        if !v.is_valid() {
            let page = zalloc(1);

            v.set_entry((page as i64 >> 2) | EntryBits::Valid as i64);
        }

        let entry = ((v.get_entry() & !0x3ff) << 2) as *mut Entry;
        v = unsafe { entry.add(vpn[i]).as_mut().unwrap() };
    }

    let entry = (ppn[2] << 28) as i64
        | (ppn[1] << 19) as i64
        | (ppn[0] << 10) as i64
        | bits
        | EntryBits::Valid as i64;

    v.set_entry(entry);
}

pub fn unmap(root: &mut Table) {
    for lv2 in 0..Table::len() {
        let entry_lv2 = &root.entries[lv2];
        if entry_lv2.is_valid() && entry_lv2.is_branch() {
            let memaddr_lv1 = (entry_lv2.get_entry() & !0x3ff) << 2;
            let table_lv1 = unsafe { (memaddr_lv1 as *mut Table).as_mut().unwrap() };
            for lv1 in 0..Table::len() {
                let entry_lv1 = &table_lv1.entries[lv1];
                if entry_lv1.is_valid() && entry_lv1.is_branch() {
                    let memaddr_lv0 = (entry_lv1.get_entry() & !0x3ff) << 2;
                    dealloc(memaddr_lv0 as *mut u8);
                }
            }
            dealloc(memaddr_lv1 as *mut u8);
        }
    }
}

pub fn virt_to_phys(root: &Table, vaddr: usize) -> Option<usize> {
    let vpn = [
        (vaddr >> 12) & 0x1ff,
        (vaddr >> 22) & 0x1ff,
        (vaddr >> 30) & 0x1ff,
    ];

    let mut v = &root.entries[vpn[2]];
    for i in (0..=2).rev() {
        if v.is_invalid() {
            break;
        } else if v.is_leaf() {
            let off_mask = (1 << (12 + i * 9)) - 1;
            let vaddr_pgoff = vaddr & off_mask;
            let addr = ((v.get_entry() << 2) as usize) & !off_mask;
            return Some(addr | vaddr_pgoff);
        }

        let entry = ((v.get_entry() & !0x3ff) << 2) as *const Entry;

        v = unsafe { entry.add(vpn[i - 1]).as_ref().unwrap() };
    }

    None
}

/// Creates a 1 to 1 mapping of virtual memory to physical memory
/// for use in kernel internals
pub fn id_map_range(root: &mut Table, start: usize, end: usize, bits: i64) {
    let mut memaddr = start & !(PAGE_SIZE - 1);

    let num_kb_pages = (align_val(end, 12) - memaddr) / PAGE_SIZE;

    for _ in 0..num_kb_pages {
        map(root, memaddr, memaddr, bits, 0);
        memaddr += 1 << 12;
    }
}
