#[doc = "Register `CHDLY1` reader"]
pub type R = crate::R<Chdly1Spec>;
#[doc = "Register `CHDLY1` writer"]
pub type W = crate::W<Chdly1Spec>;
#[doc = "Field `DLY` reader - PDB Channel Delay"]
pub type DlyR = crate::FieldReader<u16>;
#[doc = "Field `DLY` writer - PDB Channel Delay"]
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PDB Channel Delay"]
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Channel Delay"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DlyW<Chdly1Spec> {
        DlyW::new(self, 0)
    }
}
#[doc = "Channel n Delay 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chdly1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdly1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chdly1Spec;
impl crate::RegisterSpec for Chdly1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdly1::R`](R) reader structure"]
impl crate::Readable for Chdly1Spec {}
#[doc = "`write(|w| ..)` method takes [`chdly1::W`](W) writer structure"]
impl crate::Writable for Chdly1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHDLY1 to value 0"]
impl crate::Resettable for Chdly1Spec {
    const RESET_VALUE: u32 = 0;
}
