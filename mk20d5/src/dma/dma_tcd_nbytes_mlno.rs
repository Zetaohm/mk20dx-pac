#[doc = "Register `TCD%s_NBYTES_MLNO` reader"]
pub type R = crate::R<DmaTcdNbytesMlnoSpec>;
#[doc = "Register `TCD%s_NBYTES_MLNO` writer"]
pub type W = crate::W<DmaTcdNbytesMlnoSpec>;
#[doc = "Field `NBYTES` reader - Minor Byte Transfer Count"]
pub type NbytesR = crate::FieldReader<u32>;
#[doc = "Field `NBYTES` writer - Minor Byte Transfer Count"]
pub type NbytesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&self) -> NbytesR {
        NbytesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Minor Byte Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NbytesW<DmaTcdNbytesMlnoSpec> {
        NbytesW::new(self, 0)
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tcd_nbytes_mlno::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tcd_nbytes_mlno::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaTcdNbytesMlnoSpec;
impl crate::RegisterSpec for DmaTcdNbytesMlnoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tcd_nbytes_mlno::R`](R) reader structure"]
impl crate::Readable for DmaTcdNbytesMlnoSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_tcd_nbytes_mlno::W`](W) writer structure"]
impl crate::Writable for DmaTcdNbytesMlnoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCD%s_NBYTES_MLNO to value 0"]
impl crate::Resettable for DmaTcdNbytesMlnoSpec {
    const RESET_VALUE: u32 = 0;
}
