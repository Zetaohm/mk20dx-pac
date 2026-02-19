#[doc = "Register `FCCOB%s` reader"]
pub type R = crate::R<FccobSpec>;
#[doc = "Register `FCCOB%s` writer"]
pub type W = crate::W<FccobSpec>;
#[doc = "Field `CCOBn` reader - no description available"]
pub type CcobnR = crate::FieldReader;
#[doc = "Field `CCOBn` writer - no description available"]
pub type CcobnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn ccobn(&self) -> CcobnR {
        CcobnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ccobn(&mut self) -> CcobnW<FccobSpec> {
        CcobnW::new(self, 0)
    }
}
#[doc = "Flash Common Command Object Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FccobSpec;
impl crate::RegisterSpec for FccobSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fccob::R`](R) reader structure"]
impl crate::Readable for FccobSpec {}
#[doc = "`write(|w| ..)` method takes [`fccob::W`](W) writer structure"]
impl crate::Writable for FccobSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FCCOB%s to value 0"]
impl crate::Resettable for FccobSpec {
    const RESET_VALUE: u8 = 0;
}
