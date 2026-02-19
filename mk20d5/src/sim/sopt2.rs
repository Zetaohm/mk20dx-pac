#[doc = "Register `SOPT2` reader"]
pub type R = crate::R<Sopt2Spec>;
#[doc = "Register `SOPT2` writer"]
pub type W = crate::W<Sopt2Spec>;
#[doc = "RTC clock out select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcclkoutsel {
    #[doc = "0: RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0 = 0,
    #[doc = "1: RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1 = 1,
}
impl From<Rtcclkoutsel> for bool {
    #[inline(always)]
    fn from(variant: Rtcclkoutsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCLKOUTSEL` reader - RTC clock out select"]
pub type RtcclkoutselR = crate::BitReader<Rtcclkoutsel>;
impl RtcclkoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcclkoutsel {
        match self.bits {
            false => Rtcclkoutsel::_0,
            true => Rtcclkoutsel::_1,
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcclkoutsel::_0
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcclkoutsel::_1
    }
}
#[doc = "Field `RTCCLKOUTSEL` writer - RTC clock out select"]
pub type RtcclkoutselW<'a, REG> = crate::BitWriter<'a, REG, Rtcclkoutsel>;
impl<'a, REG> RtcclkoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcclkoutsel::_0)
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcclkoutsel::_1)
    }
}
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel {
    #[doc = "2: Flash clock"]
    _010 = 2,
    #[doc = "3: LPO clock (1 kHz)"]
    _011 = 3,
    #[doc = "4: IRCLK"]
    _100 = 4,
    #[doc = "5: RTC 32.768kHz"]
    _101 = 5,
    #[doc = "6: ERCLK0"]
    _110 = 6,
}
impl From<Clkoutsel> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel {}
#[doc = "Field `CLKOUTSEL` reader - CLKOUT select"]
pub type ClkoutselR = crate::FieldReader<Clkoutsel>;
impl ClkoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel> {
        match self.bits {
            2 => Some(Clkoutsel::_010),
            3 => Some(Clkoutsel::_011),
            4 => Some(Clkoutsel::_100),
            5 => Some(Clkoutsel::_101),
            6 => Some(Clkoutsel::_110),
            _ => None,
        }
    }
    #[doc = "Flash clock"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Clkoutsel::_010
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Clkoutsel::_011
    }
    #[doc = "IRCLK"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Clkoutsel::_100
    }
    #[doc = "RTC 32.768kHz"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Clkoutsel::_101
    }
    #[doc = "ERCLK0"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Clkoutsel::_110
    }
}
#[doc = "Field `CLKOUTSEL` writer - CLKOUT select"]
pub type ClkoutselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkoutsel>;
impl<'a, REG> ClkoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::_011)
    }
    #[doc = "IRCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::_100)
    }
    #[doc = "RTC 32.768kHz"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::_101)
    }
    #[doc = "ERCLK0"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::_110)
    }
}
#[doc = "PTD7 pad drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptd7pad {
    #[doc = "0: Single-pad drive strength for PTD7."]
    _0 = 0,
    #[doc = "1: Double pad drive strength for PTD7."]
    _1 = 1,
}
impl From<Ptd7pad> for bool {
    #[inline(always)]
    fn from(variant: Ptd7pad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTD7PAD` reader - PTD7 pad drive strength"]
pub type Ptd7padR = crate::BitReader<Ptd7pad>;
impl Ptd7padR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptd7pad {
        match self.bits {
            false => Ptd7pad::_0,
            true => Ptd7pad::_1,
        }
    }
    #[doc = "Single-pad drive strength for PTD7."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ptd7pad::_0
    }
    #[doc = "Double pad drive strength for PTD7."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ptd7pad::_1
    }
}
#[doc = "Field `PTD7PAD` writer - PTD7 pad drive strength"]
pub type Ptd7padW<'a, REG> = crate::BitWriter<'a, REG, Ptd7pad>;
impl<'a, REG> Ptd7padW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-pad drive strength for PTD7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptd7pad::_0)
    }
    #[doc = "Double pad drive strength for PTD7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptd7pad::_1)
    }
}
#[doc = "Debug trace clock select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Traceclksel {
    #[doc = "0: MCGOUTCLK"]
    _0 = 0,
    #[doc = "1: Core/system clock"]
    _1 = 1,
}
impl From<Traceclksel> for bool {
    #[inline(always)]
    fn from(variant: Traceclksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRACECLKSEL` reader - Debug trace clock select"]
pub type TraceclkselR = crate::BitReader<Traceclksel>;
impl TraceclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Traceclksel {
        match self.bits {
            false => Traceclksel::_0,
            true => Traceclksel::_1,
        }
    }
    #[doc = "MCGOUTCLK"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Traceclksel::_0
    }
    #[doc = "Core/system clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Traceclksel::_1
    }
}
#[doc = "Field `TRACECLKSEL` writer - Debug trace clock select"]
pub type TraceclkselW<'a, REG> = crate::BitWriter<'a, REG, Traceclksel>;
impl<'a, REG> TraceclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCGOUTCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Traceclksel::_0)
    }
    #[doc = "Core/system clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Traceclksel::_1)
    }
}
#[doc = "PLL/FLL clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllfllsel {
    #[doc = "0: MCGFLLCLK clock"]
    _0 = 0,
    #[doc = "1: MCGPLLCLK clock"]
    _1 = 1,
}
impl From<Pllfllsel> for bool {
    #[inline(always)]
    fn from(variant: Pllfllsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLFLLSEL` reader - PLL/FLL clock select"]
pub type PllfllselR = crate::BitReader<Pllfllsel>;
impl PllfllselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllfllsel {
        match self.bits {
            false => Pllfllsel::_0,
            true => Pllfllsel::_1,
        }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pllfllsel::_0
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pllfllsel::_1
    }
}
#[doc = "Field `PLLFLLSEL` writer - PLL/FLL clock select"]
pub type PllfllselW<'a, REG> = crate::BitWriter<'a, REG, Pllfllsel>;
impl<'a, REG> PllfllselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllfllsel::_0)
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllfllsel::_1)
    }
}
#[doc = "USB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbsrc {
    #[doc = "0: External bypass clock (USB_CLKIN)."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK/MCGFLLCLK clock divided by the USB fractional divider. See the SIM_CLKDIV2\\[USBFRAC, USBDIV\\]
