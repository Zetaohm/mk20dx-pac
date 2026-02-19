#[doc = "Register `PRESC` reader"]
pub type R = crate::R<PrescSpec>;
#[doc = "Register `PRESC` writer"]
pub type W = crate::W<PrescSpec>;
#[doc = "Field `PRESCVAL` reader - 3-bit prescaler for the watchdog clock source"]
pub type PrescvalR = crate::FieldReader;
#[doc = "Field `PRESCVAL` writer - 3-bit prescaler for the watchdog clock source"]
pub type PrescvalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 8:10 - 3-bit prescaler for the watchdog clock source"]
    #[inline(always)]
    pub fn prescval(&self) -> PrescvalR {
        PrescvalR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - 3-bit prescaler for the watchdog clock source"]
    #[inline(always)]
    #[must_use]
    pub fn prescval(&mut self) -> PrescvalW<PrescSpec> {
        PrescvalW::new(self, 8)
    }
}
#[doc = "Watchdog Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescSpec;
impl crate::RegisterSpec for PrescSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`presc::R`](R) reader structure"]
impl crate::Readable for PrescSpec {}
#[doc = "`write(|w| ..)` method takes [`presc::W`](W) writer structure"]
impl crate::Writable for PrescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PRESC to value 0x0400"]
impl crate::Resettable for PrescSpec {
    const RESET_VALUE: u16 = 0x0400;
}
