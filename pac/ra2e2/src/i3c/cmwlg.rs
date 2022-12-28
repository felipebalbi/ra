#[doc = "Register `CMWLG` reader"]
pub struct R(crate::R<CMWLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMWLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMWLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMWLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMWLG` writer"]
pub struct W(crate::W<CMWLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMWLG_SPEC>;
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
impl From<crate::W<CMWLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMWLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MWLG` reader - Max Write Length"]
pub type MWLG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MWLG` writer - Max Write Length"]
pub type MWLG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMWLG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Max Write Length"]
    #[inline(always)]
    pub fn mwlg(&self) -> MWLG_R {
        MWLG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Max Write Length"]
    #[inline(always)]
    #[must_use]
    pub fn mwlg(&mut self) -> MWLG_W<0> {
        MWLG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Max Write Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmwlg](index.html) module"]
pub struct CMWLG_SPEC;
impl crate::RegisterSpec for CMWLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmwlg::R](R) reader structure"]
impl crate::Readable for CMWLG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmwlg::W](W) writer structure"]
impl crate::Writable for CMWLG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMWLG to value 0"]
impl crate::Resettable for CMWLG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
