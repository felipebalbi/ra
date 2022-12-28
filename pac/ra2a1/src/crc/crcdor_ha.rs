#[doc = "Register `CRCDOR_HA` reader"]
pub struct R(crate::R<CRCDOR_HA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDOR_HA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDOR_HA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDOR_HA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDOR_HA` writer"]
pub struct W(crate::W<CRCDOR_HA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDOR_HA_SPEC>;
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
impl From<crate::W<CRCDOR_HA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDOR_HA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCDOR_HA` reader - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
pub type CRCDOR_HA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRCDOR_HA` writer - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
pub type CRCDOR_HA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CRCDOR_HA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdor_ha(&self) -> CRCDOR_HA_R {
        CRCDOR_HA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdor_ha(&mut self) -> CRCDOR_HA_W<0> {
        CRCDOR_HA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Output Register (halfword access)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdor_ha](index.html) module"]
pub struct CRCDOR_HA_SPEC;
impl crate::RegisterSpec for CRCDOR_HA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crcdor_ha::R](R) reader structure"]
impl crate::Readable for CRCDOR_HA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdor_ha::W](W) writer structure"]
impl crate::Writable for CRCDOR_HA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDOR_HA to value 0"]
impl crate::Resettable for CRCDOR_HA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
