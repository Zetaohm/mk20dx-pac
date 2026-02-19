#[doc = "Register `TCD%s_DOFF` reader"]
pub type R = crate::R<TcdDoffSpec>;
#[doc = "Register `TCD%s_DOFF` writer"]
pub type W = crate::W<TcdDoffSpec>;
#[doc = "Field `DOFF` reader - Destination Address Signed offset"]
pub type DoffR = crate::FieldReader<u16>;
#[doc = "Field `DOFF` writer - Destination Address Signed offset"]
pub type DoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Destination Address Signed offset"]
    #[inline(always)]
    pub fn doff(&self) -> DoffR {
        DoffR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Destination Address Signed offset"]
    #[inline(always)]
    #[must_use]
    pub fn doff(&mut self) -> DoffW<TcdDoffSpec> {
        DoffW::new(self, 0)
    }
}
#[doc = "TCD Signed Destination Address Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_doff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_doff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcdDoffSpec;
impl crate::RegisterSpec for TcdDoffSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcd_doff::R`](R) reader structure"]
impl crate::Readable for TcdDoffSpec {}
#[doc = "`write(|w| ..)` method takes [`tcd_doff::W`](W) writer structure"]
impl crate::Writable for TcdDoffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TCD%s_DOFF to value 0"]
impl crate::Resettable for TcdDoffSpec {
    const RESET_VALUE: u16 = 0;
}
