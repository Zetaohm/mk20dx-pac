#[doc = "Register `GPOLYHU` reader"]
pub type R = crate::R<GpolyhuSpec>;
#[doc = "Register `GPOLYHU` writer"]
pub type W = crate::W<GpolyhuSpec>;
#[doc = "Field `GPOLYHU` reader - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
pub type GpolyhuR = crate::FieldReader;
#[doc = "Field `GPOLYHU` writer - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
pub type GpolyhuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhu(&self) -> GpolyhuR {
        GpolyhuR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn gpolyhu(&mut self) -> GpolyhuW<GpolyhuSpec> {
        GpolyhuW::new(self, 0)
    }
}
#[doc = "CRC_GPOLYHU register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpolyhu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpolyhu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpolyhuSpec;
impl crate::RegisterSpec for GpolyhuSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gpolyhu::R`](R) reader structure"]
impl crate::Readable for GpolyhuSpec {}
#[doc = "`write(|w| ..)` method takes [`gpolyhu::W`](W) writer structure"]
impl crate::Writable for GpolyhuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GPOLYHU to value 0xff"]
impl crate::Resettable for GpolyhuSpec {
    const RESET_VALUE: u8 = 0xff;
}
