#[doc = "Register `SSLP` reader"]
pub struct R(crate::R<SSLP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSLP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSLP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSLP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSLP` writer"]
pub struct W(crate::W<SSLP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSLP_SPEC>;
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
impl From<crate::W<SSLP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSLP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSL0P` reader - SSLn0 Signal Polarity Setting"]
pub type SSL0P_R = crate::BitReader<SSL0P_A>;
#[doc = "SSLn0 Signal Polarity Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL0P_A {
    #[doc = "0: Set SSLn0 signal to active-low"]
    _0 = 0,
    #[doc = "1: Set SSLn0 signal to active-high"]
    _1 = 1,
}
impl From<SSL0P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL0P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL0P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL0P_A {
        match self.bits {
            false => SSL0P_A::_0,
            true => SSL0P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL0P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL0P_A::_1
    }
}
#[doc = "Field `SSL0P` writer - SSLn0 Signal Polarity Setting"]
pub type SSL0P_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSLP_SPEC, SSL0P_A, O>;
impl<'a, const O: u8> SSL0P_W<'a, O> {
    #[doc = "Set SSLn0 signal to active-low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL0P_A::_0)
    }
    #[doc = "Set SSLn0 signal to active-high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL0P_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SSLn0 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl0p(&self) -> SSL0P_R {
        SSL0P_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSLn0 Signal Polarity Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ssl0p(&mut self) -> SSL0P_W<0> {
        SSL0P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Slave Select Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sslp](index.html) module"]
pub struct SSLP_SPEC;
impl crate::RegisterSpec for SSLP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sslp::R](R) reader structure"]
impl crate::Readable for SSLP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sslp::W](W) writer structure"]
impl crate::Writable for SSLP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSLP to value 0"]
impl crate::Resettable for SSLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
