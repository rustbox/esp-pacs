#[doc = "Register `IBUS1_ACS_MISS_CNT` reader"]
pub struct R(crate::R<IBUS1_ACS_MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS1_ACS_MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS1_ACS_MISS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS1_ACS_MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBUS1_ACS_MISS_CNT` reader - The bits are used to count the number of the cache miss caused by ibus1 access."]
pub type IBUS1_ACS_MISS_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of the cache miss caused by ibus1 access."]
    #[inline(always)]
    pub fn ibus1_acs_miss_cnt(&self) -> IBUS1_ACS_MISS_CNT_R {
        IBUS1_ACS_MISS_CNT_R::new(self.bits)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus1_acs_miss_cnt](index.html) module"]
pub struct IBUS1_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for IBUS1_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus1_acs_miss_cnt::R](R) reader structure"]
impl crate::Readable for IBUS1_ACS_MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IBUS1_ACS_MISS_CNT to value 0"]
impl crate::Resettable for IBUS1_ACS_MISS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
