#[doc = "Register `SD_INFO1` reader"]
pub struct R(crate::R<SD_INFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_INFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_INFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_INFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_INFO1` writer"]
pub struct W(crate::W<SD_INFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_INFO1_SPEC>;
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
impl From<crate::W<SD_INFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_INFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSPEND` reader - Response End Detection"]
pub type RSPEND_R = crate::BitReader<RSPEND_A>;
#[doc = "Response End Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPEND_A {
    #[doc = "0: Response end is not detected"]
    _0 = 0,
    #[doc = "1: Response end is detected"]
    _1 = 1,
}
impl From<RSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: RSPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPEND_A {
        match self.bits {
            false => RSPEND_A::_0,
            true => RSPEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPEND_A::_1
    }
}
#[doc = "Field `RSPEND` writer - Response End Detection"]
pub type RSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO1_SPEC, RSPEND_A, O>;
impl<'a, const O: u8> RSPEND_W<'a, O> {
    #[doc = "Response end is not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPEND_A::_0)
    }
    #[doc = "Response end is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPEND_A::_1)
    }
}
#[doc = "Field `ACEND` reader - Access End\n\nThe field is **modified** in some way after a read operation."]
pub type ACEND_R = crate::BitReader<ACEND_A>;
#[doc = "Access End\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACEND_A {
    #[doc = "0: Access end is not detected"]
    _0 = 0,
    #[doc = "1: Access end is detected"]
    _1 = 1,
}
impl From<ACEND_A> for bool {
    #[inline(always)]
    fn from(variant: ACEND_A) -> Self {
        variant as u8 != 0
    }
}
impl ACEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACEND_A {
        match self.bits {
            false => ACEND_A::_0,
            true => ACEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACEND_A::_1
    }
}
#[doc = "Field `ACEND` writer - Access End"]
pub type ACEND_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO1_SPEC, ACEND_A, O>;
impl<'a, const O: u8> ACEND_W<'a, O> {
    #[doc = "Access end is not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACEND_A::_0)
    }
    #[doc = "Access end is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACEND_A::_1)
    }
}
#[doc = "Field `SDCDRM` reader - SDnCD Card Removal\n\nThe field is **modified** in some way after a read operation."]
pub type SDCDRM_R = crate::BitReader<SDCDRM_A>;
#[doc = "SDnCD Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDRM_A {
    #[doc = "0: Card removal not detected"]
    _0 = 0,
    #[doc = "1: Card removal detected"]
    _1 = 1,
}
impl From<SDCDRM_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDRM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDCDRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCDRM_A {
        match self.bits {
            false => SDCDRM_A::_0,
            true => SDCDRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDRM_A::_1
    }
}
#[doc = "Field `SDCDRM` writer - SDnCD Card Removal"]
pub type SDCDRM_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO1_SPEC, SDCDRM_A, O>;
impl<'a, const O: u8> SDCDRM_W<'a, O> {
    #[doc = "Card removal not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDCDRM_A::_0)
    }
    #[doc = "Card removal detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCDRM_A::_1)
    }
}
#[doc = "Field `SDCDIN` reader - SDnCD Card Insertion\n\nThe field is **modified** in some way after a read operation."]
pub type SDCDIN_R = crate::BitReader<SDCDIN_A>;
#[doc = "SDnCD Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDIN_A {
    #[doc = "0: Card insertion not detected"]
    _0 = 0,
    #[doc = "1: Card insertion detected"]
    _1 = 1,
}
impl From<SDCDIN_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDIN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDCDIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCDIN_A {
        match self.bits {
            false => SDCDIN_A::_0,
            true => SDCDIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDIN_A::_1
    }
}
#[doc = "Field `SDCDIN` writer - SDnCD Card Insertion"]
pub type SDCDIN_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO1_SPEC, SDCDIN_A, O>;
impl<'a, const O: u8> SDCDIN_W<'a, O> {
    #[doc = "Card insertion not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDCDIN_A::_0)
    }
    #[doc = "Card insertion detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCDIN_A::_1)
    }
}
#[doc = "Field `SDCDMON` reader - Indicates the SDnCD state"]
pub type SDCDMON_R = crate::BitReader<SDCDMON_A>;
#[doc = "Indicates the SDnCD state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDMON_A {
    #[doc = "0: Indicates that Mcycle has elapsed with SDnCD held 1.(Mcycle is set by bits 3 to 0 in SD_OPTION.)"]
    _0 = 0,
    #[doc = "1: Indicates that Mcycle has elapsed with SDnCD held 0. (Mcycle is set by bits 3 to 0 in SD_OPTION.)"]
    _1 = 1,
}
impl From<SDCDMON_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDMON_A) -> Self {
        variant as u8 != 0
    }
}
impl SDCDMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCDMON_A {
        match self.bits {
            false => SDCDMON_A::_0,
            true => SDCDMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDMON_A::_1
    }
}
#[doc = "Field `SDWPMON` reader - Indicates the SDnWP state"]
pub type SDWPMON_R = crate::BitReader<SDWPMON_A>;
#[doc = "Indicates the SDnWP state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDWPMON_A {
    #[doc = "0: SDnWP is set to 1."]
    _0 = 0,
    #[doc = "1: SDnWP is set to 0."]
    _1 = 1,
}
impl From<SDWPMON_A> for bool {
    #[inline(always)]
    fn from(variant: SDWPMON_A) -> Self {
        variant as u8 != 0
    }
}
impl SDWPMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDWPMON_A {
        match self.bits {
            false => SDWPMON_A::_0,
            true => SDWPMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDWPMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDWPMON_A::_1
    }
}
#[doc = "Field `SDD3RM` reader - SDnDAT3 Card Removal\n\nThe field is **modified** in some way after a read operation."]
pub type SDD3RM_R = crate::BitReader<SDD3RM_A>;
#[doc = "SDnDAT3 Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3RM_A {
    #[doc = "0: SD card removal not detected"]
    _0 = 0,
    #[doc = "1: SD card removal detected"]
    _1 = 1,
}
impl From<SDD3RM_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3RM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDD3RM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDD3RM_A {
        match self.bits {
            false => SDD3RM_A::_0,
            true => SDD3RM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3RM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3RM_A::_1
    }
}
#[doc = "Field `SDD3RM` writer - SDnDAT3 Card Removal"]
pub type SDD3RM_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO1_SPEC, SDD3RM_A, O>;
impl<'a, const O: u8> SDD3RM_W<'a, O> {
    #[doc = "SD card removal not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDD3RM_A::_0)
    }
    #[doc = "SD card removal detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDD3RM_A::_1)
    }
}
#[doc = "Field `SDD3IN` reader - SDnDAT3 Card Insertion\n\nThe field is **modified** in some way after a read operation."]
pub type SDD3IN_R = crate::BitReader<SDD3IN_A>;
#[doc = "SDnDAT3 Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3IN_A {
    #[doc = "0: SD card insertion not detected"]
    _0 = 0,
    #[doc = "1: SD card insertion detected"]
    _1 = 1,
}
impl From<SDD3IN_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3IN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDD3IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDD3IN_A {
        match self.bits {
            false => SDD3IN_A::_0,
            true => SDD3IN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3IN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3IN_A::_1
    }
}
#[doc = "Field `SDD3IN` writer - SDnDAT3 Card Insertion"]
pub type SDD3IN_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO1_SPEC, SDD3IN_A, O>;
impl<'a, const O: u8> SDD3IN_W<'a, O> {
    #[doc = "SD card insertion not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDD3IN_A::_0)
    }
    #[doc = "SD card insertion detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDD3IN_A::_1)
    }
}
#[doc = "Field `SDD3MON` reader - Inticates the SDnDAT3 State"]
pub type SDD3MON_R = crate::BitReader<SDD3MON_A>;
#[doc = "Inticates the SDnDAT3 State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3MON_A {
    #[doc = "0: SDnDAT3 is set to 0."]
    _0 = 0,
    #[doc = "1: SDnDAT3 is set to 1."]
    _1 = 1,
}
impl From<SDD3MON_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3MON_A) -> Self {
        variant as u8 != 0
    }
}
impl SDD3MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDD3MON_A {
        match self.bits {
            false => SDD3MON_A::_0,
            true => SDD3MON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3MON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3MON_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Response End Detection"]
    #[inline(always)]
    pub fn rspend(&self) -> RSPEND_R {
        RSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Access End"]
    #[inline(always)]
    pub fn acend(&self) -> ACEND_R {
        ACEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDnCD Card Removal"]
    #[inline(always)]
    pub fn sdcdrm(&self) -> SDCDRM_R {
        SDCDRM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDnCD Card Insertion"]
    #[inline(always)]
    pub fn sdcdin(&self) -> SDCDIN_R {
        SDCDIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates the SDnCD state"]
    #[inline(always)]
    pub fn sdcdmon(&self) -> SDCDMON_R {
        SDCDMON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the SDnWP state"]
    #[inline(always)]
    pub fn sdwpmon(&self) -> SDWPMON_R {
        SDWPMON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDnDAT3 Card Removal"]
    #[inline(always)]
    pub fn sdd3rm(&self) -> SDD3RM_R {
        SDD3RM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDnDAT3 Card Insertion"]
    #[inline(always)]
    pub fn sdd3in(&self) -> SDD3IN_R {
        SDD3IN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Inticates the SDnDAT3 State"]
    #[inline(always)]
    pub fn sdd3mon(&self) -> SDD3MON_R {
        SDD3MON_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Response End Detection"]
    #[inline(always)]
    #[must_use]
    pub fn rspend(&mut self) -> RSPEND_W<0> {
        RSPEND_W::new(self)
    }
    #[doc = "Bit 2 - Access End"]
    #[inline(always)]
    #[must_use]
    pub fn acend(&mut self) -> ACEND_W<2> {
        ACEND_W::new(self)
    }
    #[doc = "Bit 3 - SDnCD Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn sdcdrm(&mut self) -> SDCDRM_W<3> {
        SDCDRM_W::new(self)
    }
    #[doc = "Bit 4 - SDnCD Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn sdcdin(&mut self) -> SDCDIN_W<4> {
        SDCDIN_W::new(self)
    }
    #[doc = "Bit 8 - SDnDAT3 Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn sdd3rm(&mut self) -> SDD3RM_W<8> {
        SDD3RM_W::new(self)
    }
    #[doc = "Bit 9 - SDnDAT3 Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn sdd3in(&mut self) -> SDD3IN_W<9> {
        SDD3IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Card Interrupt Flag Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_info1](index.html) module"]
pub struct SD_INFO1_SPEC;
impl crate::RegisterSpec for SD_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_info1::R](R) reader structure"]
impl crate::Readable for SD_INFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_info1::W](W) writer structure"]
impl crate::Writable for SD_INFO1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x031c;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_INFO1 to value 0"]
impl crate::Resettable for SD_INFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
