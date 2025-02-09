#[doc = "Register `TIMGCLK` reader"]
pub struct R(crate::R<TIMGCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMGCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMGCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMGCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMGCLK` writer"]
pub struct W(crate::W<TIMGCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMGCLK_SPEC>;
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
impl From<crate::W<TIMGCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMGCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - Force clock enable for this regfile"]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - Force clock enable for this regfile"]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMGCLK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Force clock enable for this regfile"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Force clock enable for this regfile"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timgclk](index.html) module"]
pub struct TIMGCLK_SPEC;
impl crate::RegisterSpec for TIMGCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timgclk::R](R) reader structure"]
impl crate::Readable for TIMGCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timgclk::W](W) writer structure"]
impl crate::Writable for TIMGCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMGCLK to value 0"]
impl crate::Resettable for TIMGCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
