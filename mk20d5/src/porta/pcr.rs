#[doc = "Register `PCR%s` reader"]
pub type R = crate::R<PcrSpec>;
#[doc = "Register `PCR%s` writer"]
pub type W = crate::W<PcrSpec>;
#[doc = "Pull Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ps {
    #[doc = "0: Internal pull-down resistor is enabled on the corresponding pin, if the corresponding Port Pull Enable Register bit is set."]
    _0 = 0,
    #[doc = "1: Internal pull-up resistor is enabled on the corresponding pin, if the corresponding Port Pull Enable Register bit is set."]
    _1 = 1,
}
impl From<Ps> for bool {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Pull Select"]
pub type PsR = crate::BitReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            false => Ps::_0,
            true => Ps::_1,
        }
    }
    #[doc = "Internal pull-down resistor is enabled on the corresponding pin, if the corresponding Port Pull Enable Register bit is set."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ps::_0
    }
    #[doc = "Internal pull-up resistor is enabled on the corresponding pin, if the corresponding Port Pull Enable Register bit is set."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ps::_1
    }
}
#[doc = "Field `PS` writer - Pull Select"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG, Ps>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal pull-down resistor is enabled on the corresponding pin, if the corresponding Port Pull Enable Register bit is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_0)
    }
    #[doc = "Internal pull-up resistor is enabled on the corresponding pin, if the corresponding Port Pull Enable Register bit is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_1)
    }
}
#[doc = "Pull Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Internal pull-up or pull-down resistor is not enabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Internal pull-up or pull-down resistor is enabled on the corresponding pin, provided pin is configured as a digital input."]
    _1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Pull Enable"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::_0,
            true => Pe::_1,
        }
    }
    #[doc = "Internal pull-up or pull-down resistor is not enabled on the corresponding pin."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pe::_0
    }
    #[doc = "Internal pull-up or pull-down resistor is enabled on the corresponding pin, provided pin is configured as a digital input."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pe::_1
    }
}
#[doc = "Field `PE` writer - Pull Enable"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal pull-up or pull-down resistor is not enabled on the corresponding pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_0)
    }
    #[doc = "Internal pull-up or pull-down resistor is enabled on the corresponding pin, provided pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_1)
    }
}
#[doc = "Slew Rate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sre {
    #[doc = "0: Fast slew rate is configured on the corresponding pin, if pin is configured as a digital output."]
    _0 = 0,
    #[doc = "1: Slow slew rate is configured on the corresponding pin, if pin is configured as a digital output."]
    _1 = 1,
}
impl From<Sre> for bool {
    #[inline(always)]
    fn from(variant: Sre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRE` reader - Slew Rate Enable"]
pub type SreR = crate::BitReader<Sre>;
impl SreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sre {
        match self.bits {
            false => Sre::_0,
            true => Sre::_1,
        }
    }
    #[doc = "Fast slew rate is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sre::_0
    }
    #[doc = "Slow slew rate is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sre::_1
    }
}
#[doc = "Field `SRE` writer - Slew Rate Enable"]
pub type SreW<'a, REG> = crate::BitWriter<'a, REG, Sre>;
impl<'a, REG> SreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast slew rate is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sre::_0)
    }
    #[doc = "Slow slew rate is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sre::_1)
    }
}
#[doc = "Passive Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfe {
    #[doc = "0: Passive Input Filter is disabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Passive Input Filter is enabled on the corresponding pin, provided pin is configured as a digital input. A low pass filter (10 MHz to 30 MHz bandwidth) is enabled on the digital input path. Disable the Passive Input Filter when supporting high speed interfaces (> 2 MHz) on the pin."]
    _1 = 1,
}
impl From<Pfe> for bool {
    #[inline(always)]
    fn from(variant: Pfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFE` reader - Passive Filter Enable"]
pub type PfeR = crate::BitReader<Pfe>;
impl PfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfe {
        match self.bits {
            false => Pfe::_0,
            true => Pfe::_1,
        }
    }
    #[doc = "Passive Input Filter is disabled on the corresponding pin."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pfe::_0
    }
    #[doc = "Passive Input Filter is enabled on the corresponding pin, provided pin is configured as a digital input. A low pass filter (10 MHz to 30 MHz bandwidth) is enabled on the digital input path. Disable the Passive Input Filter when supporting high speed interfaces (> 2 MHz) on the pin."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pfe::_1
    }
}
#[doc = "Field `PFE` writer - Passive Filter Enable"]
pub type PfeW<'a, REG> = crate::BitWriter<'a, REG, Pfe>;
impl<'a, REG> PfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Input Filter is disabled on the corresponding pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pfe::_0)
    }
    #[doc = "Passive Input Filter is enabled on the corresponding pin, provided pin is configured as a digital input. A low pass filter (10 MHz to 30 MHz bandwidth) is enabled on the digital input path. Disable the Passive Input Filter when supporting high speed interfaces (> 2 MHz) on the pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pfe::_1)
    }
}
#[doc = "Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ode {
    #[doc = "0: Open Drain output is disabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Open Drain output is enabled on the corresponding pin, provided pin is configured as a digital output."]
    _1 = 1,
}
impl From<Ode> for bool {
    #[inline(always)]
    fn from(variant: Ode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODE` reader - Open Drain Enable"]
pub type OdeR = crate::BitReader<Ode>;
impl OdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ode {
        match self.bits {
            false => Ode::_0,
            true => Ode::_1,
        }
    }
    #[doc = "Open Drain output is disabled on the corresponding pin."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ode::_0
    }
    #[doc = "Open Drain output is enabled on the corresponding pin, provided pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ode::_1
    }
}
#[doc = "Field `ODE` writer - Open Drain Enable"]
pub type OdeW<'a, REG> = crate::BitWriter<'a, REG, Ode>;
impl<'a, REG> OdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open Drain output is disabled on the corresponding pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ode::_0)
    }
    #[doc = "Open Drain output is enabled on the corresponding pin, provided pin is configured as a digital output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ode::_1)
    }
}
#[doc = "Drive Strength Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dse {
    #[doc = "0: Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    _0 = 0,
    #[doc = "1: High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    _1 = 1,
}
impl From<Dse> for bool {
    #[inline(always)]
    fn from(variant: Dse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSE` reader - Drive Strength Enable"]
pub type DseR = crate::BitReader<Dse>;
impl DseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dse {
        match self.bits {
            false => Dse::_0,
            true => Dse::_1,
        }
    }
    #[doc = "Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dse::_0
    }
    #[doc = "High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dse::_1
    }
}
#[doc = "Field `DSE` writer - Drive Strength Enable"]
pub type DseW<'a, REG> = crate::BitWriter<'a, REG, Dse>;
impl<'a, REG> DseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dse::_0)
    }
    #[doc = "High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dse::_1)
    }
}
#[doc = "Pin Mux Control\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux {
    #[doc = "0: Pin Disabled (Analog)."]
    _000 = 0,
    #[doc = "1: Alternative 1 (GPIO)."]
    _001 = 1,
    #[doc = "2: Alternative 2 (chip specific)."]
    _010 = 2,
    #[doc = "3: Alternative 3 (chip specific)."]
    _011 = 3,
    #[doc = "4: Alternative 4 (chip specific)."]
    _100 = 4,
    #[doc = "5: Alternative 5 (chip specific)."]
    _101 = 5,
    #[doc = "6: Alternative 6 (chip specific)."]
    _110 = 6,
    #[doc = "7: Alternative 7 (chip specific / JTAG / NMI)."]
    _111 = 7,
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(variant: Mux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux {
    type Ux = u8;
}
impl crate::IsEnum for Mux {}
#[doc = "Field `MUX` reader - Pin Mux Control"]
pub type MuxR = crate::FieldReader<Mux>;
impl MuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mux {
        match self.bits {
            0 => Mux::_000,
            1 => Mux::_001,
            2 => Mux::_010,
            3 => Mux::_011,
            4 => Mux::_100,
            5 => Mux::_101,
            6 => Mux::_110,
            7 => Mux::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin Disabled (Analog)."]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Mux::_000
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Mux::_001
    }
    #[doc = "Alternative 2 (chip specific)."]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Mux::_010
    }
    #[doc = "Alternative 3 (chip specific)."]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Mux::_011
    }
    #[doc = "Alternative 4 (chip specific)."]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Mux::_100
    }
    #[doc = "Alternative 5 (chip specific)."]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Mux::_101
    }
    #[doc = "Alternative 6 (chip specific)."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Mux::_110
    }
    #[doc = "Alternative 7 (chip specific / JTAG / NMI)."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Mux::_111
    }
}
#[doc = "Field `MUX` writer - Pin Mux Control"]
pub type MuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mux, crate::Safe>;
impl<'a, REG> MuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin Disabled (Analog)."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::_000)
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::_001)
    }
    #[doc = "Alternative 2 (chip specific)."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::_010)
    }
    #[doc = "Alternative 3 (chip specific)."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::_011)
    }
    #[doc = "Alternative 4 (chip specific)."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::_100)
    }
    #[doc = "Alternative 5 (chip specific)."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::_101)
    }
    #[doc = "Alternative 6 (chip specific)."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::_110)
    }
    #[doc = "Alternative 7 (chip specific / JTAG / NMI)."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::_111)
    }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lk {
    #[doc = "0: Pin Control Register bits \\[15:0\\]
are not locked."]
    _0 = 0,
    #[doc = "1: Pin Control Register bits \\[15:0\\]
are locked and cannot be updated until the next System Reset."]
    _1 = 1,
}
impl From<Lk> for bool {
    #[inline(always)]
    fn from(variant: Lk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` reader - Lock Register"]
pub type LkR = crate::BitReader<Lk>;
impl LkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lk {
        match self.bits {
            false => Lk::_0,
            true => Lk::_1,
        }
    }
    #[doc = "Pin Control Register bits \\[15:0\\]
are not locked."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lk::_0
    }
    #[doc = "Pin Control Register bits \\[15:0\\]
are locked and cannot be updated until the next System Reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lk::_1
    }
}
#[doc = "Field `LK` writer - Lock Register"]
pub type LkW<'a, REG> = crate::BitWriter<'a, REG, Lk>;
impl<'a, REG> LkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Control Register bits \\[15:0\\]
are not locked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lk::_0)
    }
    #[doc = "Pin Control Register bits \\[15:0\\]
are locked and cannot be updated until the next System Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lk::_1)
    }
}
#[doc = "Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irqc {
    #[doc = "0: Interrupt/DMA Request disabled."]
    _0000 = 0,
    #[doc = "1: DMA Request on rising edge."]
    _0001 = 1,
    #[doc = "2: DMA Request on falling edge."]
    _0010 = 2,
    #[doc = "3: DMA Request on either edge."]
    _0011 = 3,
    #[doc = "8: Interrupt when logic zero."]
    _1000 = 8,
    #[doc = "9: Interrupt on rising edge."]
    _1001 = 9,
    #[doc = "10: Interrupt on falling edge."]
    _1010 = 10,
    #[doc = "11: Interrupt on either edge."]
    _1011 = 11,
    #[doc = "12: Interrupt when logic one."]
    _1100 = 12,
}
impl From<Irqc> for u8 {
    #[inline(always)]
    fn from(variant: Irqc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irqc {
    type Ux = u8;
}
impl crate::IsEnum for Irqc {}
#[doc = "Field `IRQC` reader - Interrupt Configuration"]
pub type IrqcR = crate::FieldReader<Irqc>;
impl IrqcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Irqc> {
        match self.bits {
            0 => Some(Irqc::_0000),
            1 => Some(Irqc::_0001),
            2 => Some(Irqc::_0010),
            3 => Some(Irqc::_0011),
            8 => Some(Irqc::_1000),
            9 => Some(Irqc::_1001),
            10 => Some(Irqc::_1010),
            11 => Some(Irqc::_1011),
            12 => Some(Irqc::_1100),
            _ => None,
        }
    }
    #[doc = "Interrupt/DMA Request disabled."]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Irqc::_0000
    }
    #[doc = "DMA Request on rising edge."]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Irqc::_0001
    }
    #[doc = "DMA Request on falling edge."]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Irqc::_0010
    }
    #[doc = "DMA Request on either edge."]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Irqc::_0011
    }
    #[doc = "Interrupt when logic zero."]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Irqc::_1000
    }
    #[doc = "Interrupt on rising edge."]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Irqc::_1001
    }
    #[doc = "Interrupt on falling edge."]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Irqc::_1010
    }
    #[doc = "Interrupt on either edge."]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Irqc::_1011
    }
    #[doc = "Interrupt when logic one."]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Irqc::_1100
    }
}
#[doc = "Field `IRQC` writer - Interrupt Configuration"]
pub type IrqcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Irqc>;
impl<'a, REG> IrqcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt/DMA Request disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_0000)
    }
    #[doc = "DMA Request on rising edge."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_0001)
    }
    #[doc = "DMA Request on falling edge."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_0010)
    }
    #[doc = "DMA Request on either edge."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_0011)
    }
    #[doc = "Interrupt when logic zero."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1000)
    }
    #[doc = "Interrupt on rising edge."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1001)
    }
    #[doc = "Interrupt on falling edge."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1010)
    }
    #[doc = "Interrupt on either edge."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1011)
    }
    #[doc = "Interrupt when logic one."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1100)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf {
    #[doc = "0: Configured interrupt has not been detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to that flag. If configured for a level sensitive interrupt that remains asserted then flag will set again immediately."]
    _1 = 1,
}
impl From<Isf> for bool {
    #[inline(always)]
    fn from(variant: Isf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF` reader - Interrupt Status Flag"]
pub type IsfR = crate::BitReader<Isf>;
impl IsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf {
        match self.bits {
            false => Isf::_0,
            true => Isf::_1,
        }
    }
    #[doc = "Configured interrupt has not been detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isf::_0
    }
    #[doc = "Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to that flag. If configured for a level sensitive interrupt that remains asserted then flag will set again immediately."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isf::_1
    }
}
#[doc = "Field `ISF` writer - Interrupt Status Flag"]
pub type IsfW<'a, REG> = crate::BitWriter<'a, REG, Isf>;
impl<'a, REG> IsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::_0)
    }
    #[doc = "Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to that flag. If configured for a level sensitive interrupt that remains asserted then flag will set again immediately."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slew Rate Enable"]
    #[inline(always)]
    pub fn sre(&self) -> SreR {
        SreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    pub fn pfe(&self) -> PfeR {
        PfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Open Drain Enable"]
    #[inline(always)]
    pub fn ode(&self) -> OdeR {
        OdeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    pub fn dse(&self) -> DseR {
        DseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LkR {
        LkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&self) -> IrqcR {
        IrqcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> IsfR {
        IsfR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<PcrSpec> {
        PsW::new(self, 0)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<PcrSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - Slew Rate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SreW<PcrSpec> {
        SreW::new(self, 2)
    }
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfe(&mut self) -> PfeW<PcrSpec> {
        PfeW::new(self, 4)
    }
    #[doc = "Bit 5 - Open Drain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ode(&mut self) -> OdeW<PcrSpec> {
        OdeW::new(self, 5)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dse(&mut self) -> DseW<PcrSpec> {
        DseW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MuxW<PcrSpec> {
        MuxW::new(self, 8)
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline(always)]
    #[must_use]
    pub fn lk(&mut self) -> LkW<PcrSpec> {
        LkW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn irqc(&mut self) -> IrqcW<PcrSpec> {
        IrqcW::new(self, 16)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf(&mut self) -> IsfW<PcrSpec> {
        IsfW::new(self, 24)
    }
}
#[doc = "Pin Control Register n\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrSpec;
impl crate::RegisterSpec for PcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR%s to value 0x0742"]
impl crate::Resettable for PcrSpec {
    const RESET_VALUE: u32 = 0x0742;
}
