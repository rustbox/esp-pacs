#[doc = "Register `GEN0_TSTMP_B` reader"]
pub struct R(crate::R<GEN0_TSTMP_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN0_TSTMP_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN0_TSTMP_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN0_TSTMP_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN0_TSTMP_B` writer"]
pub struct W(crate::W<GEN0_TSTMP_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN0_TSTMP_B_SPEC>;
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
impl From<crate::W<GEN0_TSTMP_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN0_TSTMP_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR0_B` reader - PWM generator 0 time stamp B's shadow register"]
pub type CMPR0_B_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPR0_B` writer - PWM generator 0 time stamp B's shadow register"]
pub type CMPR0_B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GEN0_TSTMP_B_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp B's shadow register"]
    #[inline(always)]
    pub fn cmpr0_b(&self) -> CMPR0_B_R {
        CMPR0_B_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp B's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_b(&mut self) -> CMPR0_B_W<0> {
        CMPR0_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow register for register B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen0_tstmp_b](index.html) module"]
pub struct GEN0_TSTMP_B_SPEC;
impl crate::RegisterSpec for GEN0_TSTMP_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen0_tstmp_b::R](R) reader structure"]
impl crate::Readable for GEN0_TSTMP_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen0_tstmp_b::W](W) writer structure"]
impl crate::Writable for GEN0_TSTMP_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN0_TSTMP_B to value 0"]
impl crate::Resettable for GEN0_TSTMP_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
