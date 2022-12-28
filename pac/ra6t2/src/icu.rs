#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - IRQ Control Register %s"]
    pub irqcr: [IRQCR; 16],
    _reserved1: [u8; 0xf0],
    #[doc = "0x100 - NMI Pin Interrupt Control Register"]
    pub nmicr: NMICR,
    _reserved2: [u8; 0x1f],
    #[doc = "0x120 - Non-Maskable Interrupt Enable Register"]
    pub nmier: NMIER,
    _reserved3: [u8; 0x0e],
    #[doc = "0x130 - Non-Maskable Interrupt Status Clear Register"]
    pub nmiclr: NMICLR,
    _reserved4: [u8; 0x0e],
    #[doc = "0x140 - Non-Maskable Interrupt Status Register"]
    pub nmisr: NMISR,
    _reserved5: [u8; 0x5e],
    #[doc = "0x1a0 - Wake Up Interrupt Enable Register 0"]
    pub wupen0: WUPEN0,
    _reserved6: [u8; 0x5c],
    #[doc = "0x200 - SYS Event Link Setting Register"]
    pub selsr0: SELSR0,
    _reserved7: [u8; 0x7e],
    #[doc = "0x280..0x2a0 - DMAC Event Link Setting Register %s"]
    pub delsr: [DELSR; 8],
    _reserved8: [u8; 0x60],
    #[doc = "0x300..0x480 - ICU Event Link Setting Register %s"]
    pub ielsr: [IELSR; 96],
}
#[doc = "IRQCR (rw) register accessor: an alias for `Reg<IRQCR_SPEC>`"]
pub type IRQCR = crate::Reg<irqcr::IRQCR_SPEC>;
#[doc = "IRQ Control Register %s"]
pub mod irqcr;
#[doc = "NMICR (rw) register accessor: an alias for `Reg<NMICR_SPEC>`"]
pub type NMICR = crate::Reg<nmicr::NMICR_SPEC>;
#[doc = "NMI Pin Interrupt Control Register"]
pub mod nmicr;
#[doc = "NMIER (rw) register accessor: an alias for `Reg<NMIER_SPEC>`"]
pub type NMIER = crate::Reg<nmier::NMIER_SPEC>;
#[doc = "Non-Maskable Interrupt Enable Register"]
pub mod nmier;
#[doc = "NMICLR (rw) register accessor: an alias for `Reg<NMICLR_SPEC>`"]
pub type NMICLR = crate::Reg<nmiclr::NMICLR_SPEC>;
#[doc = "Non-Maskable Interrupt Status Clear Register"]
pub mod nmiclr;
#[doc = "NMISR (r) register accessor: an alias for `Reg<NMISR_SPEC>`"]
pub type NMISR = crate::Reg<nmisr::NMISR_SPEC>;
#[doc = "Non-Maskable Interrupt Status Register"]
pub mod nmisr;
#[doc = "WUPEN0 (rw) register accessor: an alias for `Reg<WUPEN0_SPEC>`"]
pub type WUPEN0 = crate::Reg<wupen0::WUPEN0_SPEC>;
#[doc = "Wake Up Interrupt Enable Register 0"]
pub mod wupen0;
#[doc = "SELSR0 (rw) register accessor: an alias for `Reg<SELSR0_SPEC>`"]
pub type SELSR0 = crate::Reg<selsr0::SELSR0_SPEC>;
#[doc = "SYS Event Link Setting Register"]
pub mod selsr0;
#[doc = "DELSR (rw) register accessor: an alias for `Reg<DELSR_SPEC>`"]
pub type DELSR = crate::Reg<delsr::DELSR_SPEC>;
#[doc = "DMAC Event Link Setting Register %s"]
pub mod delsr;
#[doc = "IELSR (rw) register accessor: an alias for `Reg<IELSR_SPEC>`"]
pub type IELSR = crate::Reg<ielsr::IELSR_SPEC>;
#[doc = "ICU Event Link Setting Register %s"]
pub mod ielsr;
