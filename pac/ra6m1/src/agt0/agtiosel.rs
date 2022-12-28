#[doc = "Register `AGTIOSEL` reader"]
pub struct R(crate::R<AGTIOSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTIOSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTIOSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTIOSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTIOSEL` writer"]
pub struct W(crate::W<AGTIOSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTIOSEL_SPEC>;
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
impl From<crate::W<AGTIOSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTIOSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - AGTIO pin select"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "AGTIO pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: AGTIO_A can not be used as AGTIO input pin in deep software standby mode"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: AGTIO_B can be used as AGTIO input pin in deep software standby mode. AGTIO_B is input only. It is not possible to output."]
    _10 = 2,
    #[doc = "3: AGTIO_C can be used as AGTIO input pin in deep software standby mode. AGTIO_C is input only. It is not possible to output."]
    _11 = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::_00,
            1 => SEL_A::_01,
            2 => SEL_A::_10,
            3 => SEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEL_A::_11
    }
}
#[doc = "Field `SEL` writer - AGTIO pin select"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, AGTIOSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "AGTIO_A can not be used as AGTIO input pin in deep software standby mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SEL_A::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SEL_A::_01)
    }
    #[doc = "AGTIO_B can be used as AGTIO input pin in deep software standby mode. AGTIO_B is input only. It is not possible to output."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SEL_A::_10)
    }
    #[doc = "AGTIO_C can be used as AGTIO input pin in deep software standby mode. AGTIO_C is input only. It is not possible to output."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SEL_A::_11)
    }
}
#[doc = "Field `TIES` reader - AGTIO input enable"]
pub type TIES_R = crate::BitReader<TIES_A>;
#[doc = "AGTIO input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIES_A {
    #[doc = "0: external event input disable during software standby mode"]
    _0 = 0,
    #[doc = "1: external event input enable during software standby mode"]
    _1 = 1,
}
impl From<TIES_A> for bool {
    #[inline(always)]
    fn from(variant: TIES_A) -> Self {
        variant as u8 != 0
    }
}
impl TIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIES_A {
        match self.bits {
            false => TIES_A::_0,
            true => TIES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIES_A::_1
    }
}
#[doc = "Field `TIES` writer - AGTIO input enable"]
pub type TIES_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTIOSEL_SPEC, TIES_A, O>;
impl<'a, const O: u8> TIES_W<'a, O> {
    #[doc = "external event input disable during software standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIES_A::_0)
    }
    #[doc = "external event input enable during software standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIES_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - AGTIO pin select"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(self.bits & 3)
    }
    #[doc = "Bit 4 - AGTIO input enable"]
    #[inline(always)]
    pub fn ties(&self) -> TIES_R {
        TIES_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AGTIO pin select"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 4 - AGTIO input enable"]
    #[inline(always)]
    #[must_use]
    pub fn ties(&mut self) -> TIES_W<4> {
        TIES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT Pin Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtiosel](index.html) module"]
pub struct AGTIOSEL_SPEC;
impl crate::RegisterSpec for AGTIOSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [agtiosel::R](R) reader structure"]
impl crate::Readable for AGTIOSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtiosel::W](W) writer structure"]
impl crate::Writable for AGTIOSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTIOSEL to value 0"]
impl crate::Resettable for AGTIOSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
