#[doc = "Register `ADANSA1` reader"]
pub struct R(crate::R<ADANSA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSA1` writer"]
pub struct W(crate::W<ADANSA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSA1_SPEC>;
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
impl From<crate::W<ADANSA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSA16` reader - AN016 Select"]
pub type ANSA16_R = crate::BitReader<ANSA16_A>;
#[doc = "AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA16_A {
    #[doc = "0: AN016 is not selected"]
    _0 = 0,
    #[doc = "1: AN016 is selected"]
    _1 = 1,
}
impl From<ANSA16_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA16_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA16_A {
        match self.bits {
            false => ANSA16_A::_0,
            true => ANSA16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA16_A::_1
    }
}
#[doc = "Field `ANSA16` writer - AN016 Select"]
pub type ANSA16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA16_A, O>;
impl<'a, const O: u8> ANSA16_W<'a, O> {
    #[doc = "AN016 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA16_A::_0)
    }
    #[doc = "AN016 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA16_A::_1)
    }
}
#[doc = "Field `ANSA17` reader - AN017 Select"]
pub type ANSA17_R = crate::BitReader<ANSA17_A>;
#[doc = "AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA17_A {
    #[doc = "0: AN017 is not selected"]
    _0 = 0,
    #[doc = "1: AN017 is selected"]
    _1 = 1,
}
impl From<ANSA17_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA17_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA17_A {
        match self.bits {
            false => ANSA17_A::_0,
            true => ANSA17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA17_A::_1
    }
}
#[doc = "Field `ANSA17` writer - AN017 Select"]
pub type ANSA17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA17_A, O>;
impl<'a, const O: u8> ANSA17_W<'a, O> {
    #[doc = "AN017 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA17_A::_0)
    }
    #[doc = "AN017 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA17_A::_1)
    }
}
#[doc = "Field `ANSA18` reader - AN018 Select"]
pub type ANSA18_R = crate::BitReader<ANSA18_A>;
#[doc = "AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA18_A {
    #[doc = "0: AN018 is not selected"]
    _0 = 0,
    #[doc = "1: AN018 is selected"]
    _1 = 1,
}
impl From<ANSA18_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA18_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA18_A {
        match self.bits {
            false => ANSA18_A::_0,
            true => ANSA18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA18_A::_1
    }
}
#[doc = "Field `ANSA18` writer - AN018 Select"]
pub type ANSA18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA18_A, O>;
impl<'a, const O: u8> ANSA18_W<'a, O> {
    #[doc = "AN018 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA18_A::_0)
    }
    #[doc = "AN018 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA18_A::_1)
    }
}
#[doc = "Field `ANSA19` reader - AN019 Select"]
pub type ANSA19_R = crate::BitReader<ANSA19_A>;
#[doc = "AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA19_A {
    #[doc = "0: AN019 is not selected"]
    _0 = 0,
    #[doc = "1: AN019 is selected"]
    _1 = 1,
}
impl From<ANSA19_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA19_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA19_A {
        match self.bits {
            false => ANSA19_A::_0,
            true => ANSA19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA19_A::_1
    }
}
#[doc = "Field `ANSA19` writer - AN019 Select"]
pub type ANSA19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA19_A, O>;
impl<'a, const O: u8> ANSA19_W<'a, O> {
    #[doc = "AN019 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA19_A::_0)
    }
    #[doc = "AN019 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA19_A::_1)
    }
}
#[doc = "Field `ANSA20` reader - AN020 Select"]
pub type ANSA20_R = crate::BitReader<ANSA20_A>;
#[doc = "AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA20_A {
    #[doc = "0: AN020 is not selected"]
    _0 = 0,
    #[doc = "1: AN020 is selected"]
    _1 = 1,
}
impl From<ANSA20_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA20_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA20_A {
        match self.bits {
            false => ANSA20_A::_0,
            true => ANSA20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA20_A::_1
    }
}
#[doc = "Field `ANSA20` writer - AN020 Select"]
pub type ANSA20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA20_A, O>;
impl<'a, const O: u8> ANSA20_W<'a, O> {
    #[doc = "AN020 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA20_A::_0)
    }
    #[doc = "AN020 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA20_A::_1)
    }
}
#[doc = "Field `ANSA21` reader - AN021 Select"]
pub type ANSA21_R = crate::BitReader<ANSA21_A>;
#[doc = "AN021 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA21_A {
    #[doc = "0: AN021 is not selected"]
    _0 = 0,
    #[doc = "1: AN021 is selected"]
    _1 = 1,
}
impl From<ANSA21_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA21_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA21_A {
        match self.bits {
            false => ANSA21_A::_0,
            true => ANSA21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA21_A::_1
    }
}
#[doc = "Field `ANSA21` writer - AN021 Select"]
pub type ANSA21_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA21_A, O>;
impl<'a, const O: u8> ANSA21_W<'a, O> {
    #[doc = "AN021 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA21_A::_0)
    }
    #[doc = "AN021 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA21_A::_1)
    }
}
#[doc = "Field `ANSA22` reader - AN022 Select"]
pub type ANSA22_R = crate::BitReader<ANSA22_A>;
#[doc = "AN022 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA22_A {
    #[doc = "0: AN022 is not selected"]
    _0 = 0,
    #[doc = "1: AN022 is selected"]
    _1 = 1,
}
impl From<ANSA22_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA22_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA22_A {
        match self.bits {
            false => ANSA22_A::_0,
            true => ANSA22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA22_A::_1
    }
}
#[doc = "Field `ANSA22` writer - AN022 Select"]
pub type ANSA22_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA22_A, O>;
impl<'a, const O: u8> ANSA22_W<'a, O> {
    #[doc = "AN022 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA22_A::_0)
    }
    #[doc = "AN022 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA22_A::_1)
    }
}
#[doc = "Field `ANSA23` reader - AN023 Select"]
pub type ANSA23_R = crate::BitReader<ANSA23_A>;
#[doc = "AN023 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA23_A {
    #[doc = "0: AN023 is not selected"]
    _0 = 0,
    #[doc = "1: AN023 is selected"]
    _1 = 1,
}
impl From<ANSA23_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA23_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA23_A {
        match self.bits {
            false => ANSA23_A::_0,
            true => ANSA23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA23_A::_1
    }
}
#[doc = "Field `ANSA23` writer - AN023 Select"]
pub type ANSA23_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA23_A, O>;
impl<'a, const O: u8> ANSA23_W<'a, O> {
    #[doc = "AN023 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA23_A::_0)
    }
    #[doc = "AN023 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA23_A::_1)
    }
}
#[doc = "Field `ANSA24` reader - AN024 Select"]
pub type ANSA24_R = crate::BitReader<ANSA24_A>;
#[doc = "AN024 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA24_A {
    #[doc = "0: AN024 is not selected"]
    _0 = 0,
    #[doc = "1: AN024 is selected"]
    _1 = 1,
}
impl From<ANSA24_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA24_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA24_A {
        match self.bits {
            false => ANSA24_A::_0,
            true => ANSA24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA24_A::_1
    }
}
#[doc = "Field `ANSA24` writer - AN024 Select"]
pub type ANSA24_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA24_A, O>;
impl<'a, const O: u8> ANSA24_W<'a, O> {
    #[doc = "AN024 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA24_A::_0)
    }
    #[doc = "AN024 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA24_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn ansa16(&self) -> ANSA16_R {
        ANSA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn ansa17(&self) -> ANSA17_R {
        ANSA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn ansa18(&self) -> ANSA18_R {
        ANSA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn ansa19(&self) -> ANSA19_R {
        ANSA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn ansa20(&self) -> ANSA20_R {
        ANSA20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn ansa21(&self) -> ANSA21_R {
        ANSA21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn ansa22(&self) -> ANSA22_R {
        ANSA22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn ansa23(&self) -> ANSA23_R {
        ANSA23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn ansa24(&self) -> ANSA24_R {
        ANSA24_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa16(&mut self) -> ANSA16_W<0> {
        ANSA16_W::new(self)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa17(&mut self) -> ANSA17_W<1> {
        ANSA17_W::new(self)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa18(&mut self) -> ANSA18_W<2> {
        ANSA18_W::new(self)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa19(&mut self) -> ANSA19_W<3> {
        ANSA19_W::new(self)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa20(&mut self) -> ANSA20_W<4> {
        ANSA20_W::new(self)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa21(&mut self) -> ANSA21_W<5> {
        ANSA21_W::new(self)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa22(&mut self) -> ANSA22_W<6> {
        ANSA22_W::new(self)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa23(&mut self) -> ANSA23_W<7> {
        ANSA23_W::new(self)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa24(&mut self) -> ANSA24_W<8> {
        ANSA24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansa1](index.html) module"]
pub struct ADANSA1_SPEC;
impl crate::RegisterSpec for ADANSA1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansa1::R](R) reader structure"]
impl crate::Readable for ADANSA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansa1::W](W) writer structure"]
impl crate::Writable for ADANSA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSA1 to value 0"]
impl crate::Resettable for ADANSA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
