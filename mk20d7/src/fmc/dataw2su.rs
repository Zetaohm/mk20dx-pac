#[doc = "Register `DATAW2S%sU` reader"]
pub type R = crate::R<Dataw2suSpec>;
#[doc = "Register `DATAW2S%sU` writer"]
pub type W = crate::W<Dataw2suSpec>;
#[doc = "Field `data` reader - Bits \\[63:32\\]
of data entry"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Bits \\[63:32\\]
of data entry"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of data entry"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of data entry"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Dataw2suSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cache Data Storage (upper word)\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw2su::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw2su::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dataw2suSpec;
impl crate::RegisterSpec for Dataw2suSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataw2su::R`](R) reader structure"]
impl crate::Readable for Dataw2suSpec {}
#[doc = "`write(|w| ..)` method takes [`dataw2su::W`](W) writer structure"]
impl crate::Writable for Dataw2suSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAW2S%sU to value 0"]
impl crate::Resettable for Dataw2suSpec {
    const RESET_VALUE: u32 = 0;
}
