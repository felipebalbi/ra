#[doc = "Register `LVD2CR0` reader"]
pub struct R(crate::R<LVD2CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVD2CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVD2CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVD2CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVD2CR0` writer"]
pub struct W(crate::W<LVD2CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVD2CR0_SPEC>;
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
impl From<crate::W<LVD2CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVD2CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIE` reader - Voltage Monitor 2 Interrupt/Reset Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Voltage Monitor 2 Interrupt/Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Field `RIE` writer - Voltage Monitor 2 Interrupt/Reset Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD2CR0_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `DFDIS` reader - Voltage monitor 2 Digital Filter Disabled Mode Select"]
pub type DFDIS_R = crate::BitReader<DFDIS_A>;
#[doc = "Voltage monitor 2 Digital Filter Disabled Mode Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFDIS_A {
    #[doc = "0: Enable the digital filter"]
    _0 = 0,
    #[doc = "1: Disable the digital filter"]
    _1 = 1,
}
impl From<DFDIS_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl DFDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIS_A {
        match self.bits {
            false => DFDIS_A::_0,
            true => DFDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFDIS_A::_1
    }
}
#[doc = "Field `DFDIS` writer - Voltage monitor 2 Digital Filter Disabled Mode Select"]
pub type DFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD2CR0_SPEC, DFDIS_A, O>;
impl<'a, const O: u8> DFDIS_W<'a, O> {
    #[doc = "Enable the digital filter"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIS_A::_0)
    }
    #[doc = "Disable the digital filter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIS_A::_1)
    }
}
#[doc = "Field `CMPE` reader - Voltage Monitor 2 Circuit Comparison Result Output Enable"]
pub type CMPE_R = crate::BitReader<CMPE_A>;
#[doc = "Voltage Monitor 2 Circuit Comparison Result Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPE_A {
    #[doc = "0: Disable voltage monitor 2 circuit comparison result output"]
    _0 = 0,
    #[doc = "1: Enable voltage monitor 2 circuit comparison result output"]
    _1 = 1,
}
impl From<CMPE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPE_A {
        match self.bits {
            false => CMPE_A::_0,
            true => CMPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPE_A::_1
    }
}
#[doc = "Field `CMPE` writer - Voltage Monitor 2 Circuit Comparison Result Output Enable"]
pub type CMPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD2CR0_SPEC, CMPE_A, O>;
impl<'a, const O: u8> CMPE_W<'a, O> {
    #[doc = "Disable voltage monitor 2 circuit comparison result output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPE_A::_0)
    }
    #[doc = "Enable voltage monitor 2 circuit comparison result output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPE_A::_1)
    }
}
#[doc = "Field `FSAMP` reader - Sampling Clock Select"]
pub type FSAMP_R = crate::FieldReader<u8, FSAMP_A>;
#[doc = "Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSAMP_A {
    #[doc = "0: 1/2 LOCO frequency"]
    _00 = 0,
    #[doc = "1: 1/4 LOCO frequency"]
    _01 = 1,
    #[doc = "2: 1/8 LOCO frequency"]
    _10 = 2,
    #[doc = "3: 1/16 LOCO frequency"]
    _11 = 3,
}
impl From<FSAMP_A> for u8 {
    #[inline(always)]
    fn from(variant: FSAMP_A) -> Self {
        variant as _
    }
}
impl FSAMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSAMP_A {
        match self.bits {
            0 => FSAMP_A::_00,
            1 => FSAMP_A::_01,
            2 => FSAMP_A::_10,
            3 => FSAMP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FSAMP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FSAMP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FSAMP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FSAMP_A::_11
    }
}
#[doc = "Field `FSAMP` writer - Sampling Clock Select"]
pub type FSAMP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LVD2CR0_SPEC, u8, FSAMP_A, 2, O>;
impl<'a, const O: u8> FSAMP_W<'a, O> {
    #[doc = "1/2 LOCO frequency"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FSAMP_A::_00)
    }
    #[doc = "1/4 LOCO frequency"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FSAMP_A::_01)
    }
    #[doc = "1/8 LOCO frequency"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FSAMP_A::_10)
    }
    #[doc = "1/16 LOCO frequency"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FSAMP_A::_11)
    }
}
#[doc = "Field `RI` reader - Voltage Monitor 2 Circuit Mode Select"]
pub type RI_R = crate::BitReader<RI_A>;
#[doc = "Voltage Monitor 2 Circuit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RI_A {
    #[doc = "0: Generate voltage monitor 2 interrupt on Vdet2 crossing"]
    _0 = 0,
    #[doc = "1: Enable voltage monitor 2 reset when the voltage falls to and below Vdet2"]
    _1 = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
