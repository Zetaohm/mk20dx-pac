#[doc = "Register `SRS0` reader"]
pub type R = crate::R<Srs0Spec>;
#[doc = "Low leakage wakeup reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakeup {
    #[doc = "0: Reset not caused by LLWU module wakeup source"]
    _0 = 0,
    #[doc = "1: Reset caused by LLWU module wakeup source"]
    _1 = 1,
}
impl From<Wakeup> for bool {
    #[inline(always)]
    fn from(variant: Wakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP` reader - Low leakage wakeup reset"]
pub type WakeupR = crate::BitReader<Wakeup>;
impl WakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakeup {
        match self.bits {
            false => Wakeup::_0,
            true => Wakeup::_1,
        }
    }
    #[doc = "Reset not caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wakeup::_0
    }
    #[doc = "Reset caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wakeup::_1
    }
}
#[doc = "Low-voltage detect reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    _0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    _1 = 1,
}
impl From<Lvd> for bool {
    #[inline(always)]
    fn from(variant: Lvd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD` reader - Low-voltage detect reset"]
pub type LvdR = crate::BitReader<Lvd>;
impl LvdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd {
        match self.bits {
            false => Lvd::_0,
            true => Lvd::_1,
        }
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd::_0
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd::_1
    }
}
#[doc = "Loss-of-clock reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loc {
    #[doc = "0: Reset not caused by a loss of external clock."]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of external clock."]
    _1 = 1,
}
impl From<Loc> for bool {
    #[inline(always)]
    fn from(variant: Loc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOC` reader - Loss-of-clock reset"]
pub type LocR = crate::BitReader<Loc>;
impl LocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loc {
        match self.bits {
            false => Loc::_0,
            true => Loc::_1,
        }
    }
    #[doc = "Reset not caused by a loss of external clock."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Loc::_0
    }
    #[doc = "Reset caused by a loss of external clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Loc::_1
    }
}
#[doc = "Loss-of-lock reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lol {
    #[doc = "0: Reset not caused by a loss of lock in the PLL"]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of lock in the PLL"]
    _1 = 1,
}
impl From<Lol> for bool {
    #[inline(always)]
    fn from(variant: Lol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOL` reader - Loss-of-lock reset"]
pub type LolR = crate::BitReader<Lol>;
impl LolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lol {
        match self.bits {
            false => Lol::_0,
            true => Lol::_1,
        }
    }
    #[doc = "Reset not caused by a loss of lock in the PLL"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lol::_0
    }
    #[doc = "Reset caused by a loss of lock in the PLL"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lol::_1
    }
}
#[doc = "Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdog {
    #[doc = "0: Reset not caused by watchdog timeout"]
    _0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    _1 = 1,
}
impl From<Wdog> for bool {
    #[inline(always)]
    fn from(variant: Wdog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG` reader - Watchdog"]
pub type WdogR = crate::BitReader<Wdog>;
impl WdogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdog {
        match self.bits {
            false => Wdog::_0,
            true => Wdog::_1,
        }
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdog::_0
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdog::_1
    }
}
#[doc = "External reset pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin {
    #[doc = "0: Reset not caused by external reset pin"]
    _0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    _1 = 1,
}
impl From<Pin> for bool {
    #[inline(always)]
    fn from(variant: Pin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN` reader - External reset pin"]
pub type PinR = crate::BitReader<Pin>;
impl PinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin {
        match self.bits {
            false => Pin::_0,
            true => Pin::_1,
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pin::_0
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pin::_1
    }
}
#[doc = "Power-on reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Por {
    #[doc = "0: Reset not caused by POR"]
    _0 = 0,
    #[doc = "1: Reset caused by POR"]
    _1 = 1,
}
impl From<Por> for bool {
    #[inline(always)]
    fn from(variant: Por) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POR` reader - Power-on reset"]
pub type PorR = crate::BitReader<Por>;
impl PorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Por {
        match self.bits {
            false => Por::_0,
            true => Por::_1,
        }
    }
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Por::_0
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Por::_1
    }
}
impl R {
    #[doc = "Bit 0 - Low leakage wakeup reset"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-voltage detect reset"]
    #[inline(always)]
    pub fn lvd(&self) -> LvdR {
        LvdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Loss-of-clock reset"]
    #[inline(always)]
    pub fn loc(&self) -> LocR {
        LocR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Loss-of-lock reset"]
    #[inline(always)]
    pub fn lol(&self) -> LolR {
        LolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Watchdog"]
    #[inline(always)]
    pub fn wdog(&self) -> WdogR {
        WdogR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External reset pin"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power-on reset"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "System Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`srs0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srs0Spec;
impl crate::RegisterSpec for Srs0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`srs0::R`](R) reader structure"]
impl crate::Readable for Srs0Spec {}
#[doc = "`reset()` method sets SRS0 to value 0x82"]
impl crate::Resettable for Srs0Spec {
    const RESET_VALUE: u8 = 0x82;
}
