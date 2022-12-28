#[doc = "Register `PSPMPUEA` reader"]
pub struct R(crate::R<PSPMPUEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSPMPUEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSPMPUEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSPMPUEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSPMPUEA` writer"]
pub struct W(crate::W<PSPMPUEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSPMPUEA_SPEC>;
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
impl From<crate::W<PSPMPUEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSPMPUEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSPMPUEA` reader - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type PSPMPUEA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PSPMPUEA` writer - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type PSPMPUEA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSPMPUEA_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn pspmpuea(&self) -> PSPMPUEA_R {
        PSPMPUEA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    #[must_use]
    pub fn pspmpuea(&mut self) -> PSPMPUEA_W<2> {
        PSPMPUEA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Process Stack Pointer Monitor End Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pspmpuea](index.html) module"]
pub struct PSPMPUEA_SPEC;
impl crate::RegisterSpec for PSPMPUEA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pspmpuea::R](R) reader structure"]
impl crate::Readable for PSPMPUEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pspmpuea::W](W) writer structure"]
impl crate::Writable for PSPMPUEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSPMPUEA to value 0x03"]
impl crate::Resettable for PSPMPUEA_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
