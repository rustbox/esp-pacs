#[doc = "Register `SLP_WAKEUP_CNTL2` reader"]
pub struct R(crate::R<SLP_WAKEUP_CNTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKEUP_CNTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKEUP_CNTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKEUP_CNTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_WAKEUP_CNTL2` writer"]
pub struct W(crate::W<SLP_WAKEUP_CNTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_WAKEUP_CNTL2_SPEC>;
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
impl From<crate::W<SLP_WAKEUP_CNTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_WAKEUP_CNTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_ENA` reader - need_des"]
pub type WAKEUP_ENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WAKEUP_ENA` writer - need_des"]
pub type WAKEUP_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLP_WAKEUP_CNTL2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wakeup_ena(&self) -> WAKEUP_ENA_R {
        WAKEUP_ENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_ena(&mut self) -> WAKEUP_ENA_W<0> {
        WAKEUP_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cntl2](index.html) module"]
pub struct SLP_WAKEUP_CNTL2_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_wakeup_cntl2::R](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_wakeup_cntl2::W](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL2 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
