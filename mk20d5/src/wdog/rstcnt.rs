#[doc = "Register `RSTCNT` reader"]
pub type R = crate::R<RstcntSpec>;
#[doc = "Register `RSTCNT` writer"]
pub type W = crate::W<RstcntSpec>;
#[doc = "Field `RSTCNT` reader - Counts the number of times the watchdog resets the system"]
pub type RstcntR = crate::FieldReader<u16>;
#[doc = "Field `RSTCNT` writer - Counts the number of times the watchdog resets the system"]
pub type RstcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counts the number of times the watchdog resets the system"]
    #[inline(always)]
    pub fn rstcnt(&self) -> RstcntR {
        RstcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counts the number of times the watchdog resets the system"]
    #[inline(always)]
    #[must_use]
    pub fn rstcnt(&mut self) -> RstcntW<RstcntSpec> {
        RstcntW::new(self, 0)
    }
}
#[doc = "Watchdog Reset Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcntSpec;
impl crate::RegisterSpec for RstcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rstcnt::R`](R) reader structure"]
impl crate::Readable for RstcntSpec {}
#[doc = "`write(|w| ..)` method takes [`rstcnt::W`](W) writer structure"]
impl crate::Writable for RstcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RSTCNT to value 0"]
impl crate::Resettable for RstcntSpec {
    const RESET_VALUE: u16 = 0;
}
