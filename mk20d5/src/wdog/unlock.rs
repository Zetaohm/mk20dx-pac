#[doc = "Register `UNLOCK` reader"]
pub type R = crate::R<UnlockSpec>;
#[doc = "Register `UNLOCK` writer"]
pub type W = crate::W<UnlockSpec>;
#[doc = "Field `WDOGUNLOCK` reader - You can write the unlock sequence values to this register to make the watchdog write once registers writable again"]
pub type WdogunlockR = crate::FieldReader<u16>;
#[doc = "Field `WDOGUNLOCK` writer - You can write the unlock sequence values to this register to make the watchdog write once registers writable again"]
pub type WdogunlockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - You can write the unlock sequence values to this register to make the watchdog write once registers writable again"]
    #[inline(always)]
    pub fn wdogunlock(&self) -> WdogunlockR {
        WdogunlockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - You can write the unlock sequence values to this register to make the watchdog write once registers writable again"]
    #[inline(always)]
    #[must_use]
    pub fn wdogunlock(&mut self) -> WdogunlockW<UnlockSpec> {
        WdogunlockW::new(self, 0)
    }
}
#[doc = "Watchdog Unlock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`unlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UnlockSpec;
impl crate::RegisterSpec for UnlockSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`unlock::R`](R) reader structure"]
impl crate::Readable for UnlockSpec {}
#[doc = "`write(|w| ..)` method takes [`unlock::W`](W) writer structure"]
impl crate::Writable for UnlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UNLOCK to value 0xd928"]
impl crate::Resettable for UnlockSpec {
    const RESET_VALUE: u16 = 0xd928;
}
