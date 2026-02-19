#[doc = "Register `MA1` reader"]
pub type R = crate::R<Ma1Spec>;
#[doc = "Register `MA1` writer"]
pub type W = crate::W<Ma1Spec>;
#[doc = "Field `MA` reader - Match Address"]
pub type MaR = crate::FieldReader;
#[doc = "Field `MA` writer - Match Address"]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Match Address"]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match Address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MaW<Ma1Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "UART Match Address Registers 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ma1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ma1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ma1Spec;
impl crate::RegisterSpec for Ma1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ma1::R`](R) reader structure"]
impl crate::Readable for Ma1Spec {}
#[doc = "`write(|w| ..)` method takes [`ma1::W`](W) writer structure"]
impl crate::Writable for Ma1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MA1 to value 0"]
impl crate::Resettable for Ma1Spec {
    const RESET_VALUE: u8 = 0;
}
