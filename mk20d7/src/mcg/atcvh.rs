#[doc = "Register `ATCVH` reader"]
pub type R = crate::R<AtcvhSpec>;
#[doc = "Register `ATCVH` writer"]
pub type W = crate::W<AtcvhSpec>;
#[doc = "Field `ATCVH` reader - ATM Compare Value High"]
pub type AtcvhR = crate::FieldReader;
#[doc = "Field `ATCVH` writer - ATM Compare Value High"]
pub type AtcvhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ATM Compare Value High"]
    #[inline(always)]
    pub fn atcvh(&self) -> AtcvhR {
        AtcvhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATM Compare Value High"]
    #[inline(always)]
    #[must_use]
    pub fn atcvh(&mut self) -> AtcvhW<AtcvhSpec> {
        AtcvhW::new(self, 0)
    }
}
#[doc = "MCG Auto Trim Compare Value High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`atcvh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcvh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtcvhSpec;
impl crate::RegisterSpec for AtcvhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`atcvh::R`](R) reader structure"]
impl crate::Readable for AtcvhSpec {}
#[doc = "`write(|w| ..)` method takes [`atcvh::W`](W) writer structure"]
impl crate::Writable for AtcvhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ATCVH to value 0"]
impl crate::Resettable for AtcvhSpec {
    const RESET_VALUE: u8 = 0;
}
