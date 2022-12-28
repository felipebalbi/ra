#[doc = "Register `ICWUR` reader"]
pub struct R(crate::R<ICWUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICWUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICWUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICWUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICWUR` writer"]
pub struct W(crate::W<ICWUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICWUR_SPEC>;
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
impl From<crate::W<ICWUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICWUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUAFA` reader - Wake-Up Analog Filter Additional Selection"]
pub type WUAFA_R = crate::BitReader<WUAFA_A>;
#[doc = "Wake-Up Analog Filter Additional Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUAFA_A {
    #[doc = "0: Do not add the Wake Up analog filter."]
    _0 = 0,
    #[doc = "1: Add the Wake Up analog filter."]
    _1 = 1,
}
impl From<WUAFA_A> for bool {
    #[inline(always)]
    fn from(variant: WUAFA_A) -> Self {
        variant as u8 != 0
    }
}
impl WUAFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUAFA_A {
        match self.bits {
            false => WUAFA_A::_0,
            true => WUAFA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUAFA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUAFA_A::_1
    }
}
#[doc = "Field `WUAFA` writer - Wake-Up Analog Filter Additional Selection"]
pub type WUAFA_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICWUR_SPEC, WUAFA_A, O>;
impl<'a, const O: u8> WUAFA_W<'a, O> {
    #[doc = "Do not add the Wake Up analog filter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUAFA_A::_0)
    }
    #[doc = "Add the Wake Up analog filter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUAFA_A::_1)
    }
}
#[doc = "Field `WUACK` reader - Asynchronous/Synchronous Operation State Flag"]
pub type WUACK_R = crate::BitReader<WUACK_A>;
#[doc = "Asynchronous/Synchronous Operation State Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUACK_A {
    #[doc = "0: State of synchronous operation"]
    _0 = 0,
    #[doc = "1: State of asynchronous operation"]
    _1 = 1,
}
impl From<WUACK_A> for bool {
    #[inline(always)]
    fn from(variant: WUACK_A) -> Self {
        variant as u8 != 0
    }
}
impl WUACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUACK_A {
        match self.bits {
            false => WUACK_A::_0,
            true => WUACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUACK_A::_1
    }
}
#[doc = "Field `WUACK` writer - Asynchronous/Synchronous Operation State Flag"]
pub type WUACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICWUR_SPEC, WUACK_A, O>;
impl<'a, const O: u8> WUACK_W<'a, O> {
    #[doc = "State of synchronous operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUACK_A::_0)
    }
    #[doc = "State of asynchronous operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUACK_A::_1)
    }
}
#[doc = "Field `WUF` reader - Wake-Up Event Occurrence Flag"]
pub type WUF_R = crate::BitReader<WUF_A>;
#[doc = "Wake-Up Event Occurrence Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF_A {
    #[doc = "0: Wake-Up event does not occur"]
    _0 = 0,
    #[doc = "1: Wake-Up event occur."]
    _1 = 1,
}
impl From<WUF_A> for bool {
    #[inline(always)]
    fn from(variant: WUF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF_A {
        match self.bits {
            false => WUF_A::_0,
            true => WUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF_A::_1
    }
}
#[doc = "Field `WUF` writer - Wake-Up Event Occurrence Flag"]
pub type WUF_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICWUR_SPEC, WUF_A, O>;
impl<'a, const O: u8> WUF_W<'a, O> {
    #[doc = "Wake-Up event does not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF_A::_0)
    }
    #[doc = "Wake-Up event occur."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF_A::_1)
    }
}
#[doc = "Field `WUIE` reader - Wake Up Interrupt Request Enable"]
pub type WUIE_R = crate::BitReader<WUIE_A>;
#[doc = "Wake Up Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUIE_A {
    #[doc = "0: Wake Up Interrupt Request (WUI) is disabled."]
    _0 = 0,
    #[doc = "1: Wake Up Interrupt Request (WUI) is enabled."]
    _1 = 1,
}
impl From<WUIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUIE_A {
        match self.bits {
            false => WUIE_A::_0,
            true => WUIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUIE_A::_1
    }
}
#[doc = "Field `WUIE` writer - Wake Up Interrupt Request Enable"]
pub type WUIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICWUR_SPEC, WUIE_A, O>;
impl<'a, const O: u8> WUIE_W<'a, O> {
    #[doc = "Wake Up Interrupt Request (WUI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUIE_A::_0)
    }
    #[doc = "Wake Up Interrupt Request (WUI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUIE_A::_1)
    }
}
#[doc = "Field `WUE` reader - Wake Up function Enable"]
pub type WUE_R = crate::BitReader<WUE_A>;
#[doc = "Wake Up function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUE_A {
    #[doc = "0: Wake-up function is disabled"]
    _0 = 0,
    #[doc = "1: Wake-up function is enabled."]
    _1 = 1,
}
impl From<WUE_A> for bool {
    #[inline(always)]
    fn from(variant: WUE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUE_A {
        match self.bits {
            false => WUE_A::_0,
            true => WUE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUE_A::_1
    }
}
#[doc = "Field `WUE` writer - Wake Up function Enable"]
pub type WUE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICWUR_SPEC, WUE_A, O>;
impl<'a, const O: u8> WUE_W<'a, O> {
    #[doc = "Wake-up function is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUE_A::_0)
    }
    #[doc = "Wake-up function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up Analog Filter Additional Selection"]
    #[inline(always)]
    pub fn wuafa(&self) -> WUAFA_R {
        WUAFA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous/Synchronous Operation State Flag"]
    #[inline(always)]
    pub fn wuack(&self) -> WUACK_R {
        WUACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake-Up Event Occurrence Flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake Up Interrupt Request Enable"]
    #[inline(always)]
    pub fn wuie(&self) -> WUIE_R {
        WUIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake Up function Enable"]
    #[inline(always)]
    pub fn wue(&self) -> WUE_R {
        WUE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Analog Filter Additional Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wuafa(&mut self) -> WUAFA_W<0> {
        WUAFA_W::new(self)
    }
    #[doc = "Bit 4 - Asynchronous/Synchronous Operation State Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wuack(&mut self) -> WUACK_W<4> {
        WUACK_W::new(self)
    }
    #[doc = "Bit 5 - Wake-Up Event Occurrence Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wuf(&mut self) -> WUF_W<5> {
        WUF_W::new(self)
    }
    #[doc = "Bit 6 - Wake Up Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wuie(&mut self) -> WUIE_W<6> {
        WUIE_W::new(self)
    }
    #[doc = "Bit 7 - Wake Up function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wue(&mut self) -> WUE_W<7> {
        WUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Wake Up Unit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icwur](index.html) module"]
pub struct ICWUR_SPEC;
impl crate::RegisterSpec for ICWUR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icwur::R](R) reader structure"]
impl crate::Readable for ICWUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icwur::W](W) writer structure"]
impl crate::Writable for ICWUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICWUR to value 0x10"]
impl crate::Resettable for ICWUR_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
