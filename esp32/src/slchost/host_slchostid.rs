#[doc = "Register `HOST_SLCHOSTID` reader"]
pub struct R(crate::R<HOST_SLCHOSTID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOSTID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOSTID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOSTID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOSTID` writer"]
pub struct W(crate::W<HOST_SLCHOSTID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOSTID_SPEC>;
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
impl From<crate::W<HOST_SLCHOSTID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOSTID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_ID` reader - "]
pub type HOST_SLCHOST_ID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HOST_SLCHOST_ID` writer - "]
pub type HOST_SLCHOST_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOSTID_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_id(&self) -> HOST_SLCHOST_ID_R {
        HOST_SLCHOST_ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_id(&mut self) -> HOST_SLCHOST_ID_W<0> {
        HOST_SLCHOST_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchostid](index.html) module"]
pub struct HOST_SLCHOSTID_SPEC;
impl crate::RegisterSpec for HOST_SLCHOSTID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchostid::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOSTID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchostid::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOSTID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOSTID to value 0x0600"]
impl crate::Resettable for HOST_SLCHOSTID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
