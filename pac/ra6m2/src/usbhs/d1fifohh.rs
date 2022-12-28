#[doc = "Register `D1FIFOHH` reader"]
pub struct R(crate::R<D1FIFOHH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D1FIFOHH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D1FIFOHH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D1FIFOHH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D1FIFOHH` writer"]
pub struct W(crate::W<D1FIFOHH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D1FIFOHH_SPEC>;
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
impl From<crate::W<D1FIFOHH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D1FIFOHH_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D1FIFO Port Register HH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d1fifohh](index.html) module"]
pub struct D1FIFOHH_SPEC;
impl crate::RegisterSpec for D1FIFOHH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [d1fifohh::R](R) reader structure"]
impl crate::Readable for D1FIFOHH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d1fifohh::W](W) writer structure"]
impl crate::Writable for D1FIFOHH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D1FIFOHH to value 0"]
impl crate::Resettable for D1FIFOHH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
