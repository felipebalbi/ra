#[doc = "Register `ADSHSTR0` reader"]
pub struct R(crate::R<ADSHSTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSHSTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSHSTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSHSTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSHSTR0` writer"]
pub struct W(crate::W<ADSHSTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSHSTR0_SPEC>;
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
impl From<crate::W<ADSHSTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSHSTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHSST` reader - Channel-dedicated Sample-and-hold Circuit Unit 0 to 2"]
pub type SHSST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHSST` writer - Channel-dedicated Sample-and-hold Circuit Unit 0 to 2"]
pub type SHSST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSHSTR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `SHHST` reader - Channel-dedicated Sample-and-hold Circuit Unit 0 to 2"]
pub type SHHST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHHST` writer - Channel-dedicated Sample-and-hold Circuit Unit 0 to 2"]
pub type SHHST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSHSTR0_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Channel-dedicated Sample-and-hold Circuit Unit 0 to 2"]
    #[inline(always)]
    pub fn shsst(&self) -> SHSST_R {
        SHSST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Channel-dedicated Sample-and-hold Circuit Unit 0 to 2"]
    #[inline(always)]
    pub fn shhst(&self) -> SHHST_R {
        SHHST_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel-dedicated Sample-and-hold Circuit Unit 0 to 2"]
    #[inline(always)]
    #[must_use]
    pub fn shsst(&mut self) -> SHSST_W<0> {
        SHSST_W::new(self)
    }
    #[doc = "Bits 16:18 - Channel-dedicated Sample-and-hold Circuit Unit 0 to 2"]
    #[inline(always)]
    #[must_use]
    pub fn shhst(&mut self) -> SHHST_W<16> {
        SHHST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel-dedicated Sample-and-hold Circuit State Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adshstr0](index.html) module"]
pub struct ADSHSTR0_SPEC;
impl crate::RegisterSpec for ADSHSTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adshstr0::R](R) reader structure"]
impl crate::Readable for ADSHSTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adshstr0::W](W) writer structure"]
impl crate::Writable for ADSHSTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSHSTR0 to value 0x0002_0004"]
impl crate::Resettable for ADSHSTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0004;
}
