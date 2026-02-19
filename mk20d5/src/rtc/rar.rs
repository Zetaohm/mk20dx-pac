#[doc = "Register `RAR` reader"]
pub type R = crate::R<RarSpec>;
#[doc = "Register `RAR` writer"]
pub type W = crate::W<RarSpec>;
#[doc = "Time Seconds Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsrr {
    #[doc = "0: Reads to the time seconds register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the time seconds register complete as normal."]
    _1 = 1,
}
impl From<Tsrr> for bool {
    #[inline(always)]
    fn from(variant: Tsrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSRR` reader - Time Seconds Register Read"]
pub type TsrrR = crate::BitReader<Tsrr>;
impl TsrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsrr {
        match self.bits {
            false => Tsrr::_0,
            true => Tsrr::_1,
        }
    }
    #[doc = "Reads to the time seconds register are ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsrr::_0
    }
    #[doc = "Reads to the time seconds register complete as normal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsrr::_1
    }
}
#[doc = "Field `TSRR` writer - Time Seconds Register Read"]
pub type TsrrW<'a, REG> = crate::BitWriter<'a, REG, Tsrr>;
impl<'a, REG> TsrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reads to the time seconds register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsrr::_0)
    }
    #[doc = "Reads to the time seconds register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsrr::_1)
    }
}
#[doc = "Time Prescaler Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tprr {
    #[doc = "0: Reads to the time prescaler register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the time prescaler register complete as normal."]
    _1 = 1,
}
impl From<Tprr> for bool {
    #[inline(always)]
    fn from(variant: Tprr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPRR` reader - Time Prescaler Register Read"]
pub type TprrR = crate::BitReader<Tprr>;
impl TprrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tprr {
        match self.bits {
            false => Tprr::_0,
            true => Tprr::_1,
        }
    }
    #[doc = "Reads to the time prescaler register are ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tprr::_0
    }
    #[doc = "Reads to the time prescaler register complete as normal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tprr::_1
    }
}
#[doc = "Field `TPRR` writer - Time Prescaler Register Read"]
pub type TprrW<'a, REG> = crate::BitWriter<'a, REG, Tprr>;
impl<'a, REG> TprrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reads to the time prescaler register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tprr::_0)
    }
    #[doc = "Reads to the time prescaler register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tprr::_1)
    }
}
#[doc = "Time Alarm Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tarr {
    #[doc = "0: Reads to the time alarm register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the time alarm register complete as normal."]
    _1 = 1,
}
impl From<Tarr> for bool {
    #[inline(always)]
    fn from(variant: Tarr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARR` reader - Time Alarm Register Read"]
pub type TarrR = crate::BitReader<Tarr>;
impl TarrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tarr {
        match self.bits {
            false => Tarr::_0,
            true => Tarr::_1,
        }
    }
    #[doc = "Reads to the time alarm register are ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tarr::_0
    }
    #[doc = "Reads to the time alarm register complete as normal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tarr::_1
    }
}
#[doc = "Field `TARR` writer - Time Alarm Register Read"]
pub type TarrW<'a, REG> = crate::BitWriter<'a, REG, Tarr>;
impl<'a, REG> TarrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reads to the time alarm register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tarr::_0)
    }
    #[doc = "Reads to the time alarm register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tarr::_1)
    }
}
#[doc = "Time Compensation Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcrr {
    #[doc = "0: Reads to the time compensation register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the time compensation register complete as normal."]
    _1 = 1,
}
impl From<Tcrr> for bool {
    #[inline(always)]
    fn from(variant: Tcrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCRR` reader - Time Compensation Register Read"]
pub type TcrrR = crate::BitReader<Tcrr>;
impl TcrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcrr {
        match self.bits {
            false => Tcrr::_0,
            true => Tcrr::_1,
        }
    }
    #[doc = "Reads to the time compensation register are ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcrr::_0
    }
    #[doc = "Reads to the time compensation register complete as normal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcrr::_1
    }
}
#[doc = "Field `TCRR` writer - Time Compensation Register Read"]
pub type TcrrW<'a, REG> = crate::BitWriter<'a, REG, Tcrr>;
impl<'a, REG> TcrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reads to the time compensation register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcrr::_0)
    }
    #[doc = "Reads to the time compensation register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcrr::_1)
    }
}
#[doc = "Control Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crr {
    #[doc = "0: Reads to the control register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the control register complete as normal."]
    _1 = 1,
}
impl From<Crr> for bool {
    #[inline(always)]
    fn from(variant: Crr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRR` reader - Control Register Read"]
pub type CrrR = crate::BitReader<Crr>;
impl CrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crr {
        match self.bits {
            false => Crr::_0,
            true => Crr::_1,
        }
    }
    #[doc = "Reads to the control register are ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crr::_0
    }
    #[doc = "Reads to the control register complete as normal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crr::_1
    }
}
#[doc = "Field `CRR` writer - Control Register Read"]
pub type CrrW<'a, REG> = crate::BitWriter<'a, REG, Crr>;
impl<'a, REG> CrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reads to the control register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Crr::_0)
    }
    #[doc = "Reads to the control register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Crr::_1)
    }
}
#[doc = "Status Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srr {
    #[doc = "0: Reads to the status register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the status register complete as normal."]
    _1 = 1,
}
impl From<Srr> for bool {
    #[inline(always)]
    fn from(variant: Srr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRR` reader - Status Register Read"]
pub type SrrR = crate::BitReader<Srr>;
impl SrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srr {
        match self.bits {
            false => Srr::_0,
            true => Srr::_1,
        }
    }
    #[doc = "Reads to the status register are ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Srr::_0
    }
    #[doc = "Reads to the status register complete as normal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Srr::_1
    }
}
#[doc = "Field `SRR` writer - Status Register Read"]
pub type SrrW<'a, REG> = crate::BitWriter<'a, REG, Srr>;
impl<'a, REG> SrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reads to the status register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Srr::_0)
    }
    #[doc = "Reads to the status register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Srr::_1)
    }
}
#[doc = "Lock Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrr {
    #[doc = "0: Reads to the lock register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the lock register complete as normal."]
    _1 = 1,
}
impl From<Lrr> for bool {
    #[inline(always)]
    fn from(variant: Lrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRR` reader - Lock Register Read"]
pub type LrrR = crate::BitReader<Lrr>;
impl LrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrr {
        match self.bits {
            false => Lrr::_0,
            true => Lrr::_1,
        }
    }
    #[doc = "Reads to the lock register are ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lrr::_0
    }
    #[doc = "Reads to the lock register complete as normal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lrr::_1
    }
}
#[doc = "Field `LRR` writer - Lock Register Read"]
pub type LrrW<'a, REG> = crate::BitWriter<'a, REG, Lrr>;
impl<'a, REG> LrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reads to the lock register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lrr::_0)
    }
    #[doc = "Reads to the lock register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrr::_1)
    }
}
#[doc = "Interrupt Enable Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ierr {
    #[doc = "0: Reads to the interrupt enable register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the interrupt enable register complete as normal."]
    _1 = 1,
}
impl From<Ierr> for bool {
    #[inline(always)]
    fn from(variant: Ierr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERR` reader - Interrupt Enable Register Read"]
pub type IerrR = crate::BitReader<Ierr>;
impl IerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ierr {
        match self.bits {
            false => Ierr::_0,
            true => Ierr::_1,
        }
    }
    #[doc = "Reads to the interrupt enable register are ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ierr::_0
    }
    #[doc = "Reads to the interrupt enable register complete as normal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ierr::_1
    }
}
#[doc = "Field `IERR` writer - Interrupt Enable Register Read"]
pub type IerrW<'a, REG> = crate::BitWriter<'a, REG, Ierr>;
impl<'a, REG> IerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reads to the interrupt enable register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ierr::_0)
    }
    #[doc = "Reads to the interrupt enable register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ierr::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Time Seconds Register Read"]
    #[inline(always)]
    pub fn tsrr(&self) -> TsrrR {
        TsrrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Prescaler Register Read"]
    #[inline(always)]
    pub fn tprr(&self) -> TprrR {
        TprrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Register Read"]
    #[inline(always)]
    pub fn tarr(&self) -> TarrR {
        TarrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Compensation Register Read"]
    #[inline(always)]
    pub fn tcrr(&self) -> TcrrR {
        TcrrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control Register Read"]
    #[inline(always)]
    pub fn crr(&self) -> CrrR {
        CrrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Register Read"]
    #[inline(always)]
    pub fn srr(&self) -> SrrR {
        SrrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Register Read"]
    #[inline(always)]
    pub fn lrr(&self) -> LrrR {
        LrrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable Register Read"]
    #[inline(always)]
    pub fn ierr(&self) -> IerrR {
        IerrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Seconds Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn tsrr(&mut self) -> TsrrW<RarSpec> {
        TsrrW::new(self, 0)
    }
    #[doc = "Bit 1 - Time Prescaler Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn tprr(&mut self) -> TprrW<RarSpec> {
        TprrW::new(self, 1)
    }
    #[doc = "Bit 2 - Time Alarm Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn tarr(&mut self) -> TarrW<RarSpec> {
        TarrW::new(self, 2)
    }
    #[doc = "Bit 3 - Time Compensation Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn tcrr(&mut self) -> TcrrW<RarSpec> {
        TcrrW::new(self, 3)
    }
    #[doc = "Bit 4 - Control Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn crr(&mut self) -> CrrW<RarSpec> {
        CrrW::new(self, 4)
    }
    #[doc = "Bit 5 - Status Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SrrW<RarSpec> {
        SrrW::new(self, 5)
    }
    #[doc = "Bit 6 - Lock Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn lrr(&mut self) -> LrrW<RarSpec> {
        LrrW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Enable Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn ierr(&mut self) -> IerrW<RarSpec> {
        IerrW::new(self, 7)
    }
}
#[doc = "RTC Read Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RarSpec;
impl crate::RegisterSpec for RarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rar::R`](R) reader structure"]
impl crate::Readable for RarSpec {}
#[doc = "`write(|w| ..)` method takes [`rar::W`](W) writer structure"]
impl crate::Writable for RarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAR to value 0xff"]
impl crate::Resettable for RarSpec {
    const RESET_VALUE: u32 = 0xff;
}
