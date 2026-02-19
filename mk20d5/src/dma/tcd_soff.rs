#[doc = "Register `TCD%s_SOFF` reader"]
pub type R = crate::R<TcdSoffSpec>;
#[doc = "Register `TCD%s_SOFF` writer"]
pub type W = crate::W<TcdSoffSpec>;
#[doc = "Field `SOFF` reader - Source address signed offset"]
pub type SoffR = crate::FieldReader<u16>;
#[doc = "Field `SOFF` writer - Source address signed offset"]
pub type SoffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source address signed offset"]
    #[inline(always)]
    pub fn soff(&self) -> SoffR {
        SoffR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source address signed offset"]
    #[inline(always)]
    #[must_use]
    pub fn soff(&mut self) -> SoffW<TcdSoffSpec> {
        SoffW::new(self, 0)
    }
}
#[doc = "TCD Signed Source Address Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_soff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_soff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcdSoffSpec;
impl crate::RegisterSpec for TcdSoffSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcd_soff::R`](R) reader structure"]
impl crate::Readable for TcdSoffSpec {}
#[doc = "`write(|w| ..)` method takes [`tcd_soff::W`](W) writer structure"]
impl crate::Writable for TcdSoffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TCD%s_SOFF to value 0"]
impl crate::Resettable for TcdSoffSpec {
    const RESET_VALUE: u16 = 0;
}
