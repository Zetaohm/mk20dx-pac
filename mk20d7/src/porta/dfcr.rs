#[doc = "Register `DFCR` reader"]
pub type R = crate::R<DfcrSpec>;
#[doc = "Register `DFCR` writer"]
pub type W = crate::W<DfcrSpec>;
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cs {
    #[doc = "0: Digital Filters are clocked by the bus clock."]
    _0 = 0,
    #[doc = "1: Digital Filters are clocked by the 1 kHz LPO clock."]
    _1 = 1,
}
impl From<Cs> for bool {
    #[inline(always)]
    fn from(variant: Cs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS` reader - Clock Source"]
pub type CsR = crate::BitReader<Cs>;
impl CsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cs {
        match self.bits {
            false => Cs::_0,
            true => Cs::_1,
        }
    }
    #[doc = "Digital Filters are clocked by the bus clock."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cs::_0
    }
    #[doc = "Digital Filters are clocked by the 1 kHz LPO clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cs::_1
    }
}
#[doc = "Field `CS` writer - Clock Source"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG, Cs>;
impl<'a, REG> CsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital Filters are clocked by the bus clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::_0)
    }
    #[doc = "Digital Filters are clocked by the 1 kHz LPO clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<DfcrSpec> {
        CsW::new(self, 0)
    }
}
#[doc = "Digital Filter Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfcrSpec;
impl crate::RegisterSpec for DfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfcr::R`](R) reader structure"]
impl crate::Readable for DfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfcr::W`](W) writer structure"]
impl crate::Writable for DfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFCR to value 0"]
impl crate::Resettable for DfcrSpec {
    const RESET_VALUE: u32 = 0;
}