impl RI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::_0,
            true => RI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RI_A::_1
    }
}
#[doc = "Field `RI` writer - Voltage Monitor 2 Circuit Mode Select"]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD2CR0_SPEC, RI_A, O>;
impl<'a, const O: u8> RI_W<'a, O> {
    #[doc = "Generate voltage monitor 2 interrupt on Vdet2 crossing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RI_A::_0)
    }
    #[doc = "Enable voltage monitor 2 reset when the voltage falls to and below Vdet2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RI_A::_1)
    }
}
#[doc = "Field `RN` reader - Voltage Monitor 2 Reset Negate Select"]
pub type RN_R = crate::BitReader<RN_A>;
#[doc = "Voltage Monitor 2 Reset Negate Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RN_A {
    #[doc = "0: Negate after a stabilization time (tLVD2) when VCC > Vdet2 is detected"]
    _0 = 0,
    #[doc = "1: Negate after a stabilization time (tLVD2) on assertion of the LVD2 reset"]
    _1 = 1,
}
impl From<RN_A> for bool {
    #[inline(always)]
    fn from(variant: RN_A) -> Self {
        variant as u8 != 0
    }
}
impl RN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RN_A {
        match self.bits {
            false => RN_A::_0,
            true => RN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RN_A::_1
    }
}
#[doc = "Field `RN` writer - Voltage Monitor 2 Reset Negate Select"]
pub type RN_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD2CR0_SPEC, RN_A, O>;
impl<'a, const O: u8> RN_W<'a, O> {
    #[doc = "Negate after a stabilization time (tLVD2) when VCC > Vdet2 is detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RN_A::_0)
    }
    #[doc = "Negate after a stabilization time (tLVD2) on assertion of the LVD2 reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor 2 Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage monitor 2 Digital Filter Disabled Mode Select"]
    #[inline(always)]
    pub fn dfdis(&self) -> DFDIS_R {
        DFDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 2 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(&self) -> CMPE_R {
        CMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sampling Clock Select"]
    #[inline(always)]
    pub fn fsamp(&self) -> FSAMP_R {
        FSAMP_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Voltage Monitor 2 Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Voltage Monitor 2 Reset Negate Select"]
    #[inline(always)]
    pub fn rn(&self) -> RN_R {
        RN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor 2 Interrupt/Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<0> {
        RIE_W::new(self)
    }
    #[doc = "Bit 1 - Voltage monitor 2 Digital Filter Disabled Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dfdis(&mut self) -> DFDIS_W<1> {
        DFDIS_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Monitor 2 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpe(&mut self) -> CMPE_W<2> {
        CMPE_W::new(self)
    }
    #[doc = "Bits 4:5 - Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fsamp(&mut self) -> FSAMP_W<4> {
        FSAMP_W::new(self)
    }
    #[doc = "Bit 6 - Voltage Monitor 2 Circuit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<6> {
        RI_W::new(self)
    }
    #[doc = "Bit 7 - Voltage Monitor 2 Reset Negate Select"]
    #[inline(always)]
    #[must_use]
    pub fn rn(&mut self) -> RN_W<7> {
        RN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Monitor 2 Circuit Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvd2cr0](index.html) module"]
pub struct LVD2CR0_SPEC;
impl crate::RegisterSpec for LVD2CR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvd2cr0::R](R) reader structure"]
impl crate::Readable for LVD2CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvd2cr0::W](W) writer structure"]
impl crate::Writable for LVD2CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVD2CR0 to value 0x82"]
impl crate::Resettable for LVD2CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x82;
}
