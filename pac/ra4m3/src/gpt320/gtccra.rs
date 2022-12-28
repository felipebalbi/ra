#[doc = "Register `GTCCRA` reader"]
pub struct R(crate::R<GTCCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCCRA` writer"]
pub struct W(crate::W<GTCCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCCRA_SPEC>;
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
impl From<crate::W<GTCCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCCRA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Compare Capture Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtccra](index.html) module"]
pub struct GTCCRA_SPEC;
impl crate::RegisterSpec for GTCCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtccra::R](R) reader structure"]
impl crate::Readable for GTCCRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtccra::W](W) writer structure"]
impl crate::Writable for GTCCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRA to value 0xffff_ffff"]
impl crate::Resettable for GTCCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
