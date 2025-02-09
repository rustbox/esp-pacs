#[doc = "Register `M2_STATUS` reader"]
pub struct R(crate::R<M2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M2_EXCEPTION_STATUS` reader - Exception status"]
pub type M2_EXCEPTION_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Exception status"]
    #[inline(always)]
    pub fn m2_exception_status(&self) -> M2_EXCEPTION_STATUS_R {
        M2_EXCEPTION_STATUS_R::new((self.bits & 3) as u8)
    }
}
#[doc = "M2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2_status](index.html) module"]
pub struct M2_STATUS_SPEC;
impl crate::RegisterSpec for M2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m2_status::R](R) reader structure"]
impl crate::Readable for M2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M2_STATUS to value 0"]
impl crate::Resettable for M2_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
