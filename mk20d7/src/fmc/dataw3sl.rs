#[doc = "Register `DATAW3S%sL` reader"]
pub type R = crate::R<Dataw3slSpec>;
#[doc = "Register `DATAW3S%sL` writer"]
pub type W = crate::W<Dataw3slSpec>;
#[doc = "Field `data` reader - Bits \\[31:0\\]
of data entry"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Bits \\[31:0\\]
of data entry"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of data entry"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of data entry"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Dataw3slSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cache Data Storage (lower word)\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw3sl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw3sl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dataw3slSpec;
impl crate::RegisterSpec for Dataw3slSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataw3sl::R`](R) reader structure"]
impl crate::Readable for Dataw3slSpec {}
#[doc = "`write(|w| ..)` method takes [`dataw3sl::W`](W) writer structure"]
impl crate::Writable for Dataw3slSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAW3S%sL to value 0"]
impl crate::Resettable for Dataw3slSpec {
    const RESET_VALUE: u32 = 0;
}
