#[doc = "Register `PCTH` reader"]
pub type R = crate::R<PcthSpec>;
#[doc = "Register `PCTH` writer"]
pub type W = crate::W<PcthSpec>;
#[doc = "Field `PCTH` reader - Packet Cycle Time Counter High"]
pub type PcthR = crate::FieldReader;
#[doc = "Field `PCTH` writer - Packet Cycle Time Counter High"]
pub type PcthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Packet Cycle Time Counter High"]
    #[inline(always)]
    pub fn pcth(&self) -> PcthR {
        PcthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Packet Cycle Time Counter High"]
    #[inline(always)]
    #[must_use]
    pub fn pcth(&mut self) -> PcthW<PcthSpec> {
        PcthW::new(self, 0)
    }
}
#[doc = "UART CEA709.1-B Packet Cycle Time Counter High\n\nYou can [`read`](crate::Reg::read) this register and get [`pcth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcthSpec;
impl crate::RegisterSpec for PcthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcth::R`](R) reader structure"]
impl crate::Readable for PcthSpec {}
#[doc = "`write(|w| ..)` method takes [`pcth::W`](W) writer structure"]
impl crate::Writable for PcthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PCTH to value 0"]
impl crate::Resettable for PcthSpec {
    const RESET_VALUE: u8 = 0;
}
