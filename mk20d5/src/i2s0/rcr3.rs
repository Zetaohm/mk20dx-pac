#[doc = "Register `RCR3` reader"]
pub type R = crate::R<Rcr3Spec>;
#[doc = "Register `RCR3` writer"]
pub type W = crate::W<Rcr3Spec>;
#[doc = "Field `WDFL` reader - Word flag configuration"]
pub type WdflR = crate::FieldReader;
#[doc = "Field `WDFL` writer - Word flag configuration"]
pub type WdflW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RCE` reader - Receive channel enable"]
pub type RceR = crate::FieldReader;
#[doc = "Field `RCE` writer - Receive channel enable"]
pub type RceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - Word flag configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WdflR {
        WdflR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Receive channel enable"]
    #[inline(always)]
    pub fn rce(&self) -> RceR {
        RceR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word flag configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdfl(&mut self) -> WdflW<Rcr3Spec> {
        WdflW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Receive channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn rce(&mut self) -> RceW<Rcr3Spec> {
        RceW::new(self, 16)
    }
}
#[doc = "SAI Receive Configuration 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcr3Spec;
impl crate::RegisterSpec for Rcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr3::R`](R) reader structure"]
impl crate::Readable for Rcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`rcr3::W`](W) writer structure"]
impl crate::Writable for Rcr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR3 to value 0"]
impl crate::Resettable for Rcr3Spec {
    const RESET_VALUE: u32 = 0;
}
