#[doc = "Register `STATE0` reader"]
pub struct R(crate::R<STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_ERR_CAUSE` reader - Indicates the error types when DMA receives the error frame. 3'b001: UHCI packet checksum error. 3'b010: UHCI packet sequence number error. 3'b011: UHCI packet CRC bit error. 3'b100: find 0xC0, but received packet is uncompleted. 3'b101: 0xC0 is not found, but received packet is completed. 3'b110: CRC check error."]
pub type RX_ERR_CAUSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECODE_STATE` reader - Indicates UHCI decoder status."]
pub type DECODE_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Indicates the error types when DMA receives the error frame. 3'b001: UHCI packet checksum error. 3'b010: UHCI packet sequence number error. 3'b011: UHCI packet CRC bit error. 3'b100: find 0xC0, but received packet is uncompleted. 3'b101: 0xC0 is not found, but received packet is completed. 3'b110: CRC check error."]
    #[inline(always)]
    pub fn rx_err_cause(&self) -> RX_ERR_CAUSE_R {
        RX_ERR_CAUSE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Indicates UHCI decoder status."]
    #[inline(always)]
    pub fn decode_state(&self) -> DECODE_STATE_R {
        DECODE_STATE_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[doc = "UHCI Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](index.html) module"]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state0::R](R) reader structure"]
impl crate::Readable for STATE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for STATE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
