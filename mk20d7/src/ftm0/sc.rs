#[doc = "Register `SC` reader"]
pub type R = crate::R<ScSpec>;
#[doc = "Register `SC` writer"]
pub type W = crate::W<ScSpec>;
#[doc = "Prescale Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: Divide by 1"]
    _000 = 0,
    #[doc = "1: Divide by 2"]
    _001 = 1,
    #[doc = "2: Divide by 4"]
    _010 = 2,
    #[doc = "3: Divide by 8"]
    _011 = 3,
    #[doc = "4: Divide by 16"]
    _100 = 4,
    #[doc = "5: Divide by 32"]
    _101 = 5,
    #[doc = "6: Divide by 64"]
    _110 = 6,
    #[doc = "7: Divide by 128"]
    _111 = 7,
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ps {
    type Ux = u8;
}
impl crate::IsEnum for Ps {}
#[doc = "Field `PS` reader - Prescale Factor Selection"]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            0 => Ps::_000,
            1 => Ps::_001,
            2 => Ps::_010,
            3 => Ps::_011,
            4 => Ps::_100,
            5 => Ps::_101,
            6 => Ps::_110,
            7 => Ps::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ps::_000
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ps::_001
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ps::_010
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ps::_011
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ps::_100
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ps::_101
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ps::_110
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Ps::_111
    }
}
#[doc = "Field `PS` writer - Prescale Factor Selection"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ps, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_001)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_010)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_011)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_100)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_101)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_110)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_111)
    }
}
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clks {
    #[doc = "0: No clock selected (This in effect disables the FTM counter.)"]
    _00 = 0,
    #[doc = "1: System clock"]
    _01 = 1,
    #[doc = "2: Fixed frequency clock"]
    _10 = 2,
    #[doc = "3: External clock"]
    _11 = 3,
}
impl From<Clks> for u8 {
    #[inline(always)]
    fn from(variant: Clks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clks {
    type Ux = u8;
}
impl crate::IsEnum for Clks {}
#[doc = "Field `CLKS` reader - Clock Source Selection"]
pub type ClksR = crate::FieldReader<Clks>;
impl ClksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clks {
        match self.bits {
            0 => Clks::_00,
            1 => Clks::_01,
            2 => Clks::_10,
            3 => Clks::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock selected (This in effect disables the FTM counter.)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Clks::_00
    }
    #[doc = "System clock"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Clks::_01
    }
    #[doc = "Fixed frequency clock"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Clks::_10
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Clks::_11
    }
}
#[doc = "Field `CLKS` writer - Clock Source Selection"]
pub type ClksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clks, crate::Safe>;
impl<'a, REG> ClksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected (This in effect disables the FTM counter.)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_00)
    }
    #[doc = "System clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_01)
    }
    #[doc = "Fixed frequency clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_10)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_11)
    }
}
#[doc = "Center-aligned PWM Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpwms {
    #[doc = "0: FTM counter operates in up counting mode."]
    _0 = 0,
    #[doc = "1: FTM counter operates in up-down counting mode."]
    _1 = 1,
}
impl From<Cpwms> for bool {
    #[inline(always)]
    fn from(variant: Cpwms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPWMS` reader - Center-aligned PWM Select"]
pub type CpwmsR = crate::BitReader<Cpwms>;
impl CpwmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpwms {
        match self.bits {
            false => Cpwms::_0,
            true => Cpwms::_1,
        }
    }
    #[doc = "FTM counter operates in up counting mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpwms::_0
    }
    #[doc = "FTM counter operates in up-down counting mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpwms::_1
    }
}
#[doc = "Field `CPWMS` writer - Center-aligned PWM Select"]
pub type CpwmsW<'a, REG> = crate::BitWriter<'a, REG, Cpwms>;
impl<'a, REG> CpwmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FTM counter operates in up counting mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpwms::_0)
    }
    #[doc = "FTM counter operates in up-down counting mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpwms::_1)
    }
}
#[doc = "Timer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toie {
    #[doc = "0: Disable TOF interrupts. Use software polling."]
    _0 = 0,
    #[doc = "1: Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    _1 = 1,
}
impl From<Toie> for bool {
    #[inline(always)]
    fn from(variant: Toie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIE` reader - Timer Overflow Interrupt Enable"]
pub type ToieR = crate::BitReader<Toie>;
impl ToieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toie {
        match self.bits {
            false => Toie::_0,
            true => Toie::_1,
        }
    }
    #[doc = "Disable TOF interrupts. Use software polling."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toie::_0
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toie::_1
    }
}
#[doc = "Field `TOIE` writer - Timer Overflow Interrupt Enable"]
pub type ToieW<'a, REG> = crate::BitWriter<'a, REG, Toie>;
impl<'a, REG> ToieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TOF interrupts. Use software polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toie::_0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toie::_1)
    }
}
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tof {
    #[doc = "0: FTM counter has not overflowed."]
    _0 = 0,
    #[doc = "1: FTM counter has overflowed."]
    _1 = 1,
}
impl From<Tof> for bool {
    #[inline(always)]
    fn from(variant: Tof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Timer Overflow Flag"]
pub type TofR = crate::BitReader<Tof>;
impl TofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tof {
        match self.bits {
            false => Tof::_0,
            true => Tof::_1,
        }
    }
    #[doc = "FTM counter has not overflowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tof::_0
    }
    #[doc = "FTM counter has overflowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tof::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Center-aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&self) -> CpwmsR {
        CpwmsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> ToieR {
        ToieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TofR {
        TofR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<ScSpec> {
        PsW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> ClksW<ScSpec> {
        ClksW::new(self, 3)
    }
    #[doc = "Bit 5 - Center-aligned PWM Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpwms(&mut self) -> CpwmsW<ScSpec> {
        CpwmsW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> ToieW<ScSpec> {
        ToieW::new(self, 6)
    }
}
#[doc = "Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScSpec;
impl crate::RegisterSpec for ScSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for ScSpec {}
#[doc = "`write(|w| ..)` method takes [`sc::W`](W) writer structure"]
impl crate::Writable for ScSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for ScSpec {
    const RESET_VALUE: u32 = 0;
}
