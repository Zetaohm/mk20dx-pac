#[doc = "Register `IDLY` reader"]
pub type R = crate::R<IdlySpec>;
#[doc = "Register `IDLY` writer"]
pub type W = crate::W<IdlySpec>;
#[doc = "Field `IDLY` reader - PDB Interrupt Delay"]
pub type IdlyR = crate::FieldReader<u16>;
#[doc = "Field `IDLY` writer - PDB Interrupt Delay"]
pub type IdlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PDB Interrupt Delay"]
    #[inline(always)]
    pub fn idly(&self) -> IdlyR {
        IdlyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Interrupt Delay"]
    #[inline(always)]
    #[must_use]
    pub fn idly(&mut self) -> IdlyW<IdlySpec> {
        IdlyW::new(self, 0)
    }
}
#[doc = "Interrupt Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdlySpec;
impl crate::RegisterSpec for IdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idly::R`](R) reader structure"]
impl crate::Readable for IdlySpec {}
#[doc = "`write(|w| ..)` method takes [`idly::W`](W) writer structure"]
impl crate::Writable for IdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDLY to value 0xffff"]
impl crate::Resettable for IdlySpec {
    const RESET_VALUE: u32 = 0xffff;
}
