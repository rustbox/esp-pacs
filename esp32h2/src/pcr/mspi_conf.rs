#[doc = "Register `MSPI_CONF` reader"]
pub struct R(crate::R<MSPI_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSPI_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSPI_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSPI_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSPI_CONF` writer"]
pub struct W(crate::W<MSPI_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSPI_CONF_SPEC>;
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
impl From<crate::W<MSPI_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSPI_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSPI_CLK_EN` reader - Set 1 to enable mspi clock, include mspi pll clock"]
pub type MSPI_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `MSPI_CLK_EN` writer - Set 1 to enable mspi clock, include mspi pll clock"]
pub type MSPI_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSPI_CONF_SPEC, bool, O>;
#[doc = "Field `MSPI_RST_EN` reader - Set 0 to reset mspi module"]
pub type MSPI_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `MSPI_RST_EN` writer - Set 0 to reset mspi module"]
pub type MSPI_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSPI_CONF_SPEC, bool, O>;
#[doc = "Field `MSPI_PLL_CLK_EN` reader - Set 1 to enable mspi pll clock"]
pub type MSPI_PLL_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `MSPI_PLL_CLK_EN` writer - Set 1 to enable mspi pll clock"]
pub type MSPI_PLL_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSPI_CONF_SPEC, bool, O>;
#[doc = "Field `MSPI_CLK_SEL` reader - set this field to select clock-source."]
pub type MSPI_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSPI_CLK_SEL` writer - set this field to select clock-source."]
pub type MSPI_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MSPI_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `MSPI_READY` reader - Query this field after reset mspi module"]
pub type MSPI_READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable mspi clock, include mspi pll clock"]
    #[inline(always)]
    pub fn mspi_clk_en(&self) -> MSPI_CLK_EN_R {
        MSPI_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset mspi module"]
    #[inline(always)]
    pub fn mspi_rst_en(&self) -> MSPI_RST_EN_R {
        MSPI_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable mspi pll clock"]
    #[inline(always)]
    pub fn mspi_pll_clk_en(&self) -> MSPI_PLL_CLK_EN_R {
        MSPI_PLL_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - set this field to select clock-source."]
    #[inline(always)]
    pub fn mspi_clk_sel(&self) -> MSPI_CLK_SEL_R {
        MSPI_CLK_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Query this field after reset mspi module"]
    #[inline(always)]
    pub fn mspi_ready(&self) -> MSPI_READY_R {
        MSPI_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable mspi clock, include mspi pll clock"]
    #[inline(always)]
    #[must_use]
    pub fn mspi_clk_en(&mut self) -> MSPI_CLK_EN_W<0> {
        MSPI_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset mspi module"]
    #[inline(always)]
    #[must_use]
    pub fn mspi_rst_en(&mut self) -> MSPI_RST_EN_W<1> {
        MSPI_RST_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable mspi pll clock"]
    #[inline(always)]
    #[must_use]
    pub fn mspi_pll_clk_en(&mut self) -> MSPI_PLL_CLK_EN_W<2> {
        MSPI_PLL_CLK_EN_W::new(self)
    }
    #[doc = "Bits 3:4 - set this field to select clock-source."]
    #[inline(always)]
    #[must_use]
    pub fn mspi_clk_sel(&mut self) -> MSPI_CLK_SEL_W<3> {
        MSPI_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspi_conf](index.html) module"]
pub struct MSPI_CONF_SPEC;
impl crate::RegisterSpec for MSPI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mspi_conf::R](R) reader structure"]
impl crate::Readable for MSPI_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mspi_conf::W](W) writer structure"]
impl crate::Writable for MSPI_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPI_CONF to value 0x25"]
impl crate::Resettable for MSPI_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x25;
}
