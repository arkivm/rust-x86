/// The focus on this file is to describe the data-structures
/// for IA-32e paging mode.
use core::fmt;

pub type PAddr = u64;
pub type VAddr = u64;

pub const BASE_PAGE_SIZE: u64 = 4096;

bitflags! {
    flags PML4Entry: u64 {
        /// Read/write; if 0, writes may not be allowed to the 512-GByte region
        /// controlled by this entry (see Section 4.6)
        const PML4_RW      = 0b00000010,
        /// User/supervisor; if 0, user-mode accesses are not allowed
        /// to the 512-GByte region controlled by this entry.
        const PML4_US      = 0b00000100,
        /// Page-level write-through; indirectly determines the memory type used to
        /// access the page-directory-pointer table referenced by this entry.
        const PML4_PWT     = 0b00001000,
        /// Page-level cache disable; indirectly determines the memory type used to
        /// access the page-directory-pointer table referenced by this entry.
        const PML4_PCD     = 0b00010000,
        /// Accessed; indicates whether this entry has been used for linear-address translation.
        const PML4_A       = 0b00100000,
        /// If IA32_EFER.NXE = 1, execute-disable
        /// If 1, instruction fetches are not allowed from the 512-GByte region.
        const PML4_XD      = 1 << 63,
    }
}

impl PML4Entry {
    pub fn new(&mut self, pdpt: PAddr) {
        assert!(pdpt % BASE_PAGE_SIZE == 0);
        self.bits = pdpt;
    }
}

impl fmt::Debug for PML4Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}

bitflags! {
    flags PDPTEntry: u64 {
        /// Present; must be 1 to map a 1-GByte page or reference a page directory.
        const PDPT_P       = 0b00000001,
        /// Read/write; if 0, writes may not be allowed to the 1-GByte region controlled by this entry
        const PDPT_RW      = 0b00000010,
        /// User/supervisor; user-mode accesses are not allowed to the 1-GByte region controlled by this entry.
        const PDPT_US      = 0b00000100,
        /// Page-level write-through.
        const PDPT_PWT     = 0b00001000,
        /// Page-level cache disable.
        const PDPT_PCD     = 0b00010000,
        /// Accessed; if PDPT_PS set indicates whether software has accessed the 1-GByte page
        /// else indicates whether this entry has been used for linear-address translation
        const PDPT_A       = 0b00100000,
        /// Dirty; if PDPT_PS indicates whether software has written to the 1-GByte page referenced by this entry.
        /// else ignored.
        const PDPT_D       = 0b01000000,
        /// Page size; if set this entry maps a 1-GByte page; otherwise, this entry references a page directory.
        /// if not PDPT_PS this is ignored.
        const PDPT_PS      = 0b10000000,
        /// Global; if PDPT_PS && CR4.PGE = 1, determines whether the translation is global; ignored otherwise
        /// if not PDPT_PS this is ignored.
        const PDPT_G       = 1<<8,
        /// Indirectly determines the memory type used to access the 1-GByte page referenced by this entry.
        const PDPT_PAT     = 1<<12,
        /// If IA32_EFER.NXE = 1, execute-disable
        /// If 1, instruction fetches are not allowed from the 512-GByte region.
        const PDPT_XD      = 1 << 63,
    }
}

impl PDPTEntry {
    pub fn new(&mut self) {
        self.bits = 0;
    }
}

impl fmt::Debug for PDPTEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}


bitflags! {
    flags PDEntry: u64 {
        /// Present; must be 1 to map a 2-MByte page or reference a page table.
        const PD_P       = 0b00000001,
        /// Read/write; if 0, writes may not be allowed to the 2-MByte region controlled by this entry
        const PD_RW      = 0b00000010,
        /// User/supervisor; user-mode accesses are not allowed to the 2-MByte region controlled by this entry.
        const PD_US      = 0b00000100,
        /// Page-level write-through.
        const PD_PWT     = 0b00001000,
        /// Page-level cache disable.
        const PD_PCD     = 0b00010000,
        /// Accessed; if PD_PS set indicates whether software has accessed the 2-MByte page
        /// else indicates whether this entry has been used for linear-address translation
        const PD_A       = 0b00100000,
        /// Dirty; if PD_PS indicates whether software has written to the 2-MByte page referenced by this entry.
        /// else ignored.
        const PD_D       = 0b01000000,
        /// Page size; if set this entry maps a 2-MByte page; otherwise, this entry references a page directory.
        const PD_PS      = 0b10000000,
        /// Global; if PD_PS && CR4.PGE = 1, determines whether the translation is global; ignored otherwise
        /// if not PD_PS this is ignored.
        const PD_G       = 1<<8,
        /// Indirectly determines the memory type used to access the 2-MByte page referenced by this entry.
        /// if not PD_PS this is ignored.
        const PD_PAT     = 1<<12,
        /// If IA32_EFER.NXE = 1, execute-disable
        /// If 1, instruction fetches are not allowed from the 512-GByte region.
        const PD_XD      = 1 << 63,
    }
}

impl PDEntry {
    pub fn new(&mut self) {
        self.bits = 0;
    }
}

impl fmt::Debug for PDEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}


bitflags! {
    flags PTEntry: u64 {
        /// Present; must be 1 to map a 4-KByte page.
        const PT_P       = 0b00000001,
        /// Read/write; if 0, writes may not be allowed to the 4-KByte region controlled by this entry
        const PT_RW      = 0b00000010,
        /// User/supervisor; user-mode accesses are not allowed to the 4-KByte region controlled by this entry.
        const PT_US      = 0b00000100,
        /// Page-level write-through.
        const PT_PWT     = 0b00001000,
        /// Page-level cache disable.
        const PT_PCD     = 0b00010000,
        /// Accessed; indicates whether software has accessed the 4-KByte page
        const PT_A       = 0b00100000,
        /// Dirty; indicates whether software has written to the 4-KByte page referenced by this entry.
        const PT_D       = 0b01000000,
        /// Global; if CR4.PGE = 1, determines whether the translation is global (see Section 4.10); ignored otherwise
        const PT_G       = 1<<8,
        /// If IA32_EFER.NXE = 1, execute-disable
        /// If 1, instruction fetches are not allowed from the 512-GByte region.
        const PT_XD      = 1 << 63,
    }
}

impl PTEntry {
    pub fn new(&mut self) {
        self.bits = 0;
    }
}

impl fmt::Debug for PTEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}
