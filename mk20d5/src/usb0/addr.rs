#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `ADDR` reader - USB address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - USB address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LSEN` reader - Low Speed Enable bit"]
pub type LsenR = crate::BitReader;
#[doc = "Field `LSEN` writer - Low Speed Enable bit"]
pub type LsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - USB address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Low Speed Enable bit"]
    #[inline(always)]
    pub fn lsen(&self) -> LsenR {
        LsenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<AddrSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 7 - Low Speed Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lsen(&mut self) -> LsenW<AddrSpec> {
        LsenW::new(self, 7)
    }
}
#[doc = "Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u8 = 0;
}
