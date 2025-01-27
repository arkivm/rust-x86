//! Functions to read and write debug registers.
//!
//! * The dr{0,1,2,3} registers are used to set break points.
//! * The dr6 register contains debug conditions that were sampled at the time
//!   the last debug exception.
//! * The dr7 register enables or disables breakpoints and sets breakpoint
//!   conditions.
//!
//! See Intel Vol. 3a Chapter 17, "Debug, Branch, Profile, TSC ... Features"

use bit_field::BitField;
use bitflags::bitflags;

/// An array list of all available breakpoint registers.
pub const BREAKPOINT_REGS: [Breakpoint; 4] = [
    Breakpoint::Dr0,
    Breakpoint::Dr1,
    Breakpoint::Dr2,
    Breakpoint::Dr3,
];

/// Write dr{0-3} register based on provided `reg` enum.
pub unsafe fn dr_write(reg: Breakpoint, val: usize) {
    match reg {
        Breakpoint::Dr0 => dr0_write(val),
        Breakpoint::Dr1 => dr1_write(val),
        Breakpoint::Dr2 => dr2_write(val),
        Breakpoint::Dr3 => dr3_write(val),
    }
}

/// Read dr{0-3} register based on provided `reg` enum.
pub unsafe fn dr(reg: Breakpoint) -> usize {
    match reg {
        Breakpoint::Dr0 => dr0(),
        Breakpoint::Dr1 => dr1(),
        Breakpoint::Dr2 => dr2(),
        Breakpoint::Dr3 => dr3(),
    }
}

/// Read dr0.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr0() -> usize {
    let ret: usize;
    asm!("mov %dr0, {}", out(reg) ret, options(att_syntax));
    ret
}

/// Write dr0.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr0_write(val: usize) {
    asm!("mov {}, %dr0", in(reg) val, options(att_syntax));
}

/// Read dr1.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr1() -> usize {
    let ret: usize;
    asm!("mov %dr1, {}", out(reg) ret, options(att_syntax));
    ret
}

/// Write dr1.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr1_write(val: usize) {
    asm!("mov {}, %dr1", in(reg) val, options(att_syntax));
}

/// Read dr2.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr2() -> usize {
    let ret: usize;
    asm!("mov %dr2, {}", out(reg) ret, options(att_syntax));
    ret
}

/// Write dr2.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr2_write(val: usize) {
    asm!("mov {}, %dr2", in(reg) val, options(att_syntax));
}

/// Read dr3.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr3() -> usize {
    let ret: usize;
    asm!("mov %dr3, {}", out(reg) ret, options(att_syntax));
    ret
}

/// Write dr3.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr3_write(val: usize) {
    asm!("mov {}, %dr3", in(reg) val, options(att_syntax));
}

bitflags! {
    /// Debug register 6 (dr6) flags.
    pub struct Dr6: usize {
        /// B0 breakpoint condition detected
        ///
        /// # Notes
        ///
        /// The flag is set if the condition described for the breakpoint by
        /// the LENn, and R/Wn flags in debug control register DR7 is true. They
        /// may or may not be set if the breakpoint is not enabled by the Ln or
        /// the Gn flags in register DR7. Therefore on a #DB, a debug handler
        /// should check only those B0-B3 bits which correspond to an enabled
        /// breakpoint.
        const B0 = 0b0001;

        /// B1 breakpoint condition detected
        ///
        /// # Notes
        ///
        /// The flag is set if the condition described for the breakpoint by
        /// the LENn, and R/Wn flags in debug control register DR7 is true. They
        /// may or may not be set if the breakpoint is not enabled by the Ln or
        /// the Gn flags in register DR7. Therefore on a #DB, a debug handler
        /// should check only those B0-B3 bits which correspond to an enabled
        /// breakpoint.
        const B1 = 0b0010;

        /// B2 breakpoint condition detected
        ///
        /// # Notes
        ///
        /// The flag is set if the condition described for the breakpoint by
        /// the LENn, and R/Wn flags in debug control register DR7 is true. They
        /// may or may not be set if the breakpoint is not enabled by the Ln or
        /// the Gn flags in register DR7. Therefore on a #DB, a debug handler
        /// should check only those B0-B3 bits which correspond to an enabled
        /// breakpoint.
        const B2 = 0b0100;

        /// B3 breakpoint condition detected
        ///
        /// # Notes
        ///
        /// The flag is set if the condition described for the breakpoint by
        /// the LENn, and R/Wn flags in debug control register DR7 is true. They
        /// may or may not be set if the breakpoint is not enabled by the Ln or
        /// the Gn flags in register DR7. Therefore on a #DB, a debug handler
        /// should check only those B0-B3 bits which correspond to an enabled
        /// breakpoint.
        const B3 = 0b1000;

        /// BD debug register access detected
        ///
        /// Indicates that the next instruction in the instruction stream
        /// accesses one of the debug registers.
        ///
        /// This flag is enabled when the GD (general detect) flag in debug
        /// control register DR7 is set.
        const BD = 1 << 13;

        /// BS single step
        ///
        /// Indicates (when set) that the debug exception was triggered by the
        /// single- step execution mode (enabled with the TF flag in the EFLAGS
        /// register).
        const BS = 1 << 14;

        /// BT task switch
        ///
        /// Indicates (when set) that the debug exception resulted from a task
        /// switch where the T flag (debug trap flag) in the TSS of the target
        /// task was set.
        const BT = 1 << 15;

        /// Enables (when set) advanced debugging of RTM transactional regions.
        const RTM = 1 << 16;
    }
}

/// Read dr6.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr6() -> Dr6 {
    let ret: usize;
    asm!("mov %dr6, {}", out(reg) ret, options(att_syntax));
    Dr6::from_bits_truncate(ret)
}

