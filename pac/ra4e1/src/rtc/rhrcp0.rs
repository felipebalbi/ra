#[doc = "Register `RHRCP0` reader"]
pub struct R(crate::R<RHRCP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RHRCP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RHRCP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RHRCP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HR1` reader - 1-Hour Capture"]
pub type HR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HR10` reader - 10-Hour Capture"]
pub type HR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PM` reader - PM"]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "PM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    #[doc = "0: AM"]
    _0 = 0,
    #[doc = "1: PM"]
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Hour Capture"]
    #[inline(always)]
    pub fn hr1(&self) -> HR1_R {
        HR1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Hour Capture"]
    #[inline(always)]
    pub fn hr10(&self) -> HR10_R {
        HR10_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Hour Capture Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhrcp0](index.html) module"]
pub struct RHRCP0_SPEC;
impl crate::RegisterSpec for RHRCP0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rhrcp0::R](R) reader structure"]
impl crate::Readable for RHRCP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RHRCP0 to value 0"]
impl crate::Resettable for RHRCP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
