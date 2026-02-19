#[doc = "Register `BDL` reader"]
pub type R = crate::R<BdlSpec>;
#[doc = "Register `BDL` writer"]
pub type W = crate::W<BdlSpec>;
#[doc = "Field `SBR` reader - UART Baud Rate Bits"]
pub type SbrR = crate::FieldReader;
#[doc = "Field `SBR` writer - UART Baud Rate Bits"]
pub type SbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - UART Baud Rate Bits"]
    #[inline(always)]
    pub fn sbr(&self) -> SbrR {
        SbrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART Baud Rate Bits"]
    #[inline(always)]
    #[must_use]
    pub fn sbr(&mut self) -> SbrW<BdlSpec> {
        SbrW::new(self, 0)
    }
}
#[doc = "UART Baud Rate Registers: Low\n\nYou can [`read`](crate::Reg::read) this register and get [`bdl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdlSpec;
impl crate::RegisterSpec for BdlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bdl::R`](R) reader structure"]
impl crate::Readable for BdlSpec {}
#[doc = "`write(|w| ..)` method takes [`bdl::W`](W) writer structure"]
impl crate::Writable for BdlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BDL to value 0x04"]
impl crate::Resettable for BdlSpec {
    const RESET_VALUE: u8 = 0x04;
}
