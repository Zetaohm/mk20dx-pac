#[doc = "Register `TWFIFO` reader"]
pub type R = crate::R<TwfifoSpec>;
#[doc = "Register `TWFIFO` writer"]
pub type W = crate::W<TwfifoSpec>;
#[doc = "Field `TXWATER` reader - Transmit Watermark"]
pub type TxwaterR = crate::FieldReader;
#[doc = "Field `TXWATER` writer - Transmit Watermark"]
pub type TxwaterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Watermark"]
    #[inline(always)]
    pub fn txwater(&self) -> TxwaterR {
        TxwaterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn txwater(&mut self) -> TxwaterW<TwfifoSpec> {
        TxwaterW::new(self, 0)
    }
}
#[doc = "UART FIFO Transmit Watermark\n\nYou can [`read`](crate::Reg::read) this register and get [`twfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwfifoSpec;
impl crate::RegisterSpec for TwfifoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twfifo::R`](R) reader structure"]
impl crate::Readable for TwfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`twfifo::W`](W) writer structure"]
impl crate::Writable for TwfifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TWFIFO to value 0"]
impl crate::Resettable for TwfifoSpec {
    const RESET_VALUE: u8 = 0;
}
