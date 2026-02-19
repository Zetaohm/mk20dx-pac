#[doc = "Register `DATAW1S%sU` reader"]
pub type R = crate::R<Dataw1suSpec>;
#[doc = "Register `DATAW1S%sU` writer"]
pub type W = crate::W<Dataw1suSpec>;
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
    pub fn data(&mut self) -> DataW<Dataw1suSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cache Data Storage (upper word)\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw1su::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw1su::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dataw1suSpec;
impl crate::RegisterSpec for Dataw1suSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataw1su::R`](R) reader structure"]
impl crate::Readable for Dataw1suSpec {}
#[doc = "`write(|w| ..)` method takes [`dataw1su::W`](W) writer structure"]
impl crate::Writable for Dataw1suSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAW1S%sU to value 0"]
impl crate::Resettable for Dataw1suSpec {
    const RESET_VALUE: u32 = 0;
}