/// Write dr6.
///
/// # Notes
///
/// Certain debug exceptions may clear bits 0-3. The remaining contents of the
/// DR6 register are never cleared by the processor. To avoid confusion in
/// identifying debug exceptions, debug handlers should clear the register
/// (except bit 16, which they should set) before returning to the interrupted
/// task).
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr6_write(val: Dr6) {
    asm!("mov {}, %dr6", in(reg) val.bits, options(att_syntax));
}

/// Specifies available hardware breakpoints.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Breakpoint {
    Dr0 = 0,
    Dr1 = 1,
    Dr2 = 2,
    Dr3 = 3,
}

/// Specifies the  breakpoint condition for a corresponding breakpoint.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BreakCondition {
    /// 00 — Break on instruction execution only.
    Instructions = 0b00,
    /// 01 — Break on data writes only.
    DataWrites = 0b01,
    /// 10 — Break on I/O reads or writes.
    ///
    /// # Notes
    /// For this type to be available, the DE (debug extensions) flag in control
    /// register CR4 must be set.
    IoReadsWrites = 0b10,
    /// 11 — Break on data reads or writes but not instruction fetches.
    DataReadsWrites = 0b11,
}

/// Specify the size of the memory location at the address specified in the
/// corresponding breakpoint address register (DR0 through DR3).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BreakSize {
    /// 00 — 1-byte length.
    Bytes1 = 0b00,
    /// 01 — 2-byte length.
    Bytes2 = 0b01,
    /// 10 — 8 byte length (or undefined, on older processors).
    Bytes8 = 0b10,
    /// 11 — 4-byte length.
    Bytes4 = 0b11,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dr7(pub usize);

impl Default for Dr7 {
    fn default() -> Self {
        Self(1 << 10)
    }
}

impl Dr7 {
    fn set_bp(&mut self, bp: Breakpoint, global: bool, val: bool) {
        let bp = bp as usize;
        assert!(bp < 4);
        let idx = if global { bp + 1 } else { bp };
        self.0.set_bit(idx, val);
    }

    fn set_bc(&mut self, bp: Breakpoint, bc: BreakCondition) {
        let idx = 16 + (bp as usize * 4);
        assert!(idx == 16 || idx == 20 || idx == 24 || idx == 28);
        self.0.set_bits(idx..=idx + 1, bc as usize);
    }

    fn set_bs(&mut self, bp: Breakpoint, bs: BreakSize) {
        let idx = 18 + (bp as usize * 4);
        assert!(idx == 18 || idx == 22 || idx == 26 || idx == 30);
        self.0.set_bits(idx..=idx + 1, bs as usize);
    }

    /// Enables the breakpoint condition for the associated breakpoint.
    ///
    /// - `bp`: The breakpoint to enable.
    /// - `global`: If true, the breakpoint is global (e.g., never reset on task
    ///   switches). If false, the CPU resets the flag (disables bp) on task
    ///   switch.
    pub fn enable_bp(&mut self, bp: Breakpoint, bc: BreakCondition, bs: BreakSize, global: bool) {
        assert!(
            !(bc == BreakCondition::Instructions && bs != BreakSize::Bytes1),
            "If bc is 00 (instruction execution), then the bs field should be 00"
        );
        self.set_bp(bp, global, true);
        self.set_bc(bp, bc);
        self.set_bs(bp, bs);
    }

    /// Disables the breakpoint condition for the associated breakpoint.
    ///
    /// - `bp`: The breakpoint to disable.
    /// - `global`: If true, the breakpoint is global (e.g., never reset on task
    ///   switches). If false, the CPU resets the flag (disables bp) on task
    ///   switch.
    pub fn disable_bp(&mut self, bp: Breakpoint, global: bool) {
        self.set_bp(bp, global, false);
    }

    /// Local exact breakpoint enable.
    ///
    /// This flag causes the processor to detect the exact instruction that
    /// caused a data breakpoint condition. This feature is not supported in the
    /// P6 family processors, later IA-32 processors, and Intel 64 processors.
    ///
    /// For backward and forward compatibility with other Intel processors,
    /// Intel recommends that the LE flag be set to 1 if exact breakpoints are
    /// required.
    pub fn enable_exact_local_bp(&mut self) {
        self.0.set_bit(8, true);
    }

    /// Global exact breakpoint enable.
    ///
    /// This flag causes the processor to detect the exact instruction that
    /// caused a data breakpoint condition. This feature is not supported in the
    /// P6 family processors, later IA-32 processors, and Intel 64 processors.
    ///
    /// For backward and forward compatibility with other Intel processors,
    /// Intel recommends that the GE flag be set to 1 if exact breakpoints are
    /// required.
    pub fn enable_exact_global_bp(&mut self) {
        self.0.set_bit(9, true);
    }

    /// Enables advanced debugging of RTM transactional regions.
    ///
    /// # Note
    /// This advanced debugging is enabled only if IA32_DEBUGCTL.RTM is also
    /// set.
    pub fn enable_rtm(&mut self) {
        self.0.set_bit(11, true);
    }

    /// Enables debug-register protection, which causes a debug exception to be
    /// generated prior to any MOV instruction that accesses a debug register.
    pub fn enable_general_detect(&mut self) {
        self.0.set_bit(13, true);
    }
}

/// Read dr7.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr7() -> Dr7 {
    let ret: usize;
    asm!("mov %dr7, {}", out(reg) ret, options(att_syntax));
    Dr7(ret)
}

/// Write dr7.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn dr7_write(val: Dr7) {
    asm!("mov {}, %dr7", in(reg) val.0, options(att_syntax));
}
