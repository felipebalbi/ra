#[doc = "Register `PC1%sPFS` reader"]
pub struct R(crate::R<PC1PFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC1PFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC1PFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC1PFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC1%sPFS` writer"]
pub struct W(crate::W<PC1PFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC1PFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PC1PFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC1PFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PODR` reader - Port Output Data"]
pub type PODR_R = crate::BitReader<PODR_A>;
#[doc = "Port Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PODR_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR_A> for bool {
    #[inline(always)]
    fn from(variant: PODR_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR_A {
        match self.bits {
            false => PODR_A::_0,
            true => PODR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR_A::_1
    }
}
#[doc = "Field `PODR` writer - Port Output Data"]
pub type PODR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PC1PFS_SPEC, PODR_A, O>;
impl<'a, const O: u8> PODR_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR_A::_1)
    }
}
#[doc = "Field `PIDR` reader - Pmn State"]
pub type PIDR_R = crate::BitReader<PIDR_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR_A {
        match self.bits {
            false => PIDR_A::_0,
            true => PIDR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR_A::_1
    }
}
#[doc = "Field `PDR` reader - Port Direction"]
pub type PDR_R = crate::BitReader<PDR_A>;
#[doc = "Port Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR_A> for bool {
    #[inline(always)]
    fn from(variant: PDR_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR_A {
        match self.bits {
            false => PDR_A::_0,
            true => PDR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR_A::_1
    }
}
#[doc = "Field `PDR` writer - Port Direction"]
pub type PDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PC1PFS_SPEC, PDR_A, O>;
impl<'a, const O: u8> PDR_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR_A::_1)
    }
}
#[doc = "Field `PCR` reader - Pull-up Control"]
pub type PCR_R = crate::BitReader<PCR_A>;
#[doc = "Pull-up Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCR_A {
    #[doc = "0: Disable input pull-up"]
    _0 = 0,
    #[doc = "1: Enable input pull-up"]
    _1 = 1,
}
impl From<PCR_A> for bool {
    #[inline(always)]
    fn from(variant: PCR_A) -> Self {
        variant as u8 != 0
    }
}
impl PCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCR_A {
        match self.bits {
            false => PCR_A::_0,
            true => PCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCR_A::_1
    }
}
#[doc = "Field `PCR` writer - Pull-up Control"]
pub type PCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PC1PFS_SPEC, PCR_A, O>;
impl<'a, const O: u8> PCR_W<'a, O> {
    #[doc = "Disable input pull-up"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCR_A::_0)
    }
    #[doc = "Enable input pull-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCR_A::_1)
    }
}
#[doc = "Field `NCODR` reader - N-Channel Open-Drain Control"]
pub type NCODR_R = crate::BitReader<NCODR_A>;
#[doc = "N-Channel Open-Drain Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCODR_A {
    #[doc = "0: CMOS output"]
    _0 = 0,
    #[doc = "1: NMOS open-drain output"]
    _1 = 1,
}
impl From<NCODR_A> for bool {
    #[inline(always)]
    fn from(variant: NCODR_A) -> Self {
        variant as u8 != 0
    }
}
impl NCODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCODR_A {
        match self.bits {
            false => NCODR_A::_0,
            true => NCODR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCODR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCODR_A::_1
    }
}
#[doc = "Field `NCODR` writer - N-Channel Open-Drain Control"]
pub type NCODR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PC1PFS_SPEC, NCODR_A, O>;
impl<'a, const O: u8> NCODR_W<'a, O> {
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCODR_A::_0)
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCODR_A::_1)
    }
}
#[doc = "Field `DSCR` reader - Port Drive Capability"]
pub type DSCR_R = crate::FieldReader<u8, DSCR_A>;
#[doc = "Port Drive Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSCR_A {
    #[doc = "0: Low drive"]
    _00 = 0,
    #[doc = "1: Middle drive"]
    _01 = 1,
    #[doc = "2: High-speed high-drive / High current drive"]
    _10 = 2,
    #[doc = "3: High drive"]
    _11 = 3,
}
impl From<DSCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DSCR_A) -> Self {
        variant as _
    }
}
impl DSCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCR_A {
        match self.bits {
            0 => DSCR_A::_00,
            1 => DSCR_A::_01,
            2 => DSCR_A::_10,
            3 => DSCR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DSCR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DSCR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DSCR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DSCR_A::_11
    }
}
#[doc = "Field `DSCR` writer - Port Drive Capability"]
pub type DSCR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PC1PFS_SPEC, u8, DSCR_A, 2, O>;
impl<'a, const O: u8> DSCR_W<'a, O> {
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DSCR_A::_00)
    }
    #[doc = "Middle drive"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DSCR_A::_01)
    }
    #[doc = "High-speed high-drive / High current drive"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DSCR_A::_10)
    }
    #[doc = "High drive"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DSCR_A::_11)
    }
}
#[doc = "Field `EOFR` reader - Event on Falling/Event on Rising"]
pub type EOFR_R = crate::FieldReader<u8, EOFR_A>;
#[doc = "Event on Falling/Event on Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EOFR_A {
    #[doc = "0: Don't care"]
    _00 = 0,
    #[doc = "1: Detect rising edge"]
    _01 = 1,
    #[doc = "2: Detect falling edge"]
    _10 = 2,
    #[doc = "3: Detect both edges"]
    _11 = 3,
}
impl From<EOFR_A> for u8 {
    #[inline(always)]
    fn from(variant: EOFR_A) -> Self {
        variant as _
    }
}
impl EOFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOFR_A {
        match self.bits {
            0 => EOFR_A::_00,
            1 => EOFR_A::_01,
            2 => EOFR_A::_10,
            3 => EOFR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EOFR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EOFR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EOFR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EOFR_A::_11
    }
}
#[doc = "Field `EOFR` writer - Event on Falling/Event on Rising"]
pub type EOFR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PC1PFS_SPEC, u8, EOFR_A, 2, O>;
impl<'a, const O: u8> EOFR_W<'a, O> {
    #[doc = "Don't care"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EOFR_A::_00)
    }
    #[doc = "Detect rising edge"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EOFR_A::_01)
    }
    #[doc = "Detect falling edge"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EOFR_A::_10)
    }
    #[doc = "Detect both edges"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EOFR_A::_11)
    }
}
#[doc = "Field `ISEL` reader - IRQ Input Enable"]
pub type ISEL_R = crate::BitReader<ISEL_A>;
#[doc = "IRQ Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    #[doc = "0: Not used as an IRQn input pin"]
    _0 = 0,
    #[doc = "1: Used as an IRQn input pin"]
    _1 = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::_0,
            true => ISEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISEL_A::_1
    }
}
#[doc = "Field `ISEL` writer - IRQ Input Enable"]
pub type ISEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PC1PFS_SPEC, ISEL_A, O>;
impl<'a, const O: u8> ISEL_W<'a, O> {
    #[doc = "Not used as an IRQn input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISEL_A::_0)
    }
    #[doc = "Used as an IRQn input pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISEL_A::_1)
    }
}
#[doc = "Field `ASEL` reader - Analog Input Enable"]
pub type ASEL_R = crate::BitReader<ASEL_A>;
#[doc = "Analog Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEL_A {
    #[doc = "0: Not used as an analog pin"]
    _0 = 0,
    #[doc = "1: Used as an analog pin"]
    _1 = 1,
}
impl From<ASEL_A> for bool {
    #[inline(always)]
    fn from(variant: ASEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASEL_A {
        match self.bits {
            false => ASEL_A::_0,
            true => ASEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASEL_A::_1
    }
}
#[doc = "Field `ASEL` writer - Analog Input Enable"]
pub type ASEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PC1PFS_SPEC, ASEL_A, O>;
impl<'a, const O: u8> ASEL_W<'a, O> {
    #[doc = "Not used as an analog pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASEL_A::_0)
    }
    #[doc = "Used as an analog pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASEL_A::_1)
    }
}
#[doc = "Field `PMR` reader - Port Mode Control"]
pub type PMR_R = crate::BitReader<PMR_A>;
#[doc = "Port Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMR_A {
    #[doc = "0: Used as a general I/O pin"]
    _0 = 0,
    #[doc = "1: Used as an I/O port for peripheral functions"]
    _1 = 1,
}
impl From<PMR_A> for bool {
    #[inline(always)]
    fn from(variant: PMR_A) -> Self {
        variant as u8 != 0
    }
}
impl PMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMR_A {
        match self.bits {
            false => PMR_A::_0,
            true => PMR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PMR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PMR_A::_1
    }
}
#[doc = "Field `PMR` writer - Port Mode Control"]
pub type PMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PC1PFS_SPEC, PMR_A, O>;
impl<'a, const O: u8> PMR_W<'a, O> {
    #[doc = "Used as a general I/O pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PMR_A::_0)
    }
    #[doc = "Used as an I/O port for peripheral functions"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PMR_A::_1)
    }
}
#[doc = "Field `PSEL` reader - Peripheral Select"]
pub type PSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEL` writer - Peripheral Select"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PC1PFS_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PODR_R {
        PODR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pmn State"]
    #[inline(always)]
    pub fn pidr(&self) -> PIDR_R {
        PIDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(&self) -> NCODR_R {
        NCODR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(&self) -> EOFR_R {
        EOFR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Input Enable"]
    #[inline(always)]
    pub fn asel(&self) -> ASEL_R {
        ASEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Mode Control"]
    #[inline(always)]
    pub fn pmr(&self) -> PMR_R {
        PMR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Peripheral Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn podr(&mut self) -> PODR_W<0> {
        PODR_W::new(self)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr(&mut self) -> PDR_W<2> {
        PDR_W::new(self)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PCR_W<4> {
        PCR_W::new(self)
    }
    #[doc = "Bit 6 - N-Channel Open-Drain Control"]
    #[inline(always)]
    #[must_use]
    pub fn ncodr(&mut self) -> NCODR_W<6> {
        NCODR_W::new(self)
    }
    #[doc = "Bits 10:11 - Port Drive Capability"]
    #[inline(always)]
    #[must_use]
    pub fn dscr(&mut self) -> DSCR_W<10> {
        DSCR_W::new(self)
    }
    #[doc = "Bits 12:13 - Event on Falling/Event on Rising"]
    #[inline(always)]
    #[must_use]
    pub fn eofr(&mut self) -> EOFR_W<12> {
        EOFR_W::new(self)
    }
    #[doc = "Bit 14 - IRQ Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn isel(&mut self) -> ISEL_W<14> {
        ISEL_W::new(self)
    }
    #[doc = "Bit 15 - Analog Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asel(&mut self) -> ASEL_W<15> {
        ASEL_W::new(self)
    }
    #[doc = "Bit 16 - Port Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn pmr(&mut self) -> PMR_W<16> {
        PMR_W::new(self)
    }
    #[doc = "Bits 24:28 - Peripheral Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<24> {
        PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C1%s Pin Function Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc1pfs](index.html) module"]
pub struct PC1PFS_SPEC;
impl crate::RegisterSpec for PC1PFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc1pfs::R](R) reader structure"]
impl crate::Readable for PC1PFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc1pfs::W](W) writer structure"]
impl crate::Writable for PC1PFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC1%sPFS to value 0"]
impl crate::Resettable for PC1PFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
