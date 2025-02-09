#[doc = "Register `STORE3` reader"]
pub struct R(crate::R<STORE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE3` writer"]
pub struct W(crate::W<STORE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE3_SPEC>;
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
impl From<crate::W<STORE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_AON_STORE3` reader - need_des"]
pub type LP_AON_STORE3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LP_AON_STORE3` writer - need_des"]
pub type LP_AON_STORE3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORE3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store3(&self) -> LP_AON_STORE3_R {
        LP_AON_STORE3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store3(&mut self) -> LP_AON_STORE3_W<0> {
        LP_AON_STORE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store3](index.html) module"]
pub struct STORE3_SPEC;
impl crate::RegisterSpec for STORE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store3::R](R) reader structure"]
impl crate::Readable for STORE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store3::W](W) writer structure"]
impl crate::Writable for STORE3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE3 to value 0"]
impl crate::Resettable for STORE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
