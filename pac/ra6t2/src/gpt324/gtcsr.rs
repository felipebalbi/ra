#[doc = "Register `GTCSR` reader"]
pub struct R(crate::R<GTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCSR` writer"]
pub struct W(crate::W<GTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCSR_SPEC>;
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
impl From<crate::W<GTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGAR_R = crate::BitReader<CSGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGAR_A {
    #[doc = "0: Counter clear disabled on the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<CSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSGTRGAR_A {
        match self.bits {
            false => CSGTRGAR_A::_0,
            true => CSGTRGAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGAR_A::_1
    }
}
#[doc = "Field `CSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSGTRGAR_A, O>;
impl<'a, const O: u8> CSGTRGAR_W<'a, O> {
    #[doc = "Counter clear disabled on the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSGTRGAR_A::_0)
    }
    #[doc = "Counter clear enabled on the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSGTRGAR_A::_1)
    }
}
#[doc = "Field `CSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGAF_R = crate::BitReader<CSGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGAF_A {
    #[doc = "0: Counter clear disabled on the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<CSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSGTRGAF_A {
        match self.bits {
            false => CSGTRGAF_A::_0,
            true => CSGTRGAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGAF_A::_1
    }
}
#[doc = "Field `CSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSGTRGAF_A, O>;
impl<'a, const O: u8> CSGTRGAF_W<'a, O> {
    #[doc = "Counter clear disabled on the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSGTRGAF_A::_0)
    }
    #[doc = "Counter clear enabled on the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSGTRGAF_A::_1)
    }
}
#[doc = "Field `CSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGBR_R = crate::BitReader<CSGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGBR_A {
    #[doc = "0: Disable counter clear on the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Enable counter clear on the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<CSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSGTRGBR_A {
        match self.bits {
            false => CSGTRGBR_A::_0,
            true => CSGTRGBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGBR_A::_1
    }
}
#[doc = "Field `CSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSGTRGBR_A, O>;
impl<'a, const O: u8> CSGTRGBR_W<'a, O> {
    #[doc = "Disable counter clear on the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSGTRGBR_A::_0)
    }
    #[doc = "Enable counter clear on the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSGTRGBR_A::_1)
    }
}
#[doc = "Field `CSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGBF_R = crate::BitReader<CSGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGBF_A {
    #[doc = "0: Counter clear disabled on the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<CSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSGTRGBF_A {
        match self.bits {
            false => CSGTRGBF_A::_0,
            true => CSGTRGBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGBF_A::_1
    }
}
#[doc = "Field `CSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSGTRGBF_A, O>;
impl<'a, const O: u8> CSGTRGBF_W<'a, O> {
    #[doc = "Counter clear disabled on the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSGTRGBF_A::_0)
    }
    #[doc = "Counter clear enabled on the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSGTRGBF_A::_1)
    }
}
#[doc = "Field `CSGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGCR_R = crate::BitReader<CSGTRGCR_A>;
#[doc = "GTETRGC Pin Rising Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGCR_A {
    #[doc = "0: Disable counter clear on the rising edge of GTETRGC input"]
    _0 = 0,
    #[doc = "1: Enable counter clear on the rising edge of GTETRGC input"]
    _1 = 1,
}
impl From<CSGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSGTRGCR_A {
        match self.bits {
            false => CSGTRGCR_A::_0,
            true => CSGTRGCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGCR_A::_1
    }
}
#[doc = "Field `CSGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSGTRGCR_A, O>;
impl<'a, const O: u8> CSGTRGCR_W<'a, O> {
    #[doc = "Disable counter clear on the rising edge of GTETRGC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSGTRGCR_A::_0)
    }
    #[doc = "Enable counter clear on the rising edge of GTETRGC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSGTRGCR_A::_1)
    }
}
#[doc = "Field `CSGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGCF_R = crate::BitReader<CSGTRGCF_A>;
#[doc = "GTETRGC Pin Falling Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGCF_A {
    #[doc = "0: Counter clear disabled on the falling edge of GTETRGC input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the falling edge of GTETRGC input"]
    _1 = 1,
}
impl From<CSGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSGTRGCF_A {
        match self.bits {
            false => CSGTRGCF_A::_0,
            true => CSGTRGCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGCF_A::_1
    }
}
#[doc = "Field `CSGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSGTRGCF_A, O>;
impl<'a, const O: u8> CSGTRGCF_W<'a, O> {
    #[doc = "Counter clear disabled on the falling edge of GTETRGC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSGTRGCF_A::_0)
    }
    #[doc = "Counter clear enabled on the falling edge of GTETRGC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSGTRGCF_A::_1)
    }
}
#[doc = "Field `CSGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGDR_R = crate::BitReader<CSGTRGDR_A>;
#[doc = "GTETRGD Pin Rising Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGDR_A {
    #[doc = "0: Disable counter clear on the rising edge of GTETRGD input"]
    _0 = 0,
    #[doc = "1: Enable counter clear on the rising edge of GTETRGD input"]
    _1 = 1,
}
impl From<CSGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSGTRGDR_A {
        match self.bits {
            false => CSGTRGDR_A::_0,
            true => CSGTRGDR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGDR_A::_1
    }
}
#[doc = "Field `CSGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSGTRGDR_A, O>;
impl<'a, const O: u8> CSGTRGDR_W<'a, O> {
    #[doc = "Disable counter clear on the rising edge of GTETRGD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSGTRGDR_A::_0)
    }
    #[doc = "Enable counter clear on the rising edge of GTETRGD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSGTRGDR_A::_1)
    }
}
#[doc = "Field `CSGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGDF_R = crate::BitReader<CSGTRGDF_A>;
#[doc = "GTETRGD Pin Falling Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGDF_A {
    #[doc = "0: Counter clear disabled on the falling edge of GTETRGD input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the falling edge of GTETRGD input"]
    _1 = 1,
}
impl From<CSGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSGTRGDF_A {
        match self.bits {
            false => CSGTRGDF_A::_0,
            true => CSGTRGDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGDF_A::_1
    }
}
#[doc = "Field `CSGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSGTRGDF_A, O>;
impl<'a, const O: u8> CSGTRGDF_W<'a, O> {
    #[doc = "Counter clear disabled on the falling edge of GTETRGD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSGTRGDF_A::_0)
    }
    #[doc = "Counter clear enabled on the falling edge of GTETRGD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSGTRGDF_A::_1)
    }
}
#[doc = "Field `CSCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Clear Enable"]
pub type CSCARBL_R = crate::BitReader<CSCARBL_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCARBL_A {
    #[doc = "0: Counter clear disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    _1 = 1,
}
impl From<CSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCARBL_A {
        match self.bits {
            false => CSCARBL_A::_0,
            true => CSCARBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCARBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCARBL_A::_1
    }
}
#[doc = "Field `CSCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Clear Enable"]
pub type CSCARBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSCARBL_A, O>;
impl<'a, const O: u8> CSCARBL_W<'a, O> {
    #[doc = "Counter clear disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCARBL_A::_0)
    }
    #[doc = "Counter clear enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCARBL_A::_1)
    }
}
#[doc = "Field `CSCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Clear Enable"]
pub type CSCARBH_R = crate::BitReader<CSCARBH_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCARBH_A {
    #[doc = "0: Counter clear disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    _1 = 1,
}
impl From<CSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCARBH_A {
        match self.bits {
            false => CSCARBH_A::_0,
            true => CSCARBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCARBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCARBH_A::_1
    }
}
#[doc = "Field `CSCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Clear Enable"]
pub type CSCARBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSCARBH_A, O>;
impl<'a, const O: u8> CSCARBH_W<'a, O> {
    #[doc = "Counter clear disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCARBH_A::_0)
    }
    #[doc = "Counter clear enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCARBH_A::_1)
    }
}
#[doc = "Field `CSCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Clear Enable"]
pub type CSCAFBL_R = crate::BitReader<CSCAFBL_A>;
#[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCAFBL_A {
    #[doc = "0: Counter clear disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    _1 = 1,
}
impl From<CSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCAFBL_A {
        match self.bits {
            false => CSCAFBL_A::_0,
            true => CSCAFBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCAFBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCAFBL_A::_1
    }
}
#[doc = "Field `CSCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Clear Enable"]
pub type CSCAFBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSCAFBL_A, O>;
impl<'a, const O: u8> CSCAFBL_W<'a, O> {
    #[doc = "Counter clear disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCAFBL_A::_0)
    }
    #[doc = "Counter clear enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCAFBL_A::_1)
    }
}
#[doc = "Field `CSCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Clear Enable"]
pub type CSCAFBH_R = crate::BitReader<CSCAFBH_A>;
#[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCAFBH_A {
    #[doc = "0: Counter clear disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    _1 = 1,
}
impl From<CSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCAFBH_A {
        match self.bits {
            false => CSCAFBH_A::_0,
            true => CSCAFBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCAFBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCAFBH_A::_1
    }
}
#[doc = "Field `CSCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Clear Enable"]
pub type CSCAFBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSCAFBH_A, O>;
impl<'a, const O: u8> CSCAFBH_W<'a, O> {
    #[doc = "Counter clear disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCAFBH_A::_0)
    }
    #[doc = "Counter clear enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCAFBH_A::_1)
    }
}
#[doc = "Field `CSCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Clear Enable"]
pub type CSCBRAL_R = crate::BitReader<CSCBRAL_A>;
#[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBRAL_A {
    #[doc = "0: Counter clear disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    _1 = 1,
}
impl From<CSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCBRAL_A {
        match self.bits {
            false => CSCBRAL_A::_0,
            true => CSCBRAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBRAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBRAL_A::_1
    }
}
#[doc = "Field `CSCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Clear Enable"]
pub type CSCBRAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSCBRAL_A, O>;
impl<'a, const O: u8> CSCBRAL_W<'a, O> {
    #[doc = "Counter clear disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCBRAL_A::_0)
    }
    #[doc = "Counter clear enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCBRAL_A::_1)
    }
}
#[doc = "Field `CSCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Clear Enable"]
pub type CSCBRAH_R = crate::BitReader<CSCBRAH_A>;
#[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBRAH_A {
    #[doc = "0: Counter clear disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    _1 = 1,
}
impl From<CSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCBRAH_A {
        match self.bits {
            false => CSCBRAH_A::_0,
            true => CSCBRAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBRAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBRAH_A::_1
    }
}
#[doc = "Field `CSCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Clear Enable"]
pub type CSCBRAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSCBRAH_A, O>;
impl<'a, const O: u8> CSCBRAH_W<'a, O> {
    #[doc = "Counter clear disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCBRAH_A::_0)
    }
    #[doc = "Counter clear enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCBRAH_A::_1)
    }
}
#[doc = "Field `CSCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Clear Enable"]
pub type CSCBFAL_R = crate::BitReader<CSCBFAL_A>;
#[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBFAL_A {
    #[doc = "0: Counter clear disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    _1 = 1,
}
impl From<CSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCBFAL_A {
        match self.bits {
            false => CSCBFAL_A::_0,
            true => CSCBFAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBFAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBFAL_A::_1
    }
}
#[doc = "Field `CSCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Clear Enable"]
pub type CSCBFAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSCBFAL_A, O>;
impl<'a, const O: u8> CSCBFAL_W<'a, O> {
    #[doc = "Counter clear disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCBFAL_A::_0)
    }
    #[doc = "Counter clear enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCBFAL_A::_1)
    }
}
#[doc = "Field `CSCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Clear Enable"]
pub type CSCBFAH_R = crate::BitReader<CSCBFAH_A>;
#[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBFAH_A {
    #[doc = "0: Counter clear disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    _1 = 1,
}
impl From<CSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCBFAH_A {
        match self.bits {
            false => CSCBFAH_A::_0,
            true => CSCBFAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBFAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBFAH_A::_1
    }
}
#[doc = "Field `CSCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Clear Enable"]
pub type CSCBFAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSCBFAH_A, O>;
impl<'a, const O: u8> CSCBFAH_W<'a, O> {
    #[doc = "Counter clear disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCBFAH_A::_0)
    }
    #[doc = "Counter clear enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCBFAH_A::_1)
    }
}
#[doc = "Field `CSELCA` reader - ELC_GPTA Event Source Counter Clear Enable"]
pub type CSELCA_R = crate::BitReader<CSELCA_A>;
#[doc = "ELC_GPTA Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCA_A {
    #[doc = "0: Counter clear disabled at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the ELC_GPTA input"]
    _1 = 1,
}
impl From<CSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSELCA_A {
        match self.bits {
            false => CSELCA_A::_0,
            true => CSELCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCA_A::_1
    }
}
#[doc = "Field `CSELCA` writer - ELC_GPTA Event Source Counter Clear Enable"]
pub type CSELCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSELCA_A, O>;
impl<'a, const O: u8> CSELCA_W<'a, O> {
    #[doc = "Counter clear disabled at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSELCA_A::_0)
    }
    #[doc = "Counter clear enabled at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSELCA_A::_1)
    }
}
#[doc = "Field `CSELCB` reader - ELC_GPTB Event Source Counter Clear Enable"]
pub type CSELCB_R = crate::BitReader<CSELCB_A>;
#[doc = "ELC_GPTB Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCB_A {
    #[doc = "0: Counter clear disabled at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the ELC_GPTB input"]
    _1 = 1,
}
impl From<CSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSELCB_A {
        match self.bits {
            false => CSELCB_A::_0,
            true => CSELCB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCB_A::_1
    }
}
#[doc = "Field `CSELCB` writer - ELC_GPTB Event Source Counter Clear Enable"]
pub type CSELCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSELCB_A, O>;
impl<'a, const O: u8> CSELCB_W<'a, O> {
    #[doc = "Counter clear disabled at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSELCB_A::_0)
    }
    #[doc = "Counter clear enabled at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSELCB_A::_1)
    }
}
#[doc = "Field `CSELCC` reader - ELC_GPTC Event Source Counter Clear Enable"]
pub type CSELCC_R = crate::BitReader<CSELCC_A>;
#[doc = "ELC_GPTC Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCC_A {
    #[doc = "0: Counter clear disabled at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the ELC_GPTC input"]
    _1 = 1,
}
impl From<CSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSELCC_A {
        match self.bits {
            false => CSELCC_A::_0,
            true => CSELCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCC_A::_1
    }
}
#[doc = "Field `CSELCC` writer - ELC_GPTC Event Source Counter Clear Enable"]
pub type CSELCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSELCC_A, O>;
impl<'a, const O: u8> CSELCC_W<'a, O> {
    #[doc = "Counter clear disabled at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSELCC_A::_0)
    }
    #[doc = "Counter clear enabled at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSELCC_A::_1)
    }
}
#[doc = "Field `CSELCD` reader - ELC_GPTD Event Source Counter Clear Enable"]
pub type CSELCD_R = crate::BitReader<CSELCD_A>;
#[doc = "ELC_GPTD Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCD_A {
    #[doc = "0: Counter clear disabled at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the ELC_GPTD input"]
    _1 = 1,
}
impl From<CSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSELCD_A {
        match self.bits {
            false => CSELCD_A::_0,
            true => CSELCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCD_A::_1
    }
}
#[doc = "Field `CSELCD` writer - ELC_GPTD Event Source Counter Clear Enable"]
pub type CSELCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSELCD_A, O>;
impl<'a, const O: u8> CSELCD_W<'a, O> {
    #[doc = "Counter clear disabled at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSELCD_A::_0)
    }
    #[doc = "Counter clear enabled at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSELCD_A::_1)
    }
}
#[doc = "Field `CSELCE` reader - ELC_GPTE Event Source Counter Clear Enable"]
pub type CSELCE_R = crate::BitReader<CSELCE_A>;
#[doc = "ELC_GPTE Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCE_A {
    #[doc = "0: Counter clear disabled at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the ELC_GPTE input"]
    _1 = 1,
}
impl From<CSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSELCE_A {
        match self.bits {
            false => CSELCE_A::_0,
            true => CSELCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCE_A::_1
    }
}
#[doc = "Field `CSELCE` writer - ELC_GPTE Event Source Counter Clear Enable"]
pub type CSELCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSELCE_A, O>;
impl<'a, const O: u8> CSELCE_W<'a, O> {
    #[doc = "Counter clear disabled at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSELCE_A::_0)
    }
    #[doc = "Counter clear enabled at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSELCE_A::_1)
    }
}
#[doc = "Field `CSELCF` reader - ELC_GPTF Event Source Counter Clear Enable"]
pub type CSELCF_R = crate::BitReader<CSELCF_A>;
#[doc = "ELC_GPTF Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCF_A {
    #[doc = "0: Counter clear disabled at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the ELC_GPTF input"]
    _1 = 1,
}
impl From<CSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSELCF_A {
        match self.bits {
            false => CSELCF_A::_0,
            true => CSELCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCF_A::_1
    }
}
#[doc = "Field `CSELCF` writer - ELC_GPTF Event Source Counter Clear Enable"]
pub type CSELCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSELCF_A, O>;
impl<'a, const O: u8> CSELCF_W<'a, O> {
    #[doc = "Counter clear disabled at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSELCF_A::_0)
    }
    #[doc = "Counter clear enabled at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSELCF_A::_1)
    }
}
#[doc = "Field `CSELCG` reader - ELC_GPTG Event Source Counter Clear Enable"]
pub type CSELCG_R = crate::BitReader<CSELCG_A>;
#[doc = "ELC_GPTG Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCG_A {
    #[doc = "0: Counter clear disabled at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the ELC_GPTG input"]
    _1 = 1,
}
impl From<CSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCG_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSELCG_A {
        match self.bits {
            false => CSELCG_A::_0,
            true => CSELCG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCG_A::_1
    }
}
#[doc = "Field `CSELCG` writer - ELC_GPTG Event Source Counter Clear Enable"]
pub type CSELCG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSELCG_A, O>;
impl<'a, const O: u8> CSELCG_W<'a, O> {
    #[doc = "Counter clear disabled at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSELCG_A::_0)
    }
    #[doc = "Counter clear enabled at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSELCG_A::_1)
    }
}
#[doc = "Field `CSELCH` reader - ELC_GPTH Event Source Counter Clear Enable"]
pub type CSELCH_R = crate::BitReader<CSELCH_A>;
#[doc = "ELC_GPTH Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCH_A {
    #[doc = "0: Counter clear disabled at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the ELC_GPTH input"]
    _1 = 1,
}
impl From<CSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSELCH_A {
        match self.bits {
            false => CSELCH_A::_0,
            true => CSELCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCH_A::_1
    }
}
#[doc = "Field `CSELCH` writer - ELC_GPTH Event Source Counter Clear Enable"]
pub type CSELCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CSELCH_A, O>;
impl<'a, const O: u8> CSELCH_W<'a, O> {
    #[doc = "Counter clear disabled at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSELCH_A::_0)
    }
    #[doc = "Counter clear enabled at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSELCH_A::_1)
    }
}
#[doc = "Field `CSCMSC` reader - Compare Match/Input Capture/Synchronous counter clearing Source Counter Clear Enable"]
pub type CSCMSC_R = crate::FieldReader<u8, CSCMSC_A>;
#[doc = "Compare Match/Input Capture/Synchronous counter clearing Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSCMSC_A {
    #[doc = "0: Counter clear disabled by Compare match/ Input capture/ Synchronous counter clearing group"]
    _000 = 0,
    #[doc = "1: Counter clear enabled at the GTCCRA register compare match/ Input capture"]
    _001 = 1,
    #[doc = "2: Counter clear enabled at the GTCCRB register compare match/ Input capture"]
    _010 = 2,
    #[doc = "3: Counter clear enabled at the GTCCRC register compare match"]
    _011 = 3,
    #[doc = "4: Counter clear enabled at the GTCCRD register compare match"]
    _100 = 4,
    #[doc = "5: Counter clear enabled at the GTCCRE register compare match"]
    _101 = 5,
    #[doc = "6: Counter clear enabled at the GTCCRF register compare match"]
    _110 = 6,
    #[doc = "7: Counter clear enabled at the synchronous counter clearing group"]
    _111 = 7,
}
impl From<CSCMSC_A> for u8 {
    #[inline(always)]
    fn from(variant: CSCMSC_A) -> Self {
        variant as _
    }
}
impl CSCMSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCMSC_A {
        match self.bits {
            0 => CSCMSC_A::_000,
            1 => CSCMSC_A::_001,
            2 => CSCMSC_A::_010,
            3 => CSCMSC_A::_011,
            4 => CSCMSC_A::_100,
            5 => CSCMSC_A::_101,
            6 => CSCMSC_A::_110,
            7 => CSCMSC_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CSCMSC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CSCMSC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CSCMSC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CSCMSC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CSCMSC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CSCMSC_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CSCMSC_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CSCMSC_A::_111
    }
}
#[doc = "Field `CSCMSC` writer - Compare Match/Input Capture/Synchronous counter clearing Source Counter Clear Enable"]
pub type CSCMSC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTCSR_SPEC, u8, CSCMSC_A, 3, O>;
impl<'a, const O: u8> CSCMSC_W<'a, O> {
    #[doc = "Counter clear disabled by Compare match/ Input capture/ Synchronous counter clearing group"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CSCMSC_A::_000)
    }
    #[doc = "Counter clear enabled at the GTCCRA register compare match/ Input capture"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CSCMSC_A::_001)
    }
    #[doc = "Counter clear enabled at the GTCCRB register compare match/ Input capture"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CSCMSC_A::_010)
    }
    #[doc = "Counter clear enabled at the GTCCRC register compare match"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CSCMSC_A::_011)
    }
    #[doc = "Counter clear enabled at the GTCCRD register compare match"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CSCMSC_A::_100)
    }
    #[doc = "Counter clear enabled at the GTCCRE register compare match"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CSCMSC_A::_101)
    }
    #[doc = "Counter clear enabled at the GTCCRF register compare match"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CSCMSC_A::_110)
    }
    #[doc = "Counter clear enabled at the synchronous counter clearing group"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CSCMSC_A::_111)
    }
}
#[doc = "Field `CP1CCE` reader - Complementary PWM mode1 Crest Source Counter Clear Enable"]
pub type CP1CCE_R = crate::BitReader<CP1CCE_A>;
#[doc = "Complementary PWM mode1 Crest Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CP1CCE_A {
    #[doc = "0: Counter clear disabled at the crest of complementary PWM mode1"]
    _0 = 0,
    #[doc = "1: Counter clear enabled at the crest of complementary PWM mode1"]
    _1 = 1,
}
impl From<CP1CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CP1CCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CP1CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CP1CCE_A {
        match self.bits {
            false => CP1CCE_A::_0,
            true => CP1CCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CP1CCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CP1CCE_A::_1
    }
}
#[doc = "Field `CP1CCE` writer - Complementary PWM mode1 Crest Source Counter Clear Enable"]
pub type CP1CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CP1CCE_A, O>;
impl<'a, const O: u8> CP1CCE_W<'a, O> {
    #[doc = "Counter clear disabled at the crest of complementary PWM mode1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CP1CCE_A::_0)
    }
    #[doc = "Counter clear enabled at the crest of complementary PWM mode1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CP1CCE_A::_1)
    }
}
#[doc = "Field `CCLR` reader - Software Source Counter Clear Enable"]
pub type CCLR_R = crate::BitReader<CCLR_A>;
#[doc = "Software Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR_A {
    #[doc = "0: Counter clear disabled by the GTCLR register"]
    _0 = 0,
    #[doc = "1: Counter clear enabled by the GTCLR register"]
    _1 = 1,
}
impl From<CCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLR_A {
        match self.bits {
            false => CCLR_A::_0,
            true => CCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCLR_A::_1
    }
}
#[doc = "Field `CCLR` writer - Software Source Counter Clear Enable"]
pub type CCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCSR_SPEC, CCLR_A, O>;
impl<'a, const O: u8> CCLR_W<'a, O> {
    #[doc = "Counter clear disabled by the GTCLR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR_A::_0)
    }
    #[doc = "Counter clear enabled by the GTCLR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgar(&self) -> CSGTRGAR_R {
        CSGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgaf(&self) -> CSGTRGAF_R {
        CSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbr(&self) -> CSGTRGBR_R {
        CSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbf(&self) -> CSGTRGBF_R {
        CSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GTETRGC Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgcr(&self) -> CSGTRGCR_R {
        CSGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GTETRGC Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgcf(&self) -> CSGTRGCF_R {
        CSGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GTETRGD Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgdr(&self) -> CSGTRGDR_R {
        CSGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTETRGD Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgdf(&self) -> CSGTRGDF_R {
        CSGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbl(&self) -> CSCARBL_R {
        CSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbh(&self) -> CSCARBH_R {
        CSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbl(&self) -> CSCAFBL_R {
        CSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbh(&self) -> CSCAFBH_R {
        CSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbral(&self) -> CSCBRAL_R {
        CSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbrah(&self) -> CSCBRAH_R {
        CSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfal(&self) -> CSCBFAL_R {
        CSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfah(&self) -> CSCBFAH_R {
        CSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselca(&self) -> CSELCA_R {
        CSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcb(&self) -> CSELCB_R {
        CSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcc(&self) -> CSELCC_R {
        CSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcd(&self) -> CSELCD_R {
        CSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselce(&self) -> CSELCE_R {
        CSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcf(&self) -> CSELCF_R {
        CSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcg(&self) -> CSELCG_R {
        CSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselch(&self) -> CSELCH_R {
        CSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Compare Match/Input Capture/Synchronous counter clearing Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscmsc(&self) -> CSCMSC_R {
        CSCMSC_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Complementary PWM mode1 Crest Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cp1cce(&self) -> CP1CCE_R {
        CP1CCE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Software Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cclr(&self) -> CCLR_R {
        CCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgar(&mut self) -> CSGTRGAR_W<0> {
        CSGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgaf(&mut self) -> CSGTRGAF_W<1> {
        CSGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgbr(&mut self) -> CSGTRGBR_W<2> {
        CSGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgbf(&mut self) -> CSGTRGBF_W<3> {
        CSGTRGBF_W::new(self)
    }
    #[doc = "Bit 4 - GTETRGC Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgcr(&mut self) -> CSGTRGCR_W<4> {
        CSGTRGCR_W::new(self)
    }
    #[doc = "Bit 5 - GTETRGC Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgcf(&mut self) -> CSGTRGCF_W<5> {
        CSGTRGCF_W::new(self)
    }
    #[doc = "Bit 6 - GTETRGD Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgdr(&mut self) -> CSGTRGDR_W<6> {
        CSGTRGDR_W::new(self)
    }
    #[doc = "Bit 7 - GTETRGD Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgdf(&mut self) -> CSGTRGDF_W<7> {
        CSGTRGDF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscarbl(&mut self) -> CSCARBL_W<8> {
        CSCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscarbh(&mut self) -> CSCARBH_W<9> {
        CSCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscafbl(&mut self) -> CSCAFBL_W<10> {
        CSCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscafbh(&mut self) -> CSCAFBH_W<11> {
        CSCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscbral(&mut self) -> CSCBRAL_W<12> {
        CSCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscbrah(&mut self) -> CSCBRAH_W<13> {
        CSCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscbfal(&mut self) -> CSCBFAL_W<14> {
        CSCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscbfah(&mut self) -> CSCBFAH_W<15> {
        CSCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselca(&mut self) -> CSELCA_W<16> {
        CSELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcb(&mut self) -> CSELCB_W<17> {
        CSELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcc(&mut self) -> CSELCC_W<18> {
        CSELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcd(&mut self) -> CSELCD_W<19> {
        CSELCD_W::new(self)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselce(&mut self) -> CSELCE_W<20> {
        CSELCE_W::new(self)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcf(&mut self) -> CSELCF_W<21> {
        CSELCF_W::new(self)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcg(&mut self) -> CSELCG_W<22> {
        CSELCG_W::new(self)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselch(&mut self) -> CSELCH_W<23> {
        CSELCH_W::new(self)
    }
    #[doc = "Bits 24:26 - Compare Match/Input Capture/Synchronous counter clearing Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscmsc(&mut self) -> CSCMSC_W<24> {
        CSCMSC_W::new(self)
    }
    #[doc = "Bit 27 - Complementary PWM mode1 Crest Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cp1cce(&mut self) -> CP1CCE_W<27> {
        CP1CCE_W::new(self)
    }
    #[doc = "Bit 31 - Software Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cclr(&mut self) -> CCLR_W<31> {
        CCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Clear Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtcsr](index.html) module"]
pub struct GTCSR_SPEC;
impl crate::RegisterSpec for GTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtcsr::R](R) reader structure"]
impl crate::Readable for GTCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtcsr::W](W) writer structure"]
impl crate::Writable for GTCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCSR to value 0"]
impl crate::Resettable for GTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
