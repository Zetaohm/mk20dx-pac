#[doc = "Register `WP7816T0` reader"]
pub type R = crate::R<Uart0Wp7816t0Spec>;
#[doc = "Register `WP7816T0` writer"]
pub type W = crate::W<Uart0Wp7816t0Spec>;
#[doc = "Field `WI` reader - Wait Timer Interrupt (C7816\\[TTYPE\\]
= 0)"]
pub type WiR = crate::FieldReader;
#[doc = "Field `WI` writer - Wait Timer Interrupt (C7816\\[TTYPE\\]
= 0)"]
pub type WiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wait Timer Interrupt (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    pub fn wi(&self) -> WiR {
        WiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait Timer Interrupt (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    #[must_use]
    pub fn wi(&mut self) -> WiW<Uart0Wp7816t0Spec> {
        WiW::new(self, 0)
    }
}
#[doc = "UART 7816 Wait Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_wp7816t0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_wp7816t0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart0Wp7816t0Spec;
impl crate::RegisterSpec for Uart0Wp7816t0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart0_wp7816t0::R`](R) reader structure"]
impl crate::Readable for Uart0Wp7816t0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart0_wp7816t0::W`](W) writer structure"]
impl crate::Writable for Uart0Wp7816t0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WP7816T0 to value 0x0a"]
impl crate::Resettable for Uart0Wp7816t0Spec {
    const RESET_VALUE: u8 = 0x0a;
}
