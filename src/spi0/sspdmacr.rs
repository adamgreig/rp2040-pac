#[doc = "Register `SSPDMACR` reader"]
pub struct R(crate::R<SSPDMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPDMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPDMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPDMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPDMACR` writer"]
pub struct W(crate::W<SSPDMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPDMACR_SPEC>;
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
impl From<crate::W<SSPDMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPDMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDMAE` reader - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub struct TXDMAE_R(crate::FieldReader<bool, bool>);
impl TXDMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMAE` writer - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub struct TXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXDMAE` reader - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub struct RXDMAE_R(crate::FieldReader<bool, bool>);
impl RXDMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMAE` writer - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub struct RXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TXDMAE_W {
        TXDMAE_W { w: self }
    }
    #[doc = "Bit 0 - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RXDMAE_W {
        RXDMAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control register, SSPDMACR on page 3-12  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspdmacr](index.html) module"]
pub struct SSPDMACR_SPEC;
impl crate::RegisterSpec for SSPDMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspdmacr::R](R) reader structure"]
impl crate::Readable for SSPDMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspdmacr::W](W) writer structure"]
impl crate::Writable for SSPDMACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSPDMACR to value 0"]
impl crate::Resettable for SSPDMACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
