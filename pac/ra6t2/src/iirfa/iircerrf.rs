#[doc = "Register `IIRCERRF` reader"]
pub struct R(crate::R<IIRCERRF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRCERRF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRCERRF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRCERRF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CERRF` reader - Operation error flag"]
pub type CERRF_R = crate::FieldReader<u16, CERRF_A>;
#[doc = "Operation error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CERRF_A {
    #[doc = "0: No operation error has occurred in the corresponding channel."]
    _0 = 0,
    #[doc = "1: An operation error has occurred in the corresponding channel."]
    _1 = 1,
}
impl From<CERRF_A> for u16 {
    #[inline(always)]
    fn from(variant: CERRF_A) -> Self {
        variant as _
    }
}
impl CERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CERRF_A> {
        match self.bits {
            0 => Some(CERRF_A::_0),
            1 => Some(CERRF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CERRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CERRF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Operation error flag"]
    #[inline(always)]
    pub fn cerrf(&self) -> CERRF_R {
        CERRF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Operation Error Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iircerrf](index.html) module"]
pub struct IIRCERRF_SPEC;
impl crate::RegisterSpec for IIRCERRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iircerrf::R](R) reader structure"]
impl crate::Readable for IIRCERRF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIRCERRF to value 0"]
impl crate::Resettable for IIRCERRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
