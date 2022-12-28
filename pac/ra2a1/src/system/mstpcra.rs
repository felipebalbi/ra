#[doc = "Register `MSTPCRA` reader"]
pub struct R(crate::R<MSTPCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRA` writer"]
pub struct W(crate::W<MSTPCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRA_SPEC>;
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
impl From<crate::W<MSTPCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPA22` reader - Data Transfer Controller Module Stop"]
pub type MSTPA22_R = crate::BitReader<MSTPA22_A>;
#[doc = "Data Transfer Controller Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA22_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPA22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA22_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPA22_A {
        match self.bits {
            false => MSTPA22_A::_0,
            true => MSTPA22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA22_A::_1
    }
}
#[doc = "Field `MSTPA22` writer - Data Transfer Controller Module Stop"]
pub type MSTPA22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRA_SPEC, MSTPA22_A, O>;
impl<'a, const O: u8> MSTPA22_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPA22_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPA22_A::_1)
    }
}
impl R {
    #[doc = "Bit 22 - Data Transfer Controller Module Stop"]
    #[inline(always)]
    pub fn mstpa22(&self) -> MSTPA22_R {
        MSTPA22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Data Transfer Controller Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpa22(&mut self) -> MSTPA22_W<22> {
        MSTPA22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcra](index.html) module"]
pub struct MSTPCRA_SPEC;
impl crate::RegisterSpec for MSTPCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcra::R](R) reader structure"]
impl crate::Readable for MSTPCRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcra::W](W) writer structure"]
impl crate::Writable for MSTPCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRA to value 0xffbf_ffff"]
impl crate::Resettable for MSTPCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffbf_ffff;
}
