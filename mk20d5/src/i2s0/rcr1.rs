#[doc = "Register `RCR1` reader"]
pub type R = crate::R<Rcr1Spec>;
#[doc = "Register `RCR1` writer"]
pub type W = crate::W<Rcr1Spec>;
#[doc = "Field `RFW` reader - Receive FIFO watermark"]
pub type RfwR = crate::FieldReader;
#[doc = "Field `RFW` writer - Receive FIFO watermark"]
pub type RfwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Receive FIFO watermark"]
    #[inline(always)]
    pub fn rfw(&self) -> RfwR {
        RfwR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rfw(&mut self) -> RfwW<Rcr1Spec> {
        RfwW::new(self, 0)
    }
}
#[doc = "SAI Receive Configuration 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcr1Spec;
impl crate::RegisterSpec for Rcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr1::R`](R) reader structure"]
impl crate::Readable for Rcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcr1::W`](W) writer structure"]
impl crate::Writable for Rcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR1 to value 0"]
impl crate::Resettable for Rcr1Spec {
    const RESET_VALUE: u32 = 0;
}
