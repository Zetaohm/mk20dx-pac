#[doc = "Register `USBCTRL` reader"]
pub type R = crate::R<UsbctrlSpec>;
#[doc = "Register `USBCTRL` writer"]
pub type W = crate::W<UsbctrlSpec>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pde {
    #[doc = "0: Weak pulldowns are disabled on D+ and D-"]
    _0 = 0,
    #[doc = "1: Weak pulldowns are enabled on D+ and D-."]
    _1 = 1,
}
impl From<Pde> for bool {
    #[inline(always)]
    fn from(variant: Pde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDE` reader - no description available"]
pub type PdeR = crate::BitReader<Pde>;
impl PdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pde {
        match self.bits {
            false => Pde::_0,
            true => Pde::_1,
        }
    }
    #[doc = "Weak pulldowns are disabled on D+ and D-"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pde::_0
    }
    #[doc = "Weak pulldowns are enabled on D+ and D-."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pde::_1
    }
}
#[doc = "Field `PDE` writer - no description available"]
pub type PdeW<'a, REG> = crate::BitWriter<'a, REG, Pde>;
impl<'a, REG> PdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Weak pulldowns are disabled on D+ and D-"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pde::_0)
    }
    #[doc = "Weak pulldowns are enabled on D+ and D-."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pde::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Susp {
    #[doc = "0: USB transceiver is not in suspend state."]
    _0 = 0,
    #[doc = "1: USB transceiver is in suspend state."]
    _1 = 1,
}
impl From<Susp> for bool {
    #[inline(always)]
    fn from(variant: Susp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - no description available"]
pub type SuspR = crate::BitReader<Susp>;
impl SuspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Susp {
        match self.bits {
            false => Susp::_0,
            true => Susp::_1,
        }
    }
    #[doc = "USB transceiver is not in suspend state."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Susp::_0
    }
    #[doc = "USB transceiver is in suspend state."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Susp::_1
    }
}
#[doc = "Field `SUSP` writer - no description available"]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG, Susp>;
impl<'a, REG> SuspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB transceiver is not in suspend state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Susp::_0)
    }
    #[doc = "USB transceiver is in suspend state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Susp::_1)
    }
}
impl R {
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn pde(&self) -> PdeR {
        PdeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pde(&mut self) -> PdeW<UsbctrlSpec> {
        PdeW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SuspW<UsbctrlSpec> {
        SuspW::new(self, 7)
    }
}
#[doc = "USB Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbctrlSpec;
impl crate::RegisterSpec for UsbctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbctrl::R`](R) reader structure"]
impl crate::Readable for UsbctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbctrl::W`](W) writer structure"]
impl crate::Writable for UsbctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USBCTRL to value 0xc0"]
impl crate::Resettable for UsbctrlSpec {
    const RESET_VALUE: u8 = 0xc0;
}
