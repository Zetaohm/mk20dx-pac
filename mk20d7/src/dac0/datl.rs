#[doc = "Register `DAT%sL` reader"]
pub type R = crate::R<DatlSpec>;
#[doc = "Register `DAT%sL` writer"]
pub type W = crate::W<DatlSpec>;
#[doc = "Field `DATA` reader - no description available"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - no description available"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DatlSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "DAC Data Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`datl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatlSpec;
impl crate::RegisterSpec for DatlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`datl::R`](R) reader structure"]
impl crate::Readable for DatlSpec {}
#[doc = "`write(|w| ..)` method takes [`datl::W`](W) writer structure"]
impl crate::Writable for DatlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DAT%sL to value 0"]
impl crate::Resettable for DatlSpec {
    const RESET_VALUE: u8 = 0;
}
