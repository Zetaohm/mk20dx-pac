#[doc = "Register `ISFR` reader"]
pub type R = crate::R<IsfrSpec>;
#[doc = "Register `ISFR` writer"]
pub type W = crate::W<IsfrSpec>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Isf {
    #[doc = "0: Configured interrupt has not been detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to the flag. If configured for a level sensitive interrupt and the pin remains asserted then the flag will set again immediately after it is cleared."]
    _1 = 1,
}
impl From<Isf> for u32 {
    #[inline(always)]
    fn from(variant: Isf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Isf {
    type Ux = u32;
}
impl crate::IsEnum for Isf {}
#[doc = "Field `ISF` reader - Interrupt Status Flag"]
pub type IsfR = crate::FieldReader<Isf>;
impl IsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Isf> {
        match self.bits {
            0 => Some(Isf::_0),
            1 => Some(Isf::_1),
            _ => None,
        }
    }
    #[doc = "Configured interrupt has not been detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isf::_0
    }
    #[doc = "Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to the flag. If configured for a level sensitive interrupt and the pin remains asserted then the flag will set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isf::_1
    }
}
#[doc = "Field `ISF` writer - Interrupt Status Flag"]
pub type IsfW<'a, REG> = crate::FieldWriter<'a, REG, 32, Isf>;
impl<'a, REG> IsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Configured interrupt has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::_0)
    }
    #[doc = "Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to the flag. If configured for a level sensitive interrupt and the pin remains asserted then the flag will set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> IsfR {
        IsfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf(&mut self) -> IsfW<IsfrSpec> {
        IsfW::new(self, 0)
    }
}
#[doc = "Interrupt Status Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsfrSpec;
impl crate::RegisterSpec for IsfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isfr::R`](R) reader structure"]
impl crate::Readable for IsfrSpec {}
#[doc = "`write(|w| ..)` method takes [`isfr::W`](W) writer structure"]
impl crate::Writable for IsfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISFR to value 0"]
impl crate::Resettable for IsfrSpec {
    const RESET_VALUE: u32 = 0;
}
