#[doc = "Register `SRS1` reader"]
pub type R = crate::R<Srs1Spec>;
#[doc = "JTAG generated reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jtag {
    #[doc = "0: Reset not caused by JTAG"]
    _0 = 0,
    #[doc = "1: Reset caused by JTAG"]
    _1 = 1,
}
impl From<Jtag> for bool {
    #[inline(always)]
    fn from(variant: Jtag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JTAG` reader - JTAG generated reset"]
pub type JtagR = crate::BitReader<Jtag>;
impl JtagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jtag {
        match self.bits {
            false => Jtag::_0,
            true => Jtag::_1,
        }
    }
    #[doc = "Reset not caused by JTAG"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Jtag::_0
    }
    #[doc = "Reset caused by JTAG"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Jtag::_1
    }
}
#[doc = "Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockup {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    _0 = 0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    _1 = 1,
}
impl From<Lockup> for bool {
    #[inline(always)]
    fn from(variant: Lockup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP` reader - Core Lockup"]
pub type LockupR = crate::BitReader<Lockup>;
impl LockupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockup {
        match self.bits {
            false => Lockup::_0,
            true => Lockup::_1,
        }
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lockup::_0
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lockup::_1
    }
}
#[doc = "Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sw {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    _0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    _1 = 1,
}
impl From<Sw> for bool {
    #[inline(always)]
    fn from(variant: Sw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW` reader - Software"]
pub type SwR = crate::BitReader<Sw>;
impl SwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sw {
        match self.bits {
            false => Sw::_0,
            true => Sw::_1,
        }
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sw::_0
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sw::_1
    }
}
#[doc = "MDM-AP system reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MdmAp {
    #[doc = "0: Reset not caused by host debugger system setting of the System Reset Request bit"]
    _0 = 0,
    #[doc = "1: Reset caused by host debugger system setting of the System Reset Request bit"]
    _1 = 1,
}
impl From<MdmAp> for bool {
    #[inline(always)]
    fn from(variant: MdmAp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDM_AP` reader - MDM-AP system reset request"]
pub type MdmApR = crate::BitReader<MdmAp>;
impl MdmApR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MdmAp {
        match self.bits {
            false => MdmAp::_0,
            true => MdmAp::_1,
        }
    }
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MdmAp::_0
    }
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MdmAp::_1
    }
}
#[doc = "EzPort Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ezpt {
    #[doc = "0: Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _0 = 0,
    #[doc = "1: Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _1 = 1,
}
impl From<Ezpt> for bool {
    #[inline(always)]
    fn from(variant: Ezpt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EZPT` reader - EzPort Reset"]
pub type EzptR = crate::BitReader<Ezpt>;
impl EzptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ezpt {
        match self.bits {
            false => Ezpt::_0,
            true => Ezpt::_1,
        }
    }
    #[doc = "Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ezpt::_0
    }
    #[doc = "Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ezpt::_1
    }
}
#[doc = "Stop Mode Acknowledge Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sackerr {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0 = 0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1 = 1,
}
impl From<Sackerr> for bool {
    #[inline(always)]
    fn from(variant: Sackerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKERR` reader - Stop Mode Acknowledge Error Reset"]
pub type SackerrR = crate::BitReader<Sackerr>;
impl SackerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sackerr {
        match self.bits {
            false => Sackerr::_0,
            true => Sackerr::_1,
        }
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sackerr::_0
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sackerr::_1
    }
}
impl R {
    #[doc = "Bit 0 - JTAG generated reset"]
    #[inline(always)]
    pub fn jtag(&self) -> JtagR {
        JtagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Lockup"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MDM-AP system reset request"]
    #[inline(always)]
    pub fn mdm_ap(&self) -> MdmApR {
        MdmApR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EzPort Reset"]
    #[inline(always)]
    pub fn ezpt(&self) -> EzptR {
        EzptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn sackerr(&self) -> SackerrR {
        SackerrR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "System Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`srs1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srs1Spec;
impl crate::RegisterSpec for Srs1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`srs1::R`](R) reader structure"]
impl crate::Readable for Srs1Spec {}
#[doc = "`reset()` method sets SRS1 to value 0"]
impl crate::Resettable for Srs1Spec {
    const RESET_VALUE: u8 = 0;
}
