#[doc = "Register `TZFOAD` reader"]
pub struct R(crate::R<TZFOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZFOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZFOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZFOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZFOAD` writer"]
pub struct W(crate::W<TZFOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZFOAD_SPEC>;
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
impl From<crate::W<TZFOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZFOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OAD` reader - Operation after detection"]
pub type OAD_R = crate::BitReader<OAD_A>;
#[doc = "Operation after detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    #[doc = "0: Non-maskable interrupt"]
    _0 = 0,
    #[doc = "1: Reset"]
    _1 = 1,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAD_A {
        match self.bits {
            false => OAD_A::_0,
            true => OAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
}
#[doc = "Field `OAD` writer - Operation after detection"]
pub type OAD_W<'a, const O: u8> = crate::BitWriter<'a, u16, TZFOAD_SPEC, OAD_A, O>;
impl<'a, const O: u8> OAD_W<'a, O> {
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAD_A::_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OAD_A::_1)
    }
}
#[doc = "Field `KEY` writer - KeyCode"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TZFOAD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Operation after detection"]
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation after detection"]
    #[inline(always)]
    #[must_use]
    pub fn oad(&mut self) -> OAD_W<0> {
        OAD_W::new(self)
    }
    #[doc = "Bits 8:15 - KeyCode"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TrustZone Filter Operation After Detection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzfoad](index.html) module"]
pub struct TZFOAD_SPEC;
impl crate::RegisterSpec for TZFOAD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tzfoad::R](R) reader structure"]
impl crate::Readable for TZFOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzfoad::W](W) writer structure"]
impl crate::Writable for TZFOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZFOAD to value 0"]
impl crate::Resettable for TZFOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
