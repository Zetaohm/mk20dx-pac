#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EWMEN` reader - EWM enable."]
pub type EwmenR = crate::BitReader;
#[doc = "Field `EWMEN` writer - EWM enable."]
pub type EwmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASSIN` reader - EWM_in's Assertion State Select."]
pub type AssinR = crate::BitReader;
#[doc = "Field `ASSIN` writer - EWM_in's Assertion State Select."]
pub type AssinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN` reader - Input Enable."]
pub type InenR = crate::BitReader;
#[doc = "Field `INEN` writer - Input Enable."]
pub type InenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - Interrupt Enable."]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Interrupt Enable."]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EWM enable."]
    #[inline(always)]
    pub fn ewmen(&self) -> EwmenR {
        EwmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EWM_in's Assertion State Select."]
    #[inline(always)]
    pub fn assin(&self) -> AssinR {
        AssinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Enable."]
    #[inline(always)]
    pub fn inen(&self) -> InenR {
        InenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable."]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EWM enable."]
    #[inline(always)]
    #[must_use]
    pub fn ewmen(&mut self) -> EwmenW<CtrlSpec> {
        EwmenW::new(self, 0)
    }
    #[doc = "Bit 1 - EWM_in's Assertion State Select."]
    #[inline(always)]
    #[must_use]
    pub fn assin(&mut self) -> AssinW<CtrlSpec> {
        AssinW::new(self, 1)
    }
    #[doc = "Bit 2 - Input Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inen(&mut self) -> InenW<CtrlSpec> {
        InenW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<CtrlSpec> {
        IntenW::new(self, 3)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u8 = 0;
}
