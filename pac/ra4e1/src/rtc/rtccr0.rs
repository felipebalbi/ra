#[doc = "Register `RTCCR0` reader"]
pub struct R(crate::R<RTCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCR0` writer"]
pub struct W(crate::W<RTCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCR0_SPEC>;
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
impl From<crate::W<RTCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCCT` reader - Time Capture Control"]
pub type TCCT_R = crate::FieldReader<u8, TCCT_A>;
#[doc = "Time Capture Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCT_A {
    #[doc = "0: Do not detect events"]
    _00 = 0,
    #[doc = "1: Detect rising edge"]
    _01 = 1,
    #[doc = "2: Detect falling edge"]
    _10 = 2,
    #[doc = "3: Detect both edges"]
    _11 = 3,
}
impl From<TCCT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCT_A) -> Self {
        variant as _
    }
}
impl TCCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCT_A {
        match self.bits {
            0 => TCCT_A::_00,
            1 => TCCT_A::_01,
            2 => TCCT_A::_10,
            3 => TCCT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TCCT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCCT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCCT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCCT_A::_11
    }
}
#[doc = "Field `TCCT` writer - Time Capture Control"]
pub type TCCT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RTCCR0_SPEC, u8, TCCT_A, 2, O>;
impl<'a, const O: u8> TCCT_W<'a, O> {
    #[doc = "Do not detect events"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TCCT_A::_00)
    }
    #[doc = "Detect rising edge"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TCCT_A::_01)
    }
    #[doc = "Detect falling edge"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TCCT_A::_10)
    }
    #[doc = "Detect both edges"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TCCT_A::_11)
    }
}
#[doc = "Field `TCST` reader - Time Capture Status"]
pub type TCST_R = crate::BitReader<TCST_A>;
#[doc = "Time Capture Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCST_A {
    #[doc = "0: No event detected"]
    _0 = 0,
    #[doc = "1: Event detected"]
    _1 = 1,
}
impl From<TCST_A> for bool {
    #[inline(always)]
    fn from(variant: TCST_A) -> Self {
        variant as u8 != 0
    }
}
impl TCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCST_A {
        match self.bits {
            false => TCST_A::_0,
            true => TCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCST_A::_1
    }
}
#[doc = "Field `TCST` writer - Time Capture Status"]
pub type TCST_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTCCR0_SPEC, TCST_A, O>;
impl<'a, const O: u8> TCST_W<'a, O> {
    #[doc = "No event detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCST_A::_0)
    }
    #[doc = "Event detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCST_A::_1)
    }
}
#[doc = "Field `TCNF` reader - Time Capture Noise Filter Control"]
pub type TCNF_R = crate::FieldReader<u8, TCNF_A>;
#[doc = "Time Capture Noise Filter Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCNF_A {
    #[doc = "0: Turn noise filter off"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: Turn noise filter on (count source)"]
    _10 = 2,
    #[doc = "3: Turn noise filter on (count source by divided by 32)"]
    _11 = 3,
}
impl From<TCNF_A> for u8 {
    #[inline(always)]
    fn from(variant: TCNF_A) -> Self {
        variant as _
    }
}
impl TCNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCNF_A {
        match self.bits {
            0 => TCNF_A::_00,
            1 => TCNF_A::_01,
            2 => TCNF_A::_10,
            3 => TCNF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TCNF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCNF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCNF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCNF_A::_11
    }
}
#[doc = "Field `TCNF` writer - Time Capture Noise Filter Control"]
pub type TCNF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RTCCR0_SPEC, u8, TCNF_A, 2, O>;
impl<'a, const O: u8> TCNF_W<'a, O> {
    #[doc = "Turn noise filter off"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TCNF_A::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TCNF_A::_01)
    }
    #[doc = "Turn noise filter on (count source)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TCNF_A::_10)
    }
    #[doc = "Turn noise filter on (count source by divided by 32)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TCNF_A::_11)
    }
}
#[doc = "Field `TCEN` reader - Time Capture Event Input Pin Enable"]
pub type TCEN_R = crate::BitReader<TCEN_A>;
#[doc = "Time Capture Event Input Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCEN_A {
    #[doc = "0: Disable the RTCICn pin as the time capture event input pin"]
    _0 = 0,
    #[doc = "1: Enable the RTCICn pin as the time capture event input pin"]
    _1 = 1,
}
impl From<TCEN_A> for bool {
    #[inline(always)]
    fn from(variant: TCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCEN_A {
        match self.bits {
            false => TCEN_A::_0,
            true => TCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCEN_A::_1
    }
}
#[doc = "Field `TCEN` writer - Time Capture Event Input Pin Enable"]
pub type TCEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTCCR0_SPEC, TCEN_A, O>;
impl<'a, const O: u8> TCEN_W<'a, O> {
    #[doc = "Disable the RTCICn pin as the time capture event input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCEN_A::_0)
    }
    #[doc = "Enable the RTCICn pin as the time capture event input pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Time Capture Control"]
    #[inline(always)]
    pub fn tcct(&self) -> TCCT_R {
        TCCT_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Time Capture Status"]
    #[inline(always)]
    pub fn tcst(&self) -> TCST_R {
        TCST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Time Capture Noise Filter Control"]
    #[inline(always)]
    pub fn tcnf(&self) -> TCNF_R {
        TCNF_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Time Capture Event Input Pin Enable"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Time Capture Control"]
    #[inline(always)]
    #[must_use]
    pub fn tcct(&mut self) -> TCCT_W<0> {
        TCCT_W::new(self)
    }
    #[doc = "Bit 2 - Time Capture Status"]
    #[inline(always)]
    #[must_use]
    pub fn tcst(&mut self) -> TCST_W<2> {
        TCST_W::new(self)
    }
    #[doc = "Bits 4:5 - Time Capture Noise Filter Control"]
    #[inline(always)]
    #[must_use]
    pub fn tcnf(&mut self) -> TCNF_W<4> {
        TCNF_W::new(self)
    }
    #[doc = "Bit 7 - Time Capture Event Input Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<7> {
        TCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Capture Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccr0](index.html) module"]
pub struct RTCCR0_SPEC;
impl crate::RegisterSpec for RTCCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtccr0::R](R) reader structure"]
impl crate::Readable for RTCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccr0::W](W) writer structure"]
impl crate::Writable for RTCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCR0 to value 0"]
impl crate::Resettable for RTCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
