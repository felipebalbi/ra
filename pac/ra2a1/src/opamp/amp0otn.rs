#[doc = "Register `AMP0OTN` reader"]
pub struct R(crate::R<AMP0OTN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMP0OTN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMP0OTN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMP0OTN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMP0OTN` writer"]
pub struct W(crate::W<AMP0OTN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMP0OTN_SPEC>;
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
impl From<crate::W<AMP0OTN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMP0OTN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRMN` reader - AMP0 input offset trimming Nch side"]
pub type TRMN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRMN` writer - AMP0 input offset trimming Nch side"]
pub type TRMN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, AMP0OTN_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - AMP0 input offset trimming Nch side"]
    #[inline(always)]
    pub fn trmn(&self) -> TRMN_R {
        TRMN_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - AMP0 input offset trimming Nch side"]
    #[inline(always)]
    #[must_use]
    pub fn trmn(&mut self) -> TRMN_W<0> {
        TRMN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier 0 Offset Trimming Nch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amp0otn](index.html) module"]
pub struct AMP0OTN_SPEC;
impl crate::RegisterSpec for AMP0OTN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amp0otn::R](R) reader structure"]
impl crate::Readable for AMP0OTN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amp0otn::W](W) writer structure"]
impl crate::Writable for AMP0OTN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMP0OTN to value 0"]
impl crate::Resettable for AMP0OTN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
