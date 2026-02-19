#[doc = "Register `RPFC` reader"]
pub type R = crate::R<RpfcSpec>;
#[doc = "Register `RPFC` writer"]
pub type W = crate::W<RpfcSpec>;
#[doc = "Reset pin filter select in run and wait modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rstfltsrw {
    #[doc = "0: All filtering disabled"]
    _00 = 0,
    #[doc = "1: Bus clock filter enabled for normal operation"]
    _01 = 1,
    #[doc = "2: LPO clock filter enabled for normal operation"]
    _10 = 2,
    #[doc = "3: Reserved (all filtering disabled)"]
    _11 = 3,
}
impl From<Rstfltsrw> for u8 {
    #[inline(always)]
    fn from(variant: Rstfltsrw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rstfltsrw {
    type Ux = u8;
}
impl crate::IsEnum for Rstfltsrw {}
#[doc = "Field `RSTFLTSRW` reader - Reset pin filter select in run and wait modes"]
pub type RstfltsrwR = crate::FieldReader<Rstfltsrw>;
impl RstfltsrwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstfltsrw {
        match self.bits {
            0 => Rstfltsrw::_00,
            1 => Rstfltsrw::_01,
            2 => Rstfltsrw::_10,
            3 => Rstfltsrw::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rstfltsrw::_00
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rstfltsrw::_01
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rstfltsrw::_10
    }
    #[doc = "Reserved (all filtering disabled)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rstfltsrw::_11
    }
}
#[doc = "Field `RSTFLTSRW` writer - Reset pin filter select in run and wait modes"]
pub type RstfltsrwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rstfltsrw, crate::Safe>;
impl<'a, REG> RstfltsrwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsrw::_00)
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsrw::_01)
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsrw::_10)
    }
    #[doc = "Reserved (all filtering disabled)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsrw::_11)
    }
}
#[doc = "Reset pin filter select in stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstfltss {
    #[doc = "0: All filtering disabled"]
    _0 = 0,
    #[doc = "1: LPO clock filter enabled"]
    _1 = 1,
}
impl From<Rstfltss> for bool {
    #[inline(always)]
    fn from(variant: Rstfltss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFLTSS` reader - Reset pin filter select in stop mode"]
pub type RstfltssR = crate::BitReader<Rstfltss>;
impl RstfltssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstfltss {
        match self.bits {
            false => Rstfltss::_0,
            true => Rstfltss::_1,
        }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rstfltss::_0
    }
    #[doc = "LPO clock filter enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rstfltss::_1
    }
}
#[doc = "Field `RSTFLTSS` writer - Reset pin filter select in stop mode"]
pub type RstfltssW<'a, REG> = crate::BitWriter<'a, REG, Rstfltss>;
impl<'a, REG> RstfltssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltss::_0)
    }
    #[doc = "LPO clock filter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltss::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Reset pin filter select in run and wait modes"]
    #[inline(always)]
    pub fn rstfltsrw(&self) -> RstfltsrwR {
        RstfltsrwR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Reset pin filter select in stop mode"]
    #[inline(always)]
    pub fn rstfltss(&self) -> RstfltssR {
        RstfltssR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset pin filter select in run and wait modes"]
    #[inline(always)]
    #[must_use]
    pub fn rstfltsrw(&mut self) -> RstfltsrwW<RpfcSpec> {
        RstfltsrwW::new(self, 0)
    }
    #[doc = "Bit 2 - Reset pin filter select in stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn rstfltss(&mut self) -> RstfltssW<RpfcSpec> {
        RstfltssW::new(self, 2)
    }
}
#[doc = "Reset Pin Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RpfcSpec;
impl crate::RegisterSpec for RpfcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rpfc::R`](R) reader structure"]
impl crate::Readable for RpfcSpec {}
#[doc = "`write(|w| ..)` method takes [`rpfc::W`](W) writer structure"]
impl crate::Writable for RpfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RPFC to value 0"]
impl crate::Resettable for RpfcSpec {
    const RESET_VALUE: u8 = 0;
}
