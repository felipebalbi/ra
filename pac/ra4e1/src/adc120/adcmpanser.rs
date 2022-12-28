#[doc = "Register `ADCMPANSER` reader"]
pub struct R(crate::R<ADCMPANSER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPANSER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPANSER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPANSER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPANSER` writer"]
pub struct W(crate::W<ADCMPANSER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPANSER_SPEC>;
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
impl From<crate::W<ADCMPANSER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPANSER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPOCA` reader - Internal Reference Voltage Compare Select"]
pub type CMPOCA_R = crate::BitReader<CMPOCA_A>;
#[doc = "Internal Reference Voltage Compare Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOCA_A {
    #[doc = "0: Exclude the internal reference voltage from the compare Window A target range."]
    _0 = 0,
    #[doc = "1: Include the internal reference voltage in the compare Window A target range."]
    _1 = 1,
}
impl From<CMPOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPOCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOCA_A {
        match self.bits {
            false => CMPOCA_A::_0,
            true => CMPOCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPOCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPOCA_A::_1
    }
}
#[doc = "Field `CMPOCA` writer - Internal Reference Voltage Compare Select"]
pub type CMPOCA_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCMPANSER_SPEC, CMPOCA_A, O>;
impl<'a, const O: u8> CMPOCA_W<'a, O> {
    #[doc = "Exclude the internal reference voltage from the compare Window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPOCA_A::_0)
    }
    #[doc = "Include the internal reference voltage in the compare Window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPOCA_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Internal Reference Voltage Compare Select"]
    #[inline(always)]
    pub fn cmpoca(&self) -> CMPOCA_R {
        CMPOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Internal Reference Voltage Compare Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpoca(&mut self) -> CMPOCA_W<1> {
        CMPOCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Extended Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpanser](index.html) module"]
pub struct ADCMPANSER_SPEC;
impl crate::RegisterSpec for ADCMPANSER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcmpanser::R](R) reader structure"]
impl crate::Readable for ADCMPANSER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpanser::W](W) writer structure"]
impl crate::Writable for ADCMPANSER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPANSER to value 0"]
impl crate::Resettable for ADCMPANSER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
