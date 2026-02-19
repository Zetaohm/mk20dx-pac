#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Interrupt Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iack {
    #[doc = "0: Do not clear the interrupt."]
    _0 = 0,
    #[doc = "1: Clear the IF bit (interrupt flag)."]
    _1 = 1,
}
impl From<Iack> for bool {
    #[inline(always)]
    fn from(variant: Iack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IACK` writer - Interrupt Acknowledge"]
pub type IackW<'a, REG> = crate::BitWriter<'a, REG, Iack>;
impl<'a, REG> IackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not clear the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iack::_0)
    }
    #[doc = "Clear the IF bit (interrupt flag)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iack::_1)
    }
}
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum If {
    #[doc = "0: No interrupt is pending."]
    _0 = 0,
    #[doc = "1: An interrupt is pending."]
    _1 = 1,
}
impl From<If> for bool {
    #[inline(always)]
    fn from(variant: If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF` reader - Interrupt Flag"]
pub type IfR = crate::BitReader<If>;
impl IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> If {
        match self.bits {
            false => If::_0,
            true => If::_1,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == If::_0
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == If::_1
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ie {
    #[doc = "0: Disable interrupts to the system."]
    _0 = 0,
    #[doc = "1: Enable interrupts to the system."]
    _1 = 1,
}
impl From<Ie> for bool {
    #[inline(always)]
    fn from(variant: Ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IeR = crate::BitReader<Ie>;
impl IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ie {
        match self.bits {
            false => Ie::_0,
            true => Ie::_1,
        }
    }
    #[doc = "Disable interrupts to the system."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ie::_0
    }
    #[doc = "Enable interrupts to the system."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ie::_1
    }
}
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG, Ie>;
impl<'a, REG> IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupts to the system."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::_0)
    }
    #[doc = "Enable interrupts to the system."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::_1)
    }
}
#[doc = "Start Change Detection Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: Do not start the sequence. Writes of this value have no effect."]
    _0 = 0,
    #[doc = "1: Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    _1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Start Change Detection Sequence"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not start the sequence. Writes of this value have no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_0)
    }
    #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_1)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr {
    #[doc = "0: Do not perform a software reset."]
    _0 = 0,
    #[doc = "1: Perform a software reset."]
    _1 = 1,
}
impl From<Sr> for bool {
    #[inline(always)]
    fn from(variant: Sr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` writer - Software Reset"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG, Sr>;
impl<'a, REG> SrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not perform a software reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sr::_0)
    }
    #[doc = "Perform a software reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sr::_1)
    }
}
impl R {
    #[doc = "Bit 8 - Interrupt Flag"]
    #[inline(always)]
    pub fn if_(&self) -> IfR {
        IfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn iack(&mut self) -> IackW<ControlSpec> {
        IackW::new(self, 0)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<ControlSpec> {
        IeW::new(self, 16)
    }
    #[doc = "Bit 24 - Start Change Detection Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<ControlSpec> {
        StartW::new(self, 24)
    }
    #[doc = "Bit 25 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SrW<ControlSpec> {
        SrW::new(self, 25)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0x0001_0000"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
