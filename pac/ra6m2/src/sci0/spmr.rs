#[doc = "Register `SPMR` reader"]
pub struct R(crate::R<SPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPMR` writer"]
pub struct W(crate::W<SPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPMR_SPEC>;
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
impl From<crate::W<SPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSE` reader - SSn# Pin Function Enable"]
pub type SSE_R = crate::BitReader<SSE_A>;
#[doc = "SSn# Pin Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSE_A {
    #[doc = "0: SSn# pin function is disabled."]
    _0 = 0,
    #[doc = "1: SSn# pin function is enabled."]
    _1 = 1,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSE_A {
        match self.bits {
            false => SSE_A::_0,
            true => SSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSE_A::_1
    }
}
#[doc = "Field `SSE` writer - SSn# Pin Function Enable"]
pub type SSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMR_SPEC, SSE_A, O>;
impl<'a, const O: u8> SSE_W<'a, O> {
    #[doc = "SSn# pin function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSE_A::_0)
    }
    #[doc = "SSn# pin function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSE_A::_1)
    }
}
#[doc = "Field `CTSE` reader - CTS Enable"]
pub type CTSE_R = crate::BitReader<CTSE_A>;
#[doc = "CTS Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE_A {
    #[doc = "0: CTS function is disabled (RTS output function is enabled)."]
    _0 = 0,
    #[doc = "1: CTS function is enabled."]
    _1 = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::_0,
            true => CTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSE_A::_1
    }
}
#[doc = "Field `CTSE` writer - CTS Enable"]
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMR_SPEC, CTSE_A, O>;
impl<'a, const O: u8> CTSE_W<'a, O> {
    #[doc = "CTS function is disabled (RTS output function is enabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSE_A::_0)
    }
    #[doc = "CTS function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSE_A::_1)
    }
}
#[doc = "Field `MSS` reader - Master or slave mode selection"]
pub type MSS_R = crate::BitReader<MSS_A>;
#[doc = "Master or slave mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSS_A {
    #[doc = "0: Transmission is through the TXDn pin and reception is through the RXDn pin (master mode)."]
    _0 = 0,
    #[doc = "1: Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode)."]
    _1 = 1,
}
impl From<MSS_A> for bool {
    #[inline(always)]
    fn from(variant: MSS_A) -> Self {
        variant as u8 != 0
    }
}
impl MSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSS_A {
        match self.bits {
            false => MSS_A::_0,
            true => MSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSS_A::_1
    }
}
#[doc = "Field `MSS` writer - Master or slave mode selection"]
pub type MSS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMR_SPEC, MSS_A, O>;
impl<'a, const O: u8> MSS_W<'a, O> {
    #[doc = "Transmission is through the TXDn pin and reception is through the RXDn pin (master mode)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSS_A::_0)
    }
    #[doc = "Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSS_A::_1)
    }
}
#[doc = "Field `MFF` reader - Mode Fault Flag\n\nThe field is **modified** in some way after a read operation."]
pub type MFF_R = crate::BitReader<MFF_A>;
#[doc = "Mode Fault Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MFF_A {
    #[doc = "0: No mode fault error"]
    _0 = 0,
    #[doc = "1: Mode fault error"]
    _1 = 1,
}
impl From<MFF_A> for bool {
    #[inline(always)]
    fn from(variant: MFF_A) -> Self {
        variant as u8 != 0
    }
}
impl MFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MFF_A {
        match self.bits {
            false => MFF_A::_0,
            true => MFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MFF_A::_1
    }
}
#[doc = "Field `MFF` writer - Mode Fault Flag"]
pub type MFF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SPMR_SPEC, MFF_A, O>;
impl<'a, const O: u8> MFF_W<'a, O> {
    #[doc = "No mode fault error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MFF_A::_0)
    }
    #[doc = "Mode fault error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MFF_A::_1)
    }
}
#[doc = "Field `CKPOL` reader - Clock Polarity Select"]
pub type CKPOL_R = crate::BitReader<CKPOL_A>;
#[doc = "Clock Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL_A {
    #[doc = "0: Clock polarity is not inverted."]
    _0 = 0,
    #[doc = "1: Clock polarity is inverted"]
    _1 = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::_0,
            true => CKPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKPOL_A::_1
    }
}
#[doc = "Field `CKPOL` writer - Clock Polarity Select"]
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMR_SPEC, CKPOL_A, O>;
impl<'a, const O: u8> CKPOL_W<'a, O> {
    #[doc = "Clock polarity is not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CKPOL_A::_0)
    }
    #[doc = "Clock polarity is inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CKPOL_A::_1)
    }
}
#[doc = "Field `CKPH` reader - Clock Phase Select"]
pub type CKPH_R = crate::BitReader<CKPH_A>;
#[doc = "Clock Phase Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPH_A {
    #[doc = "0: Clock is not delayed."]
    _0 = 0,
    #[doc = "1: Clock is delayed."]
    _1 = 1,
}
impl From<CKPH_A> for bool {
    #[inline(always)]
    fn from(variant: CKPH_A) -> Self {
        variant as u8 != 0
    }
}
impl CKPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPH_A {
        match self.bits {
            false => CKPH_A::_0,
            true => CKPH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKPH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKPH_A::_1
    }
}
#[doc = "Field `CKPH` writer - Clock Phase Select"]
pub type CKPH_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMR_SPEC, CKPH_A, O>;
impl<'a, const O: u8> CKPH_W<'a, O> {
    #[doc = "Clock is not delayed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CKPH_A::_0)
    }
    #[doc = "Clock is delayed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CKPH_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SSn# Pin Function Enable"]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS Enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master or slave mode selection"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Flag"]
    #[inline(always)]
    pub fn mff(&self) -> MFF_R {
        MFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Polarity Select"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Phase Select"]
    #[inline(always)]
    pub fn ckph(&self) -> CKPH_R {
        CKPH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSn# Pin Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SSE_W<0> {
        SSE_W::new(self)
    }
    #[doc = "Bit 1 - CTS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<1> {
        CTSE_W::new(self)
    }
    #[doc = "Bit 2 - Master or slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mss(&mut self) -> MSS_W<2> {
        MSS_W::new(self)
    }
    #[doc = "Bit 4 - Mode Fault Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mff(&mut self) -> MFF_W<4> {
        MFF_W::new(self)
    }
    #[doc = "Bit 6 - Clock Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<6> {
        CKPOL_W::new(self)
    }
    #[doc = "Bit 7 - Clock Phase Select"]
    #[inline(always)]
    #[must_use]
    pub fn ckph(&mut self) -> CKPH_W<7> {
        CKPH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spmr](index.html) module"]
pub struct SPMR_SPEC;
impl crate::RegisterSpec for SPMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spmr::R](R) reader structure"]
impl crate::Readable for SPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spmr::W](W) writer structure"]
impl crate::Writable for SPMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x10;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPMR to value 0"]
impl crate::Resettable for SPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
