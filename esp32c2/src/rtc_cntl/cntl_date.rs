#[doc = "Register `CNTL_DATE` reader"]
pub struct R(crate::R<CNTL_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL_DATE` writer"]
pub struct W(crate::W<CNTL_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL_DATE_SPEC>;
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
impl From<crate::W<CNTL_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTL_DATE` reader - Need add desc"]
pub type CNTL_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNTL_DATE` writer - Need add desc"]
pub type CNTL_DATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_DATE_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - Need add desc"]
    #[inline(always)]
    pub fn cntl_date(&self) -> CNTL_DATE_R {
        CNTL_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn cntl_date(&mut self) -> CNTL_DATE_W<0> {
        CNTL_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl_date](index.html) module"]
pub struct CNTL_DATE_SPEC;
impl crate::RegisterSpec for CNTL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntl_date::R](R) reader structure"]
impl crate::Readable for CNTL_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl_date::W](W) writer structure"]
impl crate::Writable for CNTL_DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTL_DATE to value 0x0210_7190"]
impl crate::Resettable for CNTL_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0210_7190;
}
