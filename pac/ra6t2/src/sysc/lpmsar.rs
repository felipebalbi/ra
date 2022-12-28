#[doc = "Register `LPMSAR` reader"]
pub struct R(crate::R<LPMSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMSAR` writer"]
pub struct W(crate::W<LPMSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMSAR_SPEC>;
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
impl From<crate::W<LPMSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONSEC0` reader - Non Secure Attribute bit 0"]
pub type NONSEC0_R = crate::BitReader<NONSEC0_A>;
#[doc = "Non Secure Attribute bit 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC0_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC0_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC0_A {
        match self.bits {
            false => NONSEC0_A::_0,
            true => NONSEC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC0_A::_1
    }
}
#[doc = "Field `NONSEC0` writer - Non Secure Attribute bit 0"]
pub type NONSEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMSAR_SPEC, NONSEC0_A, O>;
impl<'a, const O: u8> NONSEC0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC0_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC0_A::_1)
    }
}
#[doc = "Field `NONSEC2` reader - Non Secure Attribute bit 2"]
pub type NONSEC2_R = crate::BitReader<NONSEC2_A>;
#[doc = "Non Secure Attribute bit 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC2_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC2_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC2_A {
        match self.bits {
            false => NONSEC2_A::_0,
            true => NONSEC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC2_A::_1
    }
}
#[doc = "Field `NONSEC2` writer - Non Secure Attribute bit 2"]
pub type NONSEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMSAR_SPEC, NONSEC2_A, O>;
impl<'a, const O: u8> NONSEC2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC2_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC2_A::_1)
    }
}
#[doc = "Field `NONSEC4` reader - Non Secure Attribute bit 4"]
pub type NONSEC4_R = crate::BitReader<NONSEC4_A>;
#[doc = "Non Secure Attribute bit 4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC4_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC4_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC4_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC4_A {
        match self.bits {
            false => NONSEC4_A::_0,
            true => NONSEC4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC4_A::_1
    }
}
#[doc = "Field `NONSEC4` writer - Non Secure Attribute bit 4"]
pub type NONSEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMSAR_SPEC, NONSEC4_A, O>;
impl<'a, const O: u8> NONSEC4_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC4_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC4_A::_1)
    }
}
#[doc = "Field `NONSEC8` reader - Non Secure Attribute bit 8"]
pub type NONSEC8_R = crate::BitReader<NONSEC8_A>;
#[doc = "Non Secure Attribute bit 8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC8_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC8_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC8_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC8_A {
        match self.bits {
            false => NONSEC8_A::_0,
            true => NONSEC8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC8_A::_1
    }
}
#[doc = "Field `NONSEC8` writer - Non Secure Attribute bit 8"]
pub type NONSEC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMSAR_SPEC, NONSEC8_A, O>;
impl<'a, const O: u8> NONSEC8_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC8_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC8_A::_1)
    }
}
#[doc = "Field `NONSEC9` reader - Non Secure Attribute bit 9"]
pub type NONSEC9_R = crate::BitReader<NONSEC9_A>;
#[doc = "Non Secure Attribute bit 9\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC9_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC9_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC9_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC9_A {
        match self.bits {
            false => NONSEC9_A::_0,
            true => NONSEC9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC9_A::_1
    }
}
#[doc = "Field `NONSEC9` writer - Non Secure Attribute bit 9"]
pub type NONSEC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMSAR_SPEC, NONSEC9_A, O>;
impl<'a, const O: u8> NONSEC9_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC9_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC9_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Non Secure Attribute bit 0"]
    #[inline(always)]
    pub fn nonsec0(&self) -> NONSEC0_R {
        NONSEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Non Secure Attribute bit 2"]
    #[inline(always)]
    pub fn nonsec2(&self) -> NONSEC2_R {
        NONSEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Non Secure Attribute bit 4"]
    #[inline(always)]
    pub fn nonsec4(&self) -> NONSEC4_R {
        NONSEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Non Secure Attribute bit 8"]
    #[inline(always)]
    pub fn nonsec8(&self) -> NONSEC8_R {
        NONSEC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Non Secure Attribute bit 9"]
    #[inline(always)]
    pub fn nonsec9(&self) -> NONSEC9_R {
        NONSEC9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure Attribute bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec0(&mut self) -> NONSEC0_W<0> {
        NONSEC0_W::new(self)
    }
    #[doc = "Bit 2 - Non Secure Attribute bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec2(&mut self) -> NONSEC2_W<2> {
        NONSEC2_W::new(self)
    }
    #[doc = "Bit 4 - Non Secure Attribute bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec4(&mut self) -> NONSEC4_W<4> {
        NONSEC4_W::new(self)
    }
    #[doc = "Bit 8 - Non Secure Attribute bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec8(&mut self) -> NONSEC8_W<8> {
        NONSEC8_W::new(self)
    }
    #[doc = "Bit 9 - Non Secure Attribute bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec9(&mut self) -> NONSEC9_W<9> {
        NONSEC9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Mode Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmsar](index.html) module"]
pub struct LPMSAR_SPEC;
impl crate::RegisterSpec for LPMSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmsar::R](R) reader structure"]
impl crate::Readable for LPMSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmsar::W](W) writer structure"]
impl crate::Writable for LPMSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMSAR to value 0xffff_ffff"]
impl crate::Resettable for LPMSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
