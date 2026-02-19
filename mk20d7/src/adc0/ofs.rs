#[doc = "Register `OFS` reader"]
pub type R = crate::R<OfsSpec>;
#[doc = "Register `OFS` writer"]
pub type W = crate::W<OfsSpec>;
#[doc = "Field `OFS` reader - Offset error correction value"]
pub type OfsR = crate::FieldReader<u16>;
#[doc = "Field `OFS` writer - Offset error correction value"]
pub type OfsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Offset error correction value"]
    #[inline(always)]
    pub fn ofs(&self) -> OfsR {
        OfsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset error correction value"]
    #[inline(always)]
    #[must_use]
    pub fn ofs(&mut self) -> OfsW<OfsSpec> {
        OfsW::new(self, 0)
    }
}
#[doc = "ADC offset correction register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OfsSpec;
impl crate::RegisterSpec for OfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofs::R`](R) reader structure"]
impl crate::Readable for OfsSpec {}
#[doc = "`write(|w| ..)` method takes [`ofs::W`](W) writer structure"]
impl crate::Writable for OfsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFS to value 0x04"]
impl crate::Resettable for OfsSpec {
    const RESET_VALUE: u32 = 0x04;
}
