#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - Standby Control Register"]
    pub sbycr: SBYCR,
    _reserved1: [u8; 0x0e],
    #[doc = "0x1c - Module Stop Control Register A"]
    pub mstpcra: MSTPCRA,
    #[doc = "0x20 - System Clock Division Control Register"]
    pub sckdivcr: SCKDIVCR,
    _reserved3: [u8; 0x02],
    #[doc = "0x26 - System Clock Source Control Register"]
    pub sckscr: SCKSCR,
    _reserved4: [u8; 0x0a],
    #[doc = "0x31 - Memory Wait Cycle Control Register for Code Flash"]
    pub memwait: MEMWAIT,
    #[doc = "0x32 - Main Clock Oscillator Control Register"]
    pub mosccr: MOSCCR,
    _reserved6: [u8; 0x03],
    #[doc = "0x36 - High-Speed On-Chip Oscillator Control Register"]
    pub hococr: HOCOCR,
    _reserved7: [u8; 0x01],
    #[doc = "0x38 - Middle-Speed On-Chip Oscillator Control Register"]
    pub mococr: MOCOCR,
    _reserved8: [u8; 0x03],
    #[doc = "0x3c - Oscillation Stabilization Flag Register"]
    pub oscsf: OSCSF,
    _reserved9: [u8; 0x01],
    #[doc = "0x3e - Clock Out Control Register"]
    pub ckocr: CKOCR,
    _reserved10: [u8; 0x01],
    #[doc = "0x40 - Oscillation Stop Detection Control Register"]
    pub ostdcr: OSTDCR,
    #[doc = "0x41 - Oscillation Stop Detection Status Register"]
    pub ostdsr: OSTDSR,
    _reserved12: [u8; 0x0a],
    #[doc = "0x4c - Lower Power Operation Control Register"]
    pub lpopt: LPOPT,
    _reserved13: [u8; 0x14],
    #[doc = "0x61 - MOCO User Trimming Control Register"]
    pub mocoutcr: MOCOUTCR,
    #[doc = "0x62 - HOCO User Trimming Control Register"]
    pub hocoutcr: HOCOUTCR,
    _reserved15: [u8; 0x2f],
    #[doc = "0x92 - Snooze Control Register"]
    pub snzcr: SNZCR,
    _reserved16: [u8; 0x01],
    #[doc = "0x94 - Snooze End Control Register 0"]
    pub snzedcr0: SNZEDCR0,
    _reserved17: [u8; 0x03],
    #[doc = "0x98 - Snooze Request Control Register 0"]
    pub snzreqcr0: SNZREQCR0,
    _reserved18: [u8; 0x03],
    #[doc = "0x9f - Power Save Memory Control Register"]
    pub psmcr: PSMCR,
    #[doc = "0xa0 - Operating Power Control Register"]
    pub opccr: OPCCR,
    _reserved20: [u8; 0x01],
    #[doc = "0xa2 - Main Clock Oscillator Wait Control Register"]
    pub moscwtcr: MOSCWTCR,
    _reserved21: [u8; 0x07],
    #[doc = "0xaa - Sub Operating Power Control Register"]
    pub sopccr: SOPCCR,
    _reserved22: [u8; 0x15],
    #[doc = "0xc0 - Reset Status Register 1"]
    pub rstsr1: RSTSR1,
    _reserved23: [u8; 0x1e],
    #[doc = "0xe0 - Voltage Monitor 1 Circuit Control Register"]
    pub lvd1cr1: LVD1CR1,
    #[doc = "0xe1 - Voltage Monitor 1 Circuit Status Register"]
    pub lvd1sr: LVD1SR,
    #[doc = "0xe2 - Voltage Monitor 2 Circuit Control Register 1"]
    pub lvd2cr1: LVD2CR1,
    #[doc = "0xe3 - Voltage Monitor 2 Circuit Status Register"]
    pub lvd2sr: LVD2SR,
    _reserved27: [u8; 0x031a],
    #[doc = "0x3fe - Protect Register"]
    pub prcr: PRCR,
    _reserved28: [u8; 0x0e],
    #[doc = "0x40e - System Control OCD Control Register"]
    pub syocdcr: SYOCDCR,
    _reserved29: [u8; 0x01],
    #[doc = "0x410 - Reset Status Register 0"]
    pub rstsr0: RSTSR0,
    #[doc = "0x411 - Reset Status Register 2"]
    pub rstsr2: RSTSR2,
    _reserved31: [u8; 0x01],
    #[doc = "0x413 - Main Clock Oscillator Mode Oscillation Control Register"]
    pub momcr: MOMCR,
    _reserved32: [u8; 0x03],
    #[doc = "0x417 - Voltage Monitor Circuit Control Register"]
    pub lvcmpcr: LVCMPCR,
    #[doc = "0x418 - Voltage Detection Level Select Register"]
    pub lvdlvlr: LVDLVLR,
    _reserved34: [u8; 0x01],
    #[doc = "0x41a - Voltage Monitor 1 Circuit Control Register 0"]
    pub lvd1cr0: LVD1CR0,
    #[doc = "0x41b - Voltage Monitor 2 Circuit Control Register 0"]
    pub lvd2cr0: LVD2CR0,
    _reserved36: [u8; 0x24],
    #[doc = "0x440 - DCDC/LDO Control Register"]
    pub dcdcctl: DCDCCTL,
    #[doc = "0x441 - Voltage Level Selection Control Register"]
    pub vccsel: VCCSEL,
    _reserved38: [u8; 0x3e],
    #[doc = "0x480 - Sub-Clock Oscillator Control Register"]
    pub sosccr: SOSCCR,
    #[doc = "0x481 - Sub-Clock Oscillator Mode Control Register"]
    pub somcr: SOMCR,
    #[doc = "0x482 - Sub-Clock Oscillator Margin Check Register"]
    pub somrg: SOMRG,
    _reserved41: [u8; 0x0d],
    #[doc = "0x490 - Low-Speed On-Chip Oscillator Control Register"]
    pub lococr: LOCOCR,
    _reserved42: [u8; 0x01],
    #[doc = "0x492 - LOCO User Trimming Control Register"]
    pub locoutcr: LOCOUTCR,
}
#[doc = "SBYCR (rw) register accessor: an alias for `Reg<SBYCR_SPEC>`"]
pub type SBYCR = crate::Reg<sbycr::SBYCR_SPEC>;
#[doc = "Standby Control Register"]
pub mod sbycr;
#[doc = "MSTPCRA (rw) register accessor: an alias for `Reg<MSTPCRA_SPEC>`"]
pub type MSTPCRA = crate::Reg<mstpcra::MSTPCRA_SPEC>;
#[doc = "Module Stop Control Register A"]
pub mod mstpcra;
#[doc = "SCKDIVCR (rw) register accessor: an alias for `Reg<SCKDIVCR_SPEC>`"]
pub type SCKDIVCR = crate::Reg<sckdivcr::SCKDIVCR_SPEC>;
#[doc = "System Clock Division Control Register"]
pub mod sckdivcr;
#[doc = "SCKSCR (rw) register accessor: an alias for `Reg<SCKSCR_SPEC>`"]
pub type SCKSCR = crate::Reg<sckscr::SCKSCR_SPEC>;
#[doc = "System Clock Source Control Register"]
pub mod sckscr;
#[doc = "MEMWAIT (rw) register accessor: an alias for `Reg<MEMWAIT_SPEC>`"]
pub type MEMWAIT = crate::Reg<memwait::MEMWAIT_SPEC>;
#[doc = "Memory Wait Cycle Control Register for Code Flash"]
pub mod memwait;
#[doc = "MOSCCR (rw) register accessor: an alias for `Reg<MOSCCR_SPEC>`"]
pub type MOSCCR = crate::Reg<mosccr::MOSCCR_SPEC>;
#[doc = "Main Clock Oscillator Control Register"]
pub mod mosccr;
#[doc = "HOCOCR (rw) register accessor: an alias for `Reg<HOCOCR_SPEC>`"]
pub type HOCOCR = crate::Reg<hococr::HOCOCR_SPEC>;
#[doc = "High-Speed On-Chip Oscillator Control Register"]
pub mod hococr;
#[doc = "MOCOCR (rw) register accessor: an alias for `Reg<MOCOCR_SPEC>`"]
pub type MOCOCR = crate::Reg<mococr::MOCOCR_SPEC>;
#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
pub mod mococr;
#[doc = "OSCSF (r) register accessor: an alias for `Reg<OSCSF_SPEC>`"]
pub type OSCSF = crate::Reg<oscsf::OSCSF_SPEC>;
#[doc = "Oscillation Stabilization Flag Register"]
pub mod oscsf;
#[doc = "CKOCR (rw) register accessor: an alias for `Reg<CKOCR_SPEC>`"]
pub type CKOCR = crate::Reg<ckocr::CKOCR_SPEC>;
#[doc = "Clock Out Control Register"]
pub mod ckocr;
#[doc = "OSTDCR (rw) register accessor: an alias for `Reg<OSTDCR_SPEC>`"]
pub type OSTDCR = crate::Reg<ostdcr::OSTDCR_SPEC>;
#[doc = "Oscillation Stop Detection Control Register"]
pub mod ostdcr;
#[doc = "OSTDSR (rw) register accessor: an alias for `Reg<OSTDSR_SPEC>`"]
pub type OSTDSR = crate::Reg<ostdsr::OSTDSR_SPEC>;
#[doc = "Oscillation Stop Detection Status Register"]
pub mod ostdsr;
#[doc = "LPOPT (rw) register accessor: an alias for `Reg<LPOPT_SPEC>`"]
pub type LPOPT = crate::Reg<lpopt::LPOPT_SPEC>;
#[doc = "Lower Power Operation Control Register"]
pub mod lpopt;
#[doc = "MOCOUTCR (rw) register accessor: an alias for `Reg<MOCOUTCR_SPEC>`"]
pub type MOCOUTCR = crate::Reg<mocoutcr::MOCOUTCR_SPEC>;
#[doc = "MOCO User Trimming Control Register"]
pub mod mocoutcr;
#[doc = "HOCOUTCR (rw) register accessor: an alias for `Reg<HOCOUTCR_SPEC>`"]
pub type HOCOUTCR = crate::Reg<hocoutcr::HOCOUTCR_SPEC>;
#[doc = "HOCO User Trimming Control Register"]
pub mod hocoutcr;
#[doc = "SNZCR (rw) register accessor: an alias for `Reg<SNZCR_SPEC>`"]
pub type SNZCR = crate::Reg<snzcr::SNZCR_SPEC>;
#[doc = "Snooze Control Register"]
pub mod snzcr;
#[doc = "SNZEDCR0 (rw) register accessor: an alias for `Reg<SNZEDCR0_SPEC>`"]
pub type SNZEDCR0 = crate::Reg<snzedcr0::SNZEDCR0_SPEC>;
#[doc = "Snooze End Control Register 0"]
pub mod snzedcr0;
#[doc = "SNZREQCR0 (rw) register accessor: an alias for `Reg<SNZREQCR0_SPEC>`"]
pub type SNZREQCR0 = crate::Reg<snzreqcr0::SNZREQCR0_SPEC>;
#[doc = "Snooze Request Control Register 0"]
pub mod snzreqcr0;
#[doc = "PSMCR (rw) register accessor: an alias for `Reg<PSMCR_SPEC>`"]
pub type PSMCR = crate::Reg<psmcr::PSMCR_SPEC>;
#[doc = "Power Save Memory Control Register"]
pub mod psmcr;
#[doc = "OPCCR (rw) register accessor: an alias for `Reg<OPCCR_SPEC>`"]
pub type OPCCR = crate::Reg<opccr::OPCCR_SPEC>;
#[doc = "Operating Power Control Register"]
pub mod opccr;
#[doc = "MOSCWTCR (rw) register accessor: an alias for `Reg<MOSCWTCR_SPEC>`"]
pub type MOSCWTCR = crate::Reg<moscwtcr::MOSCWTCR_SPEC>;
#[doc = "Main Clock Oscillator Wait Control Register"]
pub mod moscwtcr;
#[doc = "SOPCCR (rw) register accessor: an alias for `Reg<SOPCCR_SPEC>`"]
pub type SOPCCR = crate::Reg<sopccr::SOPCCR_SPEC>;
#[doc = "Sub Operating Power Control Register"]
pub mod sopccr;
#[doc = "RSTSR1 (rw) register accessor: an alias for `Reg<RSTSR1_SPEC>`"]
pub type RSTSR1 = crate::Reg<rstsr1::RSTSR1_SPEC>;
#[doc = "Reset Status Register 1"]
pub mod rstsr1;
#[doc = "LVD1CR1 (rw) register accessor: an alias for `Reg<LVD1CR1_SPEC>`"]
pub type LVD1CR1 = crate::Reg<lvd1cr1::LVD1CR1_SPEC>;
#[doc = "Voltage Monitor 1 Circuit Control Register"]
pub mod lvd1cr1;
#[doc = "LVD1SR (rw) register accessor: an alias for `Reg<LVD1SR_SPEC>`"]
pub type LVD1SR = crate::Reg<lvd1sr::LVD1SR_SPEC>;
#[doc = "Voltage Monitor 1 Circuit Status Register"]
pub mod lvd1sr;
#[doc = "LVD2CR1 (rw) register accessor: an alias for `Reg<LVD2CR1_SPEC>`"]
pub type LVD2CR1 = crate::Reg<lvd2cr1::LVD2CR1_SPEC>;
#[doc = "Voltage Monitor 2 Circuit Control Register 1"]
pub mod lvd2cr1;
#[doc = "LVD2SR (rw) register accessor: an alias for `Reg<LVD2SR_SPEC>`"]
pub type LVD2SR = crate::Reg<lvd2sr::LVD2SR_SPEC>;
#[doc = "Voltage Monitor 2 Circuit Status Register"]
pub mod lvd2sr;
#[doc = "PRCR (rw) register accessor: an alias for `Reg<PRCR_SPEC>`"]
pub type PRCR = crate::Reg<prcr::PRCR_SPEC>;
#[doc = "Protect Register"]
pub mod prcr;
#[doc = "SYOCDCR (rw) register accessor: an alias for `Reg<SYOCDCR_SPEC>`"]
pub type SYOCDCR = crate::Reg<syocdcr::SYOCDCR_SPEC>;
#[doc = "System Control OCD Control Register"]
pub mod syocdcr;
#[doc = "RSTSR0 (rw) register accessor: an alias for `Reg<RSTSR0_SPEC>`"]
pub type RSTSR0 = crate::Reg<rstsr0::RSTSR0_SPEC>;
#[doc = "Reset Status Register 0"]
pub mod rstsr0;
#[doc = "RSTSR2 (rw) register accessor: an alias for `Reg<RSTSR2_SPEC>`"]
pub type RSTSR2 = crate::Reg<rstsr2::RSTSR2_SPEC>;
#[doc = "Reset Status Register 2"]
pub mod rstsr2;
#[doc = "MOMCR (rw) register accessor: an alias for `Reg<MOMCR_SPEC>`"]
pub type MOMCR = crate::Reg<momcr::MOMCR_SPEC>;
#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub mod momcr;
#[doc = "LVCMPCR (rw) register accessor: an alias for `Reg<LVCMPCR_SPEC>`"]
pub type LVCMPCR = crate::Reg<lvcmpcr::LVCMPCR_SPEC>;
#[doc = "Voltage Monitor Circuit Control Register"]
pub mod lvcmpcr;
#[doc = "LVDLVLR (rw) register accessor: an alias for `Reg<LVDLVLR_SPEC>`"]
pub type LVDLVLR = crate::Reg<lvdlvlr::LVDLVLR_SPEC>;
#[doc = "Voltage Detection Level Select Register"]
pub mod lvdlvlr;
#[doc = "LVD1CR0 (rw) register accessor: an alias for `Reg<LVD1CR0_SPEC>`"]
pub type LVD1CR0 = crate::Reg<lvd1cr0::LVD1CR0_SPEC>;
#[doc = "Voltage Monitor 1 Circuit Control Register 0"]
pub mod lvd1cr0;
#[doc = "LVD2CR0 (rw) register accessor: an alias for `Reg<LVD2CR0_SPEC>`"]
pub type LVD2CR0 = crate::Reg<lvd2cr0::LVD2CR0_SPEC>;
#[doc = "Voltage Monitor 2 Circuit Control Register 0"]
pub mod lvd2cr0;
#[doc = "DCDCCTL (rw) register accessor: an alias for `Reg<DCDCCTL_SPEC>`"]
pub type DCDCCTL = crate::Reg<dcdcctl::DCDCCTL_SPEC>;
#[doc = "DCDC/LDO Control Register"]
pub mod dcdcctl;
#[doc = "VCCSEL (rw) register accessor: an alias for `Reg<VCCSEL_SPEC>`"]
pub type VCCSEL = crate::Reg<vccsel::VCCSEL_SPEC>;
#[doc = "Voltage Level Selection Control Register"]
pub mod vccsel;
#[doc = "SOSCCR (rw) register accessor: an alias for `Reg<SOSCCR_SPEC>`"]
pub type SOSCCR = crate::Reg<sosccr::SOSCCR_SPEC>;
#[doc = "Sub-Clock Oscillator Control Register"]
pub mod sosccr;
#[doc = "SOMCR (rw) register accessor: an alias for `Reg<SOMCR_SPEC>`"]
pub type SOMCR = crate::Reg<somcr::SOMCR_SPEC>;
#[doc = "Sub-Clock Oscillator Mode Control Register"]
pub mod somcr;
#[doc = "SOMRG (rw) register accessor: an alias for `Reg<SOMRG_SPEC>`"]
pub type SOMRG = crate::Reg<somrg::SOMRG_SPEC>;
#[doc = "Sub-Clock Oscillator Margin Check Register"]
pub mod somrg;
#[doc = "LOCOCR (rw) register accessor: an alias for `Reg<LOCOCR_SPEC>`"]
pub type LOCOCR = crate::Reg<lococr::LOCOCR_SPEC>;
#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub mod lococr;
#[doc = "LOCOUTCR (rw) register accessor: an alias for `Reg<LOCOUTCR_SPEC>`"]
pub type LOCOUTCR = crate::Reg<locoutcr::LOCOUTCR_SPEC>;
#[doc = "LOCO User Trimming Control Register"]
pub mod locoutcr;
