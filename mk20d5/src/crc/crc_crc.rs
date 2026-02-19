#[doc = "Register `CRC` reader"]
pub type R = crate::R<CrcCrcSpec>;
#[doc = "Register `CRC` writer"]
pub type W = crate::W<CrcCrcSpec>;
#[doc = "Field `LL` reader - CRC Low Lower Byte"]
pub type LlR = crate::FieldReader;
#[doc = "Field `LL` writer - CRC Low Lower Byte"]
pub type LlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LU` reader - CRC Low Upper Byte"]
pub type LuR = crate::FieldReader;
#[doc = "Field `LU` writer - CRC Low Upper Byte"]
pub type LuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HL` reader - CRC High Lower Byte"]
pub type HlR = crate::FieldReader;
#[doc = "Field `HL` writer - CRC High Lower Byte"]
pub type HlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HU` reader - CRC High Upper Byte"]
pub type HuR = crate::FieldReader;
#[doc = "Field `HU` writer - CRC High Upper Byte"]
pub type HuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CRC Low Lower Byte"]
    #[inline(always)]
    pub fn ll(&self) -> LlR {
        LlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CRC Low Upper Byte"]
    #[inline(always)]
    pub fn lu(&self) -> LuR {
        LuR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CRC High Lower Byte"]
    #[inline(always)]
    pub fn hl(&self) -> HlR {
        HlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CRC High Upper Byte"]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRC Low Lower Byte"]
    #[inline(always)]
    #[must_use]
    pub fn ll(&mut self) -> LlW<CrcCrcSpec> {
        LlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - CRC Low Upper Byte"]
    #[inline(always)]
    #[must_use]
    pub fn lu(&mut self) -> LuW<CrcCrcSpec> {
        LuW::new(self, 8)
    }
    #[doc = "Bits 16:23 - CRC High Lower Byte"]
    #[inline(always)]
    #[must_use]
    pub fn hl(&mut self) -> HlW<CrcCrcSpec> {
        HlW::new(self, 16)
    }
    #[doc = "Bits 24:31 - CRC High Upper Byte"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HuW<CrcCrcSpec> {
        HuW::new(self, 24)
    }
}
#[doc = "CRC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_crc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_crc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCrcSpec;
impl crate::RegisterSpec for CrcCrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_crc::R`](R) reader structure"]
impl crate::Readable for CrcCrcSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_crc::W`](W) writer structure"]
impl crate::Writable for CrcCrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC to value 0xffff_ffff"]
impl crate::Resettable for CrcCrcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
