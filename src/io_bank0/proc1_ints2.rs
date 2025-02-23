#[doc = "Register `PROC1_INTS2` reader"]
pub struct R(crate::R<PROC1_INTS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC1_INTS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC1_INTS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC1_INTS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO23_EDGE_HIGH` reader - "]
pub struct GPIO23_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO23_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO23_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO23_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO23_EDGE_LOW` reader - "]
pub struct GPIO23_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO23_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO23_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO23_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO23_LEVEL_HIGH` reader - "]
pub struct GPIO23_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO23_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO23_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO23_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO23_LEVEL_LOW` reader - "]
pub struct GPIO23_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO23_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO23_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO23_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO22_EDGE_HIGH` reader - "]
pub struct GPIO22_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO22_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO22_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO22_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO22_EDGE_LOW` reader - "]
pub struct GPIO22_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO22_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO22_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO22_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO22_LEVEL_HIGH` reader - "]
pub struct GPIO22_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO22_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO22_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO22_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO22_LEVEL_LOW` reader - "]
pub struct GPIO22_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO22_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO22_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO22_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO21_EDGE_HIGH` reader - "]
pub struct GPIO21_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO21_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO21_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO21_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO21_EDGE_LOW` reader - "]
pub struct GPIO21_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO21_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO21_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO21_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO21_LEVEL_HIGH` reader - "]
pub struct GPIO21_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO21_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO21_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO21_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO21_LEVEL_LOW` reader - "]
pub struct GPIO21_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO21_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO21_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO21_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO20_EDGE_HIGH` reader - "]
pub struct GPIO20_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO20_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO20_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO20_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO20_EDGE_LOW` reader - "]
pub struct GPIO20_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO20_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO20_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO20_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO20_LEVEL_HIGH` reader - "]
pub struct GPIO20_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO20_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO20_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO20_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO20_LEVEL_LOW` reader - "]
pub struct GPIO20_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO20_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO20_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO20_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO19_EDGE_HIGH` reader - "]
pub struct GPIO19_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO19_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO19_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO19_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO19_EDGE_LOW` reader - "]
pub struct GPIO19_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO19_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO19_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO19_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO19_LEVEL_HIGH` reader - "]
pub struct GPIO19_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO19_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO19_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO19_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO19_LEVEL_LOW` reader - "]
pub struct GPIO19_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO19_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO19_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO19_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO18_EDGE_HIGH` reader - "]
pub struct GPIO18_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO18_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO18_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO18_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO18_EDGE_LOW` reader - "]
pub struct GPIO18_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO18_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO18_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO18_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO18_LEVEL_HIGH` reader - "]
pub struct GPIO18_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO18_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO18_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO18_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO18_LEVEL_LOW` reader - "]
pub struct GPIO18_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO18_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO18_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO18_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO17_EDGE_HIGH` reader - "]
pub struct GPIO17_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO17_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO17_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO17_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO17_EDGE_LOW` reader - "]
pub struct GPIO17_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO17_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO17_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO17_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO17_LEVEL_HIGH` reader - "]
pub struct GPIO17_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO17_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO17_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO17_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO17_LEVEL_LOW` reader - "]
pub struct GPIO17_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO17_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO17_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO17_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO16_EDGE_HIGH` reader - "]
pub struct GPIO16_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO16_EDGE_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO16_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO16_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO16_EDGE_LOW` reader - "]
pub struct GPIO16_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO16_EDGE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO16_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO16_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO16_LEVEL_HIGH` reader - "]
pub struct GPIO16_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO16_LEVEL_HIGH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO16_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO16_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO16_LEVEL_LOW` reader - "]
pub struct GPIO16_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO16_LEVEL_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO16_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO16_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio23_edge_high(&self) -> GPIO23_EDGE_HIGH_R {
        GPIO23_EDGE_HIGH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio23_edge_low(&self) -> GPIO23_EDGE_LOW_R {
        GPIO23_EDGE_LOW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio23_level_high(&self) -> GPIO23_LEVEL_HIGH_R {
        GPIO23_LEVEL_HIGH_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio23_level_low(&self) -> GPIO23_LEVEL_LOW_R {
        GPIO23_LEVEL_LOW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio22_edge_high(&self) -> GPIO22_EDGE_HIGH_R {
        GPIO22_EDGE_HIGH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio22_edge_low(&self) -> GPIO22_EDGE_LOW_R {
        GPIO22_EDGE_LOW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpio22_level_high(&self) -> GPIO22_LEVEL_HIGH_R {
        GPIO22_LEVEL_HIGH_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpio22_level_low(&self) -> GPIO22_LEVEL_LOW_R {
        GPIO22_LEVEL_LOW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio21_edge_high(&self) -> GPIO21_EDGE_HIGH_R {
        GPIO21_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio21_edge_low(&self) -> GPIO21_EDGE_LOW_R {
        GPIO21_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio21_level_high(&self) -> GPIO21_LEVEL_HIGH_R {
        GPIO21_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio21_level_low(&self) -> GPIO21_LEVEL_LOW_R {
        GPIO21_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio20_edge_high(&self) -> GPIO20_EDGE_HIGH_R {
        GPIO20_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio20_edge_low(&self) -> GPIO20_EDGE_LOW_R {
        GPIO20_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio20_level_high(&self) -> GPIO20_LEVEL_HIGH_R {
        GPIO20_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio20_level_low(&self) -> GPIO20_LEVEL_LOW_R {
        GPIO20_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio19_edge_high(&self) -> GPIO19_EDGE_HIGH_R {
        GPIO19_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio19_edge_low(&self) -> GPIO19_EDGE_LOW_R {
        GPIO19_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio19_level_high(&self) -> GPIO19_LEVEL_HIGH_R {
        GPIO19_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio19_level_low(&self) -> GPIO19_LEVEL_LOW_R {
        GPIO19_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio18_edge_high(&self) -> GPIO18_EDGE_HIGH_R {
        GPIO18_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio18_edge_low(&self) -> GPIO18_EDGE_LOW_R {
        GPIO18_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio18_level_high(&self) -> GPIO18_LEVEL_HIGH_R {
        GPIO18_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio18_level_low(&self) -> GPIO18_LEVEL_LOW_R {
        GPIO18_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio17_edge_high(&self) -> GPIO17_EDGE_HIGH_R {
        GPIO17_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio17_edge_low(&self) -> GPIO17_EDGE_LOW_R {
        GPIO17_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio17_level_high(&self) -> GPIO17_LEVEL_HIGH_R {
        GPIO17_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio17_level_low(&self) -> GPIO17_LEVEL_LOW_R {
        GPIO17_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio16_edge_high(&self) -> GPIO16_EDGE_HIGH_R {
        GPIO16_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio16_edge_low(&self) -> GPIO16_EDGE_LOW_R {
        GPIO16_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio16_level_high(&self) -> GPIO16_LEVEL_HIGH_R {
        GPIO16_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio16_level_low(&self) -> GPIO16_LEVEL_LOW_R {
        GPIO16_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status after masking & forcing for proc1  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc1_ints2](index.html) module"]
pub struct PROC1_INTS2_SPEC;
impl crate::RegisterSpec for PROC1_INTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc1_ints2::R](R) reader structure"]
impl crate::Readable for PROC1_INTS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROC1_INTS2 to value 0"]
impl crate::Resettable for PROC1_INTS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
