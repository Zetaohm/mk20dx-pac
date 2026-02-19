#[doc = "Register `PMCTRL` reader"]
pub type R = crate::R<PmctrlSpec>;
#[doc = "Register `PMCTRL` writer"]
pub type W = crate::W<PmctrlSpec>;
#[doc = "Stop Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stopm {
    #[doc = "0: Normal stop (STOP)"]
    _000 = 0,
    #[doc = "2: Very low power stop (VLPS)"]
    _010 = 2,
    #[doc = "3: Low leakage stop (LLS)"]
    _011 = 3,
    #[doc = "4: Very low leakage stop (VLLSx)"]
    _100 = 4,
    #[doc = "6: Reseved"]
    _110 = 6,
}
impl From<Stopm> for u8 {
    #[inline(always)]
    fn from(variant: Stopm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stopm {
    type Ux = u8;
}
impl crate::IsEnum for Stopm {}
#[doc = "Field `STOPM` reader - Stop Mode Control"]
pub type StopmR = crate::FieldReader<Stopm>;
impl StopmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stopm> {
        match self.bits {
            0 => Some(Stopm::_000),
            2 => Some(Stopm::_010),
            3 => Some(Stopm::_011),
            4 => Some(Stopm::_100),
            6 => Some(Stopm::_110),
            _ => None,
        }
    }
    #[doc = "Normal stop (STOP)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Stopm::_000
    }
    #[doc = "Very low power stop (VLPS)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Stopm::_010
    }
    #[doc = "Low leakage stop (LLS)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Stopm::_011
    }
    #[doc = "Very low leakage stop (VLLSx)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Stopm::_100
    }
    #[doc = "Reseved"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Stopm::_110
    }
}
#[doc = "Field `STOPM` writer - Stop Mode Control"]
pub type StopmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Stopm>;
impl<'a, REG> StopmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal stop (STOP)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Stopm::_000)
    }
    #[doc = "Very low power stop (VLPS)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Stopm::_010)
    }
    #[doc = "Low leakage stop (LLS)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Stopm::_011)
    }
    #[doc = "Very low leakage stop (VLLSx)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Stopm::_100)
    }
    #[doc = "Reseved"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Stopm::_110)
    }
}
#[doc = "Stop Aborted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopa {
    #[doc = "0: The previous stop mode entry was successsful."]
    _0 = 0,
    #[doc = "1: The previous stop mode entry was aborted."]
    _1 = 1,
}
impl From<Stopa> for bool {
    #[inline(always)]
    fn from(variant: Stopa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPA` reader - Stop Aborted"]
pub type StopaR = crate::BitReader<Stopa>;
impl StopaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopa {
        match self.bits {
            false => Stopa::_0,
            true => Stopa::_1,
        }
    }
    #[doc = "The previous stop mode entry was successsful."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stopa::_0
    }
    #[doc = "The previous stop mode entry was aborted."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stopa::_1
    }
}
#[doc = "Run Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Runm {
    #[doc = "0: Normal run mode (RUN)"]
    _00 = 0,
    #[doc = "2: Very low power run mode (VLPR)"]
    _10 = 2,
}
impl From<Runm> for u8 {
    #[inline(always)]
    fn from(variant: Runm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Runm {
    type Ux = u8;
}
impl crate::IsEnum for Runm {}
#[doc = "Field `RUNM` reader - Run Mode Control"]
pub type RunmR = crate::FieldReader<Runm>;
impl RunmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Runm> {
        match self.bits {
            0 => Some(Runm::_00),
            2 => Some(Runm::_10),
            _ => None,
        }
    }
    #[doc = "Normal run mode (RUN)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Runm::_00
    }
    #[doc = "Very low power run mode (VLPR)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Runm::_10
    }
}
#[doc = "Field `RUNM` writer - Run Mode Control"]
pub type RunmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Runm>;
impl<'a, REG> RunmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal run mode (RUN)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Runm::_00)
    }
    #[doc = "Very low power run mode (VLPR)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Runm::_10)
    }
}
impl R {
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline(always)]
    pub fn stopm(&self) -> StopmR {
        StopmR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Stop Aborted"]
    #[inline(always)]
    pub fn stopa(&self) -> StopaR {
        StopaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline(always)]
    pub fn runm(&self) -> RunmR {
        RunmR::new((self.bits >> 5) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn stopm(&mut self) -> StopmW<PmctrlSpec> {
        StopmW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn runm(&mut self) -> RunmW<PmctrlSpec> {
        RunmW::new(self, 5)
    }
}
#[doc = "Power Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmctrlSpec;
impl crate::RegisterSpec for PmctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmctrl::R`](R) reader structure"]
impl crate::Readable for PmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pmctrl::W`](W) writer structure"]
impl crate::Writable for PmctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PMCTRL to value 0"]
impl crate::Resettable for PmctrlSpec {
    const RESET_VALUE: u8 = 0;
}
