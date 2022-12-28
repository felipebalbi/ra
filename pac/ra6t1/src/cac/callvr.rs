#[doc = "Register `CALLVR` reader"]
pub struct R(crate::R<CALLVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALLVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALLVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALLVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALLVR` writer"]
pub struct W(crate::W<CALLVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALLVR_SPEC>;
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
impl From<crate::W<CALLVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALLVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALLVR` reader - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
pub type CALLVR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALLVR` writer - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
pub type CALLVR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CALLVR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
    #[inline(always)]
    pub fn callvr(&self) -> CALLVR_R {
        CALLVR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
    #[inline(always)]
    #[must_use]
    pub fn callvr(&mut self) -> CALLVR_W<0> {
        CALLVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAC Lower-Limit Value Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [callvr](index.html) module"]
pub struct CALLVR_SPEC;
impl crate::RegisterSpec for CALLVR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [callvr::R](R) reader structure"]
impl crate::Readable for CALLVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [callvr::W](W) writer structure"]
impl crate::Writable for CALLVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALLVR to value 0"]
impl crate::Resettable for CALLVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
