#[doc = "Register `MG` reader"]
pub type R = crate::R<MgSpec>;
#[doc = "Register `MG` writer"]
pub type W = crate::W<MgSpec>;
#[doc = "Field `MG` reader - Minus-side gain"]
pub type MgR = crate::FieldReader<u16>;
#[doc = "Field `MG` writer - Minus-side gain"]
pub type MgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Minus-side gain"]
    #[inline(always)]
    pub fn mg(&self) -> MgR {
        MgR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minus-side gain"]
    #[inline(always)]
    #[must_use]
    pub fn mg(&mut self) -> MgW<MgSpec> {
        MgW::new(self, 0)
    }
}
#[doc = "ADC minus-side gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`mg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MgSpec;
impl crate::RegisterSpec for MgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mg::R`](R) reader structure"]
impl crate::Readable for MgSpec {}
#[doc = "`write(|w| ..)` method takes [`mg::W`](W) writer structure"]
impl crate::Writable for MgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MG to value 0x8200"]
impl crate::Resettable for MgSpec {
    const RESET_VALUE: u32 = 0x8200;
}
