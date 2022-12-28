#[doc = "Register `TECR` reader"]
pub struct R(crate::R<TECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TECR` reader - Transmit error count functionTECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
pub type TECR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit error count functionTECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
    #[inline(always)]
    pub fn tecr(&self) -> TECR_R {
        TECR_R::new(self.bits)
    }
}
#[doc = "Transmit Error Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tecr](index.html) module"]
pub struct TECR_SPEC;
impl crate::RegisterSpec for TECR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tecr::R](R) reader structure"]
impl crate::Readable for TECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TECR to value 0"]
impl crate::Resettable for TECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
