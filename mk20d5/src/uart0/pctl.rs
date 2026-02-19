#[doc = "Register `PCTL` reader"]
pub type R = crate::R<PctlSpec>;
#[doc = "Register `PCTL` writer"]
pub type W = crate::W<PctlSpec>;
#[doc = "Field `PCTL` reader - Packet Cycle Time Counter Low"]
pub type PctlR = crate::FieldReader;
#[doc = "Field `PCTL` writer - Packet Cycle Time Counter Low"]
pub type PctlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Packet Cycle Time Counter Low"]
    #[inline(always)]
    pub fn pctl(&self) -> PctlR {
        PctlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Packet Cycle Time Counter Low"]
    #[inline(always)]
    #[must_use]
    pub fn pctl(&mut self) -> PctlW<PctlSpec> {
        PctlW::new(self, 0)
    }
}
#[doc = "UART CEA709.1-B Packet Cycle Time Counter Low\n\nYou can [`read`](crate::Reg::read) this register and get [`pctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PctlSpec;
impl crate::RegisterSpec for PctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pctl::R`](R) reader structure"]
impl crate::Readable for PctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pctl::W`](W) writer structure"]
impl crate::Writable for PctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PCTL to value 0"]
impl crate::Resettable for PctlSpec {
    const RESET_VALUE: u8 = 0;
}
