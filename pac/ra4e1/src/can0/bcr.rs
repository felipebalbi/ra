#[doc = "Register `BCR` reader"]
pub struct R(crate::R<BCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCR` writer"]
pub struct W(crate::W<BCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR_SPEC>;
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
impl From<crate::W<BCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLKS` reader - CAN Clock Source Selection"]
pub type CCLKS_R = crate::BitReader<CCLKS_A>;
#[doc = "CAN Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLKS_A {
    #[doc = "0: PCLKB (generated by the PLL clock)"]
    _0 = 0,
    #[doc = "1: CANMCLK (generated by the main clock oscillator)"]
    _1 = 1,
}
impl From<CCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: CCLKS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLKS_A {
        match self.bits {
            false => CCLKS_A::_0,
            true => CCLKS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCLKS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCLKS_A::_1
    }
}
#[doc = "Field `CCLKS` writer - CAN Clock Source Selection"]
pub type CCLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, CCLKS_A, O>;
impl<'a, const O: u8> CCLKS_W<'a, O> {
    #[doc = "PCLKB (generated by the PLL clock)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLKS_A::_0)
    }
    #[doc = "CANMCLK (generated by the main clock oscillator)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLKS_A::_1)
    }
}
#[doc = "Field `TSEG2` reader - Time Segment 2 Control"]
pub type TSEG2_R = crate::FieldReader<u8, TSEG2_A>;
#[doc = "Time Segment 2 Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEG2_A {
    #[doc = "0: Setting prohibited"]
    _000 = 0,
    #[doc = "1: 2 Tq"]
    _001 = 1,
    #[doc = "2: 3 Tq"]
    _010 = 2,
    #[doc = "3: 4 Tq"]
    _011 = 3,
    #[doc = "4: 5 Tq"]
    _100 = 4,
    #[doc = "5: 6 Tq"]
    _101 = 5,
    #[doc = "6: 7 Tq"]
    _110 = 6,
    #[doc = "7: 8 Tq"]
    _111 = 7,
}
impl From<TSEG2_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEG2_A) -> Self {
        variant as _
    }
}
impl TSEG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEG2_A {
        match self.bits {
            0 => TSEG2_A::_000,
            1 => TSEG2_A::_001,
            2 => TSEG2_A::_010,
            3 => TSEG2_A::_011,
            4 => TSEG2_A::_100,
            5 => TSEG2_A::_101,
            6 => TSEG2_A::_110,
            7 => TSEG2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TSEG2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TSEG2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TSEG2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TSEG2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TSEG2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TSEG2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TSEG2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == TSEG2_A::_111
    }
}
#[doc = "Field `TSEG2` writer - Time Segment 2 Control"]
pub type TSEG2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BCR_SPEC, u8, TSEG2_A, 3, O>;
impl<'a, const O: u8> TSEG2_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(TSEG2_A::_000)
    }
    #[doc = "2 Tq"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(TSEG2_A::_001)
    }
    #[doc = "3 Tq"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(TSEG2_A::_010)
    }
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(TSEG2_A::_011)
    }
    #[doc = "5 Tq"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TSEG2_A::_100)
    }
    #[doc = "6 Tq"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(TSEG2_A::_101)
    }
    #[doc = "7 Tq"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(TSEG2_A::_110)
    }
    #[doc = "8 Tq"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(TSEG2_A::_111)
    }
}
#[doc = "Field `SJW` reader - Synchronization Jump Width Control"]
pub type SJW_R = crate::FieldReader<u8, SJW_A>;
#[doc = "Synchronization Jump Width Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SJW_A {
    #[doc = "0: 1 Tq"]
    _00 = 0,
    #[doc = "1: 2 Tq"]
    _01 = 1,
    #[doc = "2: 3 Tq"]
    _10 = 2,
    #[doc = "3: 4 Tq"]
    _11 = 3,
}
impl From<SJW_A> for u8 {
    #[inline(always)]
    fn from(variant: SJW_A) -> Self {
        variant as _
    }
}
impl SJW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SJW_A {
        match self.bits {
            0 => SJW_A::_00,
            1 => SJW_A::_01,
            2 => SJW_A::_10,
            3 => SJW_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SJW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SJW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SJW_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SJW_A::_11
    }
}
#[doc = "Field `SJW` writer - Synchronization Jump Width Control"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BCR_SPEC, u8, SJW_A, 2, O>;
impl<'a, const O: u8> SJW_W<'a, O> {
    #[doc = "1 Tq"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SJW_A::_00)
    }
    #[doc = "2 Tq"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SJW_A::_01)
    }
    #[doc = "3 Tq"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SJW_A::_10)
    }
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SJW_A::_11)
    }
}
#[doc = "Field `BRP` reader - Baud Rate Prescaler Select"]
pub type BRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRP` writer - Baud Rate Prescaler Select"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR_SPEC, u16, u16, 10, O>;
#[doc = "Field `TSEG1` reader - Time Segment 1 Control"]
pub type TSEG1_R = crate::FieldReader<u8, TSEG1_A>;
#[doc = "Time Segment 1 Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEG1_A {
    #[doc = "3: 4 Tq"]
    _0X3 = 3,
    #[doc = "4: 5 Tq"]
    _0X4 = 4,
    #[doc = "5: 6 Tq"]
    _0X5 = 5,
    #[doc = "6: 7 Tq"]
    _0X6 = 6,
    #[doc = "7: 8 Tq"]
    _0X7 = 7,
    #[doc = "8: 9 Tq"]
    _0X8 = 8,
    #[doc = "9: 10 Tq"]
    _0X9 = 9,
    #[doc = "10: 11 Tq"]
    _0X_A = 10,
    #[doc = "11: 12 Tq"]
    _0X_B = 11,
    #[doc = "12: 13 Tq"]
    _0X_C = 12,
    #[doc = "13: 14 Tq"]
    _0X_D = 13,
    #[doc = "14: 15 Tq"]
    _0X_E = 14,
    #[doc = "15: 16 Tq"]
    _0X_F = 15,
}
impl From<TSEG1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEG1_A) -> Self {
        variant as _
    }
}
impl TSEG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEG1_A> {
        match self.bits {
            3 => Some(TSEG1_A::_0X3),
            4 => Some(TSEG1_A::_0X4),
            5 => Some(TSEG1_A::_0X5),
            6 => Some(TSEG1_A::_0X6),
            7 => Some(TSEG1_A::_0X7),
            8 => Some(TSEG1_A::_0X8),
            9 => Some(TSEG1_A::_0X9),
            10 => Some(TSEG1_A::_0X_A),
            11 => Some(TSEG1_A::_0X_B),
            12 => Some(TSEG1_A::_0X_C),
            13 => Some(TSEG1_A::_0X_D),
            14 => Some(TSEG1_A::_0X_E),
            15 => Some(TSEG1_A::_0X_F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == TSEG1_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == TSEG1_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == TSEG1_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == TSEG1_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == TSEG1_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == TSEG1_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == TSEG1_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == TSEG1_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == TSEG1_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == TSEG1_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == TSEG1_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == TSEG1_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == TSEG1_A::_0X_F
    }
}
#[doc = "Field `TSEG1` writer - Time Segment 1 Control"]
pub type TSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR_SPEC, u8, TSEG1_A, 4, O>;
impl<'a, const O: u8> TSEG1_W<'a, O> {
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X3)
    }
    #[doc = "5 Tq"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X4)
    }
    #[doc = "6 Tq"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X5)
    }
    #[doc = "7 Tq"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X6)
    }
    #[doc = "8 Tq"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X7)
    }
    #[doc = "9 Tq"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X8)
    }
    #[doc = "10 Tq"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X9)
    }
    #[doc = "11 Tq"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X_A)
    }
    #[doc = "12 Tq"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X_B)
    }
    #[doc = "13 Tq"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X_C)
    }
    #[doc = "14 Tq"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X_D)
    }
    #[doc = "15 Tq"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X_E)
    }
    #[doc = "16 Tq"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(TSEG1_A::_0X_F)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Clock Source Selection"]
    #[inline(always)]
    pub fn cclks(&self) -> CCLKS_R {
        CCLKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Time Segment 2 Control"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Synchronization Jump Width Control"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Baud Rate Prescaler Select"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Time Segment 1 Control"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cclks(&mut self) -> CCLKS_W<0> {
        CCLKS_W::new(self)
    }
    #[doc = "Bits 8:10 - Time Segment 2 Control"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<8> {
        TSEG2_W::new(self)
    }
    #[doc = "Bits 12:13 - Synchronization Jump Width Control"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<12> {
        SJW_W::new(self)
    }
    #[doc = "Bits 16:25 - Baud Rate Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<16> {
        BRP_W::new(self)
    }
    #[doc = "Bits 28:31 - Time Segment 1 Control"]
    #[inline(always)]
    #[must_use]
    pub fn tseg1(&mut self) -> TSEG1_W<28> {
        TSEG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](index.html) module"]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcr::R](R) reader structure"]
impl crate::Readable for BCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcr::W](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
