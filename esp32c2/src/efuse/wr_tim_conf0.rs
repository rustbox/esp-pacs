#[doc = "Register `WR_TIM_CONF0` reader"]
pub struct R(crate::R<WR_TIM_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_TIM_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_TIM_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_TIM_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_TIM_CONF0` writer"]
pub struct W(crate::W<WR_TIM_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_TIM_CONF0_SPEC>;
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
impl From<crate::W<WR_TIM_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_TIM_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THP_A` reader - Configures hold time for efuse program."]
pub type THP_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THP_A` writer - Configures hold time for efuse program."]
pub type THP_A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR_TIM_CONF0_SPEC, u8, u8, 8, O>;
#[doc = "Field `TPGM_INACTIVE` reader - Configures pulse time for burning '0' bit."]
pub type TPGM_INACTIVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPGM_INACTIVE` writer - Configures pulse time for burning '0' bit."]
pub type TPGM_INACTIVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WR_TIM_CONF0_SPEC, u8, u8, 8, O>;
#[doc = "Field `TPGM` reader - Configures pulse time for burning '1' bit."]
pub type TPGM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPGM` writer - Configures pulse time for burning '1' bit."]
pub type TPGM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR_TIM_CONF0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - Configures hold time for efuse program."]
    #[inline(always)]
    pub fn thp_a(&self) -> THP_A_R {
        THP_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures pulse time for burning '0' bit."]
    #[inline(always)]
    pub fn tpgm_inactive(&self) -> TPGM_INACTIVE_R {
        TPGM_INACTIVE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Configures pulse time for burning '1' bit."]
    #[inline(always)]
    pub fn tpgm(&self) -> TPGM_R {
        TPGM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures hold time for efuse program."]
    #[inline(always)]
    #[must_use]
    pub fn thp_a(&mut self) -> THP_A_W<0> {
        THP_A_W::new(self)
    }
    #[doc = "Bits 8:15 - Configures pulse time for burning '0' bit."]
    #[inline(always)]
    #[must_use]
    pub fn tpgm_inactive(&mut self) -> TPGM_INACTIVE_W<8> {
        TPGM_INACTIVE_W::new(self)
    }
    #[doc = "Bits 16:31 - Configures pulse time for burning '1' bit."]
    #[inline(always)]
    #[must_use]
    pub fn tpgm(&mut self) -> TPGM_W<16> {
        TPGM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurarion register 0 of eFuse programming timing parameters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_tim_conf0](index.html) module"]
pub struct WR_TIM_CONF0_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_tim_conf0::R](R) reader structure"]
impl crate::Readable for WR_TIM_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_tim_conf0::W](W) writer structure"]
impl crate::Writable for WR_TIM_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_TIM_CONF0 to value 0x00c8_0101"]
impl crate::Resettable for WR_TIM_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c8_0101;
}
