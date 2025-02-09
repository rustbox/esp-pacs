#[doc = "Register `TX_GENRL_CFG` reader"]
pub struct R(crate::R<TX_GENRL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_GENRL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_GENRL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_GENRL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_GENRL_CFG` writer"]
pub struct W(crate::W<TX_GENRL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_GENRL_CFG_SPEC>;
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
impl From<crate::W<TX_GENRL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_GENRL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_IDLE_VALUE` reader - Configures bus value of transmitter in IDLE state."]
pub type TX_IDLE_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_IDLE_VALUE` writer - Configures bus value of transmitter in IDLE state."]
pub type TX_IDLE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_GENRL_CFG_SPEC, u16, u16, 16, O>;
#[doc = "Field `TX_GATING_EN` reader - Set this bit to enable the clock gating of output tx clock."]
pub type TX_GATING_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_GATING_EN` writer - Set this bit to enable the clock gating of output tx clock."]
pub type TX_GATING_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_GENRL_CFG_SPEC, bool, O>;
#[doc = "Field `TX_VALID_OUTPUT_EN` reader - Set this bit to enable the output of tx data valid signal."]
pub type TX_VALID_OUTPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_VALID_OUTPUT_EN` writer - Set this bit to enable the output of tx data valid signal."]
pub type TX_VALID_OUTPUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TX_GENRL_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 14:29 - Configures bus value of transmitter in IDLE state."]
    #[inline(always)]
    pub fn tx_idle_value(&self) -> TX_IDLE_VALUE_R {
        TX_IDLE_VALUE_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Set this bit to enable the clock gating of output tx clock."]
    #[inline(always)]
    pub fn tx_gating_en(&self) -> TX_GATING_EN_R {
        TX_GATING_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to enable the output of tx data valid signal."]
    #[inline(always)]
    pub fn tx_valid_output_en(&self) -> TX_VALID_OUTPUT_EN_R {
        TX_VALID_OUTPUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 14:29 - Configures bus value of transmitter in IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle_value(&mut self) -> TX_IDLE_VALUE_W<14> {
        TX_IDLE_VALUE_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to enable the clock gating of output tx clock."]
    #[inline(always)]
    #[must_use]
    pub fn tx_gating_en(&mut self) -> TX_GATING_EN_W<30> {
        TX_GATING_EN_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to enable the output of tx data valid signal."]
    #[inline(always)]
    #[must_use]
    pub fn tx_valid_output_en(&mut self) -> TX_VALID_OUTPUT_EN_W<31> {
        TX_VALID_OUTPUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel TX general configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_genrl_cfg](index.html) module"]
pub struct TX_GENRL_CFG_SPEC;
impl crate::RegisterSpec for TX_GENRL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_genrl_cfg::R](R) reader structure"]
impl crate::Readable for TX_GENRL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_genrl_cfg::W](W) writer structure"]
impl crate::Writable for TX_GENRL_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_GENRL_CFG to value 0"]
impl crate::Resettable for TX_GENRL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
