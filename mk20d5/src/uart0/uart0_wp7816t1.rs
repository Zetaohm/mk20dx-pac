#[doc = "Register `WP7816T1` reader"]
pub type R = crate::R<Uart0Wp7816t1Spec>;
#[doc = "Register `WP7816T1` writer"]
pub type W = crate::W<Uart0Wp7816t1Spec>;
#[doc = "Field `BWI` reader - Block Wait Time Integer(C7816\\[TTYPE\\]
= 1)"]
pub type BwiR = crate::FieldReader;
#[doc = "Field `BWI` writer - Block Wait Time Integer(C7816\\[TTYPE\\]
= 1)"]
pub type BwiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CWI` reader - Character Wait Time Integer (C7816\\[TTYPE\\]
= 1)"]
pub type CwiR = crate::FieldReader;
#[doc = "Field `CWI` writer - Character Wait Time Integer (C7816\\[TTYPE\\]
= 1)"]
pub type CwiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Block Wait Time Integer(C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    pub fn bwi(&self) -> BwiR {
        BwiR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Character Wait Time Integer (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    pub fn cwi(&self) -> CwiR {
        CwiR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Block Wait Time Integer(C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    #[must_use]
    pub fn bwi(&mut self) -> BwiW<Uart0Wp7816t1Spec> {
        BwiW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Character Wait Time Integer (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    #[must_use]
    pub fn cwi(&mut self) -> CwiW<Uart0Wp7816t1Spec> {
        CwiW::new(self, 4)
    }
}
#[doc = "UART 7816 Wait Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_wp7816t1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_wp7816t1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart0Wp7816t1Spec;
impl crate::RegisterSpec for Uart0Wp7816t1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart0_wp7816t1::R`](R) reader structure"]
impl crate::Readable for Uart0Wp7816t1Spec {}
#[doc = "`write(|w| ..)` method takes [`uart0_wp7816t1::W`](W) writer structure"]
impl crate::Writable for Uart0Wp7816t1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WP7816T1 to value 0x0a"]
impl crate::Resettable for Uart0Wp7816t1Spec {
    const RESET_VALUE: u8 = 0x0a;
}
