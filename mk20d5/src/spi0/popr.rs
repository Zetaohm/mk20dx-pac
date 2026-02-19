#[doc = "Register `POPR` reader"]
pub type R = crate::R<PoprSpec>;
#[doc = "Field `RXDATA` reader - Received Data"]
pub type RxdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new(self.bits)
    }
}
#[doc = "DSPI POP RX FIFO Register\n\nYou can [`read`](crate::Reg::read) this register and get [`popr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PoprSpec;
impl crate::RegisterSpec for PoprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`popr::R`](R) reader structure"]
impl crate::Readable for PoprSpec {}
#[doc = "`reset()` method sets POPR to value 0"]
impl crate::Resettable for PoprSpec {
    const RESET_VALUE: u32 = 0;
}
