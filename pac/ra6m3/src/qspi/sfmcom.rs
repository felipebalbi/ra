#[doc = "Register `SFMCOM` reader"]
pub struct R(crate::R<SFMCOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMCOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMCOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMCOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMCOM` writer"]
pub struct W(crate::W<SFMCOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMCOM_SPEC>;
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
impl From<crate::W<SFMCOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMCOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMD` reader - Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode."]
pub type SFMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFMD` writer - Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode."]
pub type SFMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFMCOM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode."]
    #[inline(always)]
    pub fn sfmd(&self) -> SFMD_R {
        SFMD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode."]
    #[inline(always)]
    #[must_use]
    pub fn sfmd(&mut self) -> SFMD_W<0> {
        SFMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Communication Port Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmcom](index.html) module"]
pub struct SFMCOM_SPEC;
impl crate::RegisterSpec for SFMCOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmcom::R](R) reader structure"]
impl crate::Readable for SFMCOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmcom::W](W) writer structure"]
impl crate::Writable for SFMCOM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMCOM to value 0"]
impl crate::Resettable for SFMCOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
