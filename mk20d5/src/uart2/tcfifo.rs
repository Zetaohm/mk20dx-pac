#[doc = "Register `TCFIFO` reader"]
pub type R = crate::R<TcfifoSpec>;
#[doc = "Field `TXCOUNT` reader - Transmit Counter"]
pub type TxcountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Counter"]
    #[inline(always)]
    pub fn txcount(&self) -> TxcountR {
        TxcountR::new(self.bits)
    }
}
#[doc = "UART FIFO Transmit Count\n\nYou can [`read`](crate::Reg::read) this register and get [`tcfifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcfifoSpec;
impl crate::RegisterSpec for TcfifoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcfifo::R`](R) reader structure"]
impl crate::Readable for TcfifoSpec {}
#[doc = "`reset()` method sets TCFIFO to value 0"]
impl crate::Resettable for TcfifoSpec {
    const RESET_VALUE: u8 = 0;
}
