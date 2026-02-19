#[doc = "Register `FOPT` reader"]
pub type R = crate::R<FoptSpec>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpboot {
    #[doc = "0: Low-power boot"]
    _00 = 0,
    #[doc = "1: Normal boot"]
    _01 = 1,
}
impl From<Lpboot> for bool {
    #[inline(always)]
    fn from(variant: Lpboot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBOOT` reader - no description available"]
pub type LpbootR = crate::BitReader<Lpboot>;
impl LpbootR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpboot {
        match self.bits {
            false => Lpboot::_00,
            true => Lpboot::_01,
        }
    }
    #[doc = "Low-power boot"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Lpboot::_00
    }
    #[doc = "Normal boot"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Lpboot::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EzportDis {
    #[doc = "0: EzPort operation is disabled"]
    _00 = 0,
    #[doc = "1: EzPort operation is enabled"]
    _01 = 1,
}
impl From<EzportDis> for bool {
    #[inline(always)]
    fn from(variant: EzportDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EZPORT_DIS` reader - no description available"]
pub type EzportDisR = crate::BitReader<EzportDis>;
impl EzportDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EzportDis {
        match self.bits {
            false => EzportDis::_00,
            true => EzportDis::_01,
        }
    }
    #[doc = "EzPort operation is disabled"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EzportDis::_00
    }
    #[doc = "EzPort operation is enabled"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EzportDis::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiDis {
    #[doc = "0: NMI interrupts are always blocked"]
    _00 = 0,
    #[doc = "1: NMI_b pin/interrupts reset default to enabled"]
    _01 = 1,
}
impl From<NmiDis> for bool {
    #[inline(always)]
    fn from(variant: NmiDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMI_DIS` reader - no description available"]
pub type NmiDisR = crate::BitReader<NmiDis>;
impl NmiDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NmiDis {
        match self.bits {
            false => NmiDis::_00,
            true => NmiDis::_01,
        }
    }
    #[doc = "NMI interrupts are always blocked"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NmiDis::_00
    }
    #[doc = "NMI_b pin/interrupts reset default to enabled"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NmiDis::_01
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn lpboot(&self) -> LpbootR {
        LpbootR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn ezport_dis(&self) -> EzportDisR {
        EzportDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nmi_dis(&self) -> NmiDisR {
        NmiDisR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Non-volatile Flash Option Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fopt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FoptSpec;
impl crate::RegisterSpec for FoptSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fopt::R`](R) reader structure"]
impl crate::Readable for FoptSpec {}
#[doc = "`reset()` method sets FOPT to value 0xff"]
impl crate::Resettable for FoptSpec {
    const RESET_VALUE: u8 = 0xff;
}
