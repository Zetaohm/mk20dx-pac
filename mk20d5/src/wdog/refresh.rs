#[doc = "Register `REFRESH` reader"]
pub type R = crate::R<RefreshSpec>;
#[doc = "Register `REFRESH` writer"]
pub type W = crate::W<RefreshSpec>;
#[doc = "Field `WDOGREFRESH` reader - Watchdog refresh register"]
pub type WdogrefreshR = crate::FieldReader<u16>;
#[doc = "Field `WDOGREFRESH` writer - Watchdog refresh register"]
pub type WdogrefreshW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Watchdog refresh register"]
    #[inline(always)]
    pub fn wdogrefresh(&self) -> WdogrefreshR {
        WdogrefreshR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog refresh register"]
    #[inline(always)]
    #[must_use]
    pub fn wdogrefresh(&mut self) -> WdogrefreshW<RefreshSpec> {
        WdogrefreshW::new(self, 0)
    }
}
#[doc = "Watchdog Refresh Register\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefreshSpec;
impl crate::RegisterSpec for RefreshSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`refresh::R`](R) reader structure"]
impl crate::Readable for RefreshSpec {}
#[doc = "`write(|w| ..)` method takes [`refresh::W`](W) writer structure"]
impl crate::Writable for RefreshSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REFRESH to value 0xb480"]
impl crate::Resettable for RefreshSpec {
    const RESET_VALUE: u16 = 0xb480;
}
