#[doc = "Register `RWFIFO` reader"]
pub type R = crate::R<RwfifoSpec>;
#[doc = "Register `RWFIFO` writer"]
pub type W = crate::W<RwfifoSpec>;
#[doc = "Field `RXWATER` reader - Receive Watermark"]
pub type RxwaterR = crate::FieldReader;
#[doc = "Field `RXWATER` writer - Receive Watermark"]
pub type RxwaterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Watermark"]
    #[inline(always)]
    pub fn rxwater(&self) -> RxwaterR {
        RxwaterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rxwater(&mut self) -> RxwaterW<RwfifoSpec> {
        RxwaterW::new(self, 0)
    }
}
#[doc = "UART FIFO Receive Watermark\n\nYou can [`read`](crate::Reg::read) this register and get [`rwfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RwfifoSpec;
impl crate::RegisterSpec for RwfifoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rwfifo::R`](R) reader structure"]
impl crate::Readable for RwfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`rwfifo::W`](W) writer structure"]
impl crate::Writable for RwfifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RWFIFO to value 0x01"]
impl crate::Resettable for RwfifoSpec {
    const RESET_VALUE: u8 = 0x01;
}
