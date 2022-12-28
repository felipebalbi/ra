#[doc = "Register `RSTSR0` reader"]
pub struct R(crate::R<RSTSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTSR0` writer"]
pub struct W(crate::W<RSTSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTSR0_SPEC>;
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
impl From<crate::W<RSTSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORF` reader - Power-On Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nThe field is **modified** in some way after a read operation."]
pub type PORF_R = crate::BitReader<PORF_A>;
#[doc = "Power-On Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORF_A {
    #[doc = "0: Power-on reset not detected."]
    _0 = 0,
    #[doc = "1: Power-on reset detected."]
    _1 = 1,
}
impl From<PORF_A> for bool {
    #[inline(always)]
    fn from(variant: PORF_A) -> Self {
        variant as u8 != 0
    }
}
impl PORF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORF_A {
        match self.bits {
            false => PORF_A::_0,
            true => PORF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORF_A::_1
    }
}
#[doc = "Field `PORF` writer - Power-On Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
pub type PORF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, RSTSR0_SPEC, PORF_A, O>;
impl<'a, const O: u8> PORF_W<'a, O> {
    #[doc = "Power-on reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORF_A::_0)
    }
    #[doc = "Power-on reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORF_A::_1)
    }
}
#[doc = "Field `LVD0RF` reader - Voltage Monitor 0 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nThe field is **modified** in some way after a read operation."]
pub type LVD0RF_R = crate::BitReader<LVD0RF_A>;
#[doc = "Voltage Monitor 0 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD0RF_A {
    #[doc = "0: Voltage Monitor 0 reset not detected."]
    _0 = 0,
    #[doc = "1: Voltage Monitor 0 reset detected."]
    _1 = 1,
}
impl From<LVD0RF_A> for bool {
    #[inline(always)]
    fn from(variant: LVD0RF_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD0RF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD0RF_A {
        match self.bits {
            false => LVD0RF_A::_0,
            true => LVD0RF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD0RF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD0RF_A::_1
    }
}
#[doc = "Field `LVD0RF` writer - Voltage Monitor 0 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
pub type LVD0RF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, RSTSR0_SPEC, LVD0RF_A, O>;
impl<'a, const O: u8> LVD0RF_W<'a, O> {
    #[doc = "Voltage Monitor 0 reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD0RF_A::_0)
    }
    #[doc = "Voltage Monitor 0 reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD0RF_A::_1)
    }
}
#[doc = "Field `LVD1RF` reader - Voltage Monitor 1 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nThe field is **modified** in some way after a read operation."]
pub type LVD1RF_R = crate::BitReader<LVD1RF_A>;
#[doc = "Voltage Monitor 1 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1RF_A {
    #[doc = "0: Voltage Monitor 1 reset not detected."]
    _0 = 0,
    #[doc = "1: Voltage Monitor 1 reset detected."]
    _1 = 1,
}
impl From<LVD1RF_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1RF_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1RF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD1RF_A {
        match self.bits {
            false => LVD1RF_A::_0,
            true => LVD1RF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1RF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1RF_A::_1
    }
}
#[doc = "Field `LVD1RF` writer - Voltage Monitor 1 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
pub type LVD1RF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, RSTSR0_SPEC, LVD1RF_A, O>;
impl<'a, const O: u8> LVD1RF_W<'a, O> {
    #[doc = "Voltage Monitor 1 reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD1RF_A::_0)
    }
    #[doc = "Voltage Monitor 1 reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD1RF_A::_1)
    }
}
#[doc = "Field `LVD2RF` reader - Voltage Monitor 2 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nThe field is **modified** in some way after a read operation."]
pub type LVD2RF_R = crate::BitReader<LVD2RF_A>;
#[doc = "Voltage Monitor 2 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2RF_A {
    #[doc = "0: Voltage Monitor 2 reset not detected."]
    _0 = 0,
    #[doc = "1: Voltage Monitor 2 reset detected."]
    _1 = 1,
}
impl From<LVD2RF_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2RF_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2RF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD2RF_A {
        match self.bits {
            false => LVD2RF_A::_0,
            true => LVD2RF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2RF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2RF_A::_1
    }
}
#[doc = "Field `LVD2RF` writer - Voltage Monitor 2 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
pub type LVD2RF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, RSTSR0_SPEC, LVD2RF_A, O>;
impl<'a, const O: u8> LVD2RF_W<'a, O> {
    #[doc = "Voltage Monitor 2 reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD2RF_A::_0)
    }
    #[doc = "Voltage Monitor 2 reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD2RF_A::_1)
    }
}
#[doc = "Field `DPSRSTF` reader - Deep Software Standby Reset FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nThe field is **modified** in some way after a read operation."]
pub type DPSRSTF_R = crate::BitReader<DPSRSTF_A>;
#[doc = "Deep Software Standby Reset FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSRSTF_A {
    #[doc = "0: Deep software standby mode cancelation not requested by an interrupt."]
    _0 = 0,
    #[doc = "1: Deep software standby mode cancelation requested by an interrupt."]
    _1 = 1,
}
impl From<DPSRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: DPSRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl DPSRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSRSTF_A {
        match self.bits {
            false => DPSRSTF_A::_0,
            true => DPSRSTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPSRSTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPSRSTF_A::_1
    }
}
#[doc = "Field `DPSRSTF` writer - Deep Software Standby Reset FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
pub type DPSRSTF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, RSTSR0_SPEC, DPSRSTF_A, O>;
impl<'a, const O: u8> DPSRSTF_W<'a, O> {
    #[doc = "Deep software standby mode cancelation not requested by an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPSRSTF_A::_0)
    }
    #[doc = "Deep software standby mode cancelation requested by an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPSRSTF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Power-On Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Monitor 0 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn lvd0rf(&self) -> LVD0RF_R {
        LVD0RF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn lvd1rf(&self) -> LVD1RF_R {
        LVD1RF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Monitor 2 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn lvd2rf(&self) -> LVD2RF_R {
        LVD2RF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Deep Software Standby Reset FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn dpsrstf(&self) -> DPSRSTF_R {
        DPSRSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-On Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    #[must_use]
    pub fn porf(&mut self) -> PORF_W<0> {
        PORF_W::new(self)
    }
    #[doc = "Bit 1 - Voltage Monitor 0 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    #[must_use]
    pub fn lvd0rf(&mut self) -> LVD0RF_W<1> {
        LVD0RF_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    #[must_use]
    pub fn lvd1rf(&mut self) -> LVD1RF_W<2> {
        LVD1RF_W::new(self)
    }
    #[doc = "Bit 3 - Voltage Monitor 2 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    #[must_use]
    pub fn lvd2rf(&mut self) -> LVD2RF_W<3> {
        LVD2RF_W::new(self)
    }
    #[doc = "Bit 7 - Deep Software Standby Reset FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    #[must_use]
    pub fn dpsrstf(&mut self) -> DPSRSTF_W<7> {
        DPSRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsr0](index.html) module"]
pub struct RSTSR0_SPEC;
impl crate::RegisterSpec for RSTSR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rstsr0::R](R) reader structure"]
impl crate::Readable for RSTSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstsr0::W](W) writer structure"]
impl crate::Writable for RSTSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTSR0 to value 0"]
impl crate::Resettable for RSTSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