descriptions."]
    _1 = 1,
}
impl From<Usbsrc> for bool {
    #[inline(always)]
    fn from(variant: Usbsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSRC` reader - USB clock source select"]
pub type UsbsrcR = crate::BitReader<Usbsrc>;
impl UsbsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbsrc {
        match self.bits {
            false => Usbsrc::_0,
            true => Usbsrc::_1,
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbsrc::_0
    }
    #[doc = "MCGPLLCLK/MCGFLLCLK clock divided by the USB fractional divider. See the SIM_CLKDIV2\\[USBFRAC, USBDIV\\]
descriptions."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbsrc::_1
    }
}
#[doc = "Field `USBSRC` writer - USB clock source select"]
pub type UsbsrcW<'a, REG> = crate::BitWriter<'a, REG, Usbsrc>;
impl<'a, REG> UsbsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsrc::_0)
    }
    #[doc = "MCGPLLCLK/MCGFLLCLK clock divided by the USB fractional divider. See the SIM_CLKDIV2\\[USBFRAC, USBDIV\\]
descriptions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsrc::_1)
    }
}
impl R {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&self) -> RtcclkoutselR {
        RtcclkoutselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> ClkoutselR {
        ClkoutselR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 11 - PTD7 pad drive strength"]
    #[inline(always)]
    pub fn ptd7pad(&self) -> Ptd7padR {
        Ptd7padR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclksel(&self) -> TraceclkselR {
        TraceclkselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&self) -> PllfllselR {
        PllfllselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&self) -> UsbsrcR {
        UsbsrcR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    #[must_use]
    pub fn rtcclkoutsel(&mut self) -> RtcclkoutselW<Sopt2Spec> {
        RtcclkoutselW::new(self, 4)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel(&mut self) -> ClkoutselW<Sopt2Spec> {
        ClkoutselW::new(self, 5)
    }
    #[doc = "Bit 11 - PTD7 pad drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ptd7pad(&mut self) -> Ptd7padW<Sopt2Spec> {
        Ptd7padW::new(self, 11)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    #[must_use]
    pub fn traceclksel(&mut self) -> TraceclkselW<Sopt2Spec> {
        TraceclkselW::new(self, 12)
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pllfllsel(&mut self) -> PllfllselW<Sopt2Spec> {
        PllfllselW::new(self, 16)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn usbsrc(&mut self) -> UsbsrcW<Sopt2Spec> {
        UsbsrcW::new(self, 18)
    }
}
#[doc = "System Options Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt2Spec;
impl crate::RegisterSpec for Sopt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt2::R`](R) reader structure"]
impl crate::Readable for Sopt2Spec {}
#[doc = "`write(|w| ..)` method takes [`sopt2::W`](W) writer structure"]
impl crate::Writable for Sopt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT2 to value 0x1000"]
impl crate::Resettable for Sopt2Spec {
    const RESET_VALUE: u32 = 0x1000;
}
