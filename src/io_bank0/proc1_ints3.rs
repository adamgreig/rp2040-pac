#[doc = "Register `PROC1_INTS3` reader"]
pub struct R(crate::R<PROC1_INTS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC1_INTS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC1_INTS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC1_INTS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO29_EDGE_HIGH` reader - "]
pub struct GPIO29_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO29_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO29_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO29_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO29_EDGE_LOW` reader - "]
pub struct GPIO29_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO29_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO29_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO29_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO29_LEVEL_HIGH` reader - "]
pub struct GPIO29_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO29_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO29_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO29_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO29_LEVEL_LOW` reader - "]
pub struct GPIO29_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO29_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO29_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO29_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO28_EDGE_HIGH` reader - "]
pub struct GPIO28_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO28_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO28_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO28_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO28_EDGE_LOW` reader - "]
pub struct GPIO28_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO28_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO28_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO28_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO28_LEVEL_HIGH` reader - "]
pub struct GPIO28_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO28_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO28_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO28_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO28_LEVEL_LOW` reader - "]
pub struct GPIO28_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO28_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO28_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO28_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO27_EDGE_HIGH` reader - "]
pub struct GPIO27_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO27_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO27_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO27_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO27_EDGE_LOW` reader - "]
pub struct GPIO27_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO27_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO27_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO27_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO27_LEVEL_HIGH` reader - "]
pub struct GPIO27_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO27_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO27_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO27_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO27_LEVEL_LOW` reader - "]
pub struct GPIO27_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO27_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO27_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO27_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO26_EDGE_HIGH` reader - "]
pub struct GPIO26_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO26_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO26_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO26_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO26_EDGE_LOW` reader - "]
pub struct GPIO26_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO26_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO26_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO26_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO26_LEVEL_HIGH` reader - "]
pub struct GPIO26_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO26_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO26_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO26_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO26_LEVEL_LOW` reader - "]
pub struct GPIO26_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO26_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO26_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO26_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO25_EDGE_HIGH` reader - "]
pub struct GPIO25_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO25_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO25_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO25_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO25_EDGE_LOW` reader - "]
pub struct GPIO25_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO25_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO25_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO25_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO25_LEVEL_HIGH` reader - "]
pub struct GPIO25_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO25_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO25_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO25_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO25_LEVEL_LOW` reader - "]
pub struct GPIO25_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO25_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO25_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO25_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO24_EDGE_HIGH` reader - "]
pub struct GPIO24_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO24_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO24_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO24_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO24_EDGE_LOW` reader - "]
pub struct GPIO24_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO24_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO24_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO24_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO24_LEVEL_HIGH` reader - "]
pub struct GPIO24_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO24_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO24_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO24_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO24_LEVEL_LOW` reader - "]
pub struct GPIO24_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO24_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO24_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO24_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio29_edge_high(&self) -> GPIO29_EDGE_HIGH_R {
        GPIO29_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio29_edge_low(&self) -> GPIO29_EDGE_LOW_R {
        GPIO29_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio29_level_high(&self) -> GPIO29_LEVEL_HIGH_R {
        GPIO29_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio29_level_low(&self) -> GPIO29_LEVEL_LOW_R {
        GPIO29_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio28_edge_high(&self) -> GPIO28_EDGE_HIGH_R {
        GPIO28_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio28_edge_low(&self) -> GPIO28_EDGE_LOW_R {
        GPIO28_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio28_level_high(&self) -> GPIO28_LEVEL_HIGH_R {
        GPIO28_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio28_level_low(&self) -> GPIO28_LEVEL_LOW_R {
        GPIO28_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio27_edge_high(&self) -> GPIO27_EDGE_HIGH_R {
        GPIO27_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio27_edge_low(&self) -> GPIO27_EDGE_LOW_R {
        GPIO27_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio27_level_high(&self) -> GPIO27_LEVEL_HIGH_R {
        GPIO27_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio27_level_low(&self) -> GPIO27_LEVEL_LOW_R {
        GPIO27_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio26_edge_high(&self) -> GPIO26_EDGE_HIGH_R {
        GPIO26_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio26_edge_low(&self) -> GPIO26_EDGE_LOW_R {
        GPIO26_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio26_level_high(&self) -> GPIO26_LEVEL_HIGH_R {
        GPIO26_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio26_level_low(&self) -> GPIO26_LEVEL_LOW_R {
        GPIO26_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio25_edge_high(&self) -> GPIO25_EDGE_HIGH_R {
        GPIO25_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio25_edge_low(&self) -> GPIO25_EDGE_LOW_R {
        GPIO25_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio25_level_high(&self) -> GPIO25_LEVEL_HIGH_R {
        GPIO25_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio25_level_low(&self) -> GPIO25_LEVEL_LOW_R {
        GPIO25_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio24_edge_high(&self) -> GPIO24_EDGE_HIGH_R {
        GPIO24_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio24_edge_low(&self) -> GPIO24_EDGE_LOW_R {
        GPIO24_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio24_level_high(&self) -> GPIO24_LEVEL_HIGH_R {
        GPIO24_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio24_level_low(&self) -> GPIO24_LEVEL_LOW_R {
        GPIO24_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status after masking & forcing for proc1  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc1_ints3](index.html) module"]
pub struct PROC1_INTS3_SPEC;
impl crate::RegisterSpec for PROC1_INTS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc1_ints3::R](R) reader structure"]
impl crate::Readable for PROC1_INTS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROC1_INTS3 to value 0"]
impl crate::Resettable for PROC1_INTS3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
