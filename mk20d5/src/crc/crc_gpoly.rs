#[doc = "Register `GPOLY` reader"]
pub type R = crate::R<CrcGpolySpec>;
#[doc = "Register `GPOLY` writer"]
pub type W = crate::W<CrcGpolySpec>;
#[doc = "Field `LOW` reader - Low polynominal half-word"]
pub type LowR = crate::FieldReader<u16>;
#[doc = "Field `LOW` writer - Low polynominal half-word"]
pub type LowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HIGH` reader - High polynominal half-word"]
pub type HighR = crate::FieldReader<u16>;
#[doc = "Field `HIGH` writer - High polynominal half-word"]
pub type HighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low polynominal half-word"]
    #[inline(always)]
    pub fn low(&self) -> LowR {
        LowR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High polynominal half-word"]
    #[inline(always)]
    pub fn high(&self) -> HighR {
        HighR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low polynominal half-word"]
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LowW<CrcGpolySpec> {
        LowW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High polynominal half-word"]
    #[inline(always)]
    #[must_use]
    pub fn high(&mut self) -> HighW<CrcGpolySpec> {
        HighW::new(self, 16)
    }
}
#[doc = "CRC Polynomial Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_gpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_gpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcGpolySpec;
impl crate::RegisterSpec for CrcGpolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_gpoly::R`](R) reader structure"]
impl crate::Readable for CrcGpolySpec {}
#[doc = "`write(|w| ..)` method takes [`crc_gpoly::W`](W) writer structure"]
impl crate::Writable for CrcGpolySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPOLY to value 0x1021"]
impl crate::Resettable for CrcGpolySpec {
    const RESET_VALUE: u32 = 0x1021;
}
