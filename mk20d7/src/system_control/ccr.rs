#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonbasethrdena {
    #[doc = "0: processor can enter Thread mode only when no exception is active"]
    _0 = 0,
    #[doc = "1: processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    _1 = 1,
}
impl From<Nonbasethrdena> for bool {
    #[inline(always)]
    fn from(variant: Nonbasethrdena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NONBASETHRDENA` reader - no description available"]
pub type NonbasethrdenaR = crate::BitReader<Nonbasethrdena>;
impl NonbasethrdenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nonbasethrdena {
        match self.bits {
            false => Nonbasethrdena::_0,
            true => Nonbasethrdena::_1,
        }
    }
    #[doc = "processor can enter Thread mode only when no exception is active"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonbasethrdena::_0
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonbasethrdena::_1
    }
}
#[doc = "Field `NONBASETHRDENA` writer - no description available"]
pub type NonbasethrdenaW<'a, REG> = crate::BitWriter<'a, REG, Nonbasethrdena>;
impl<'a, REG> NonbasethrdenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processor can enter Thread mode only when no exception is active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonbasethrdena::_0)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonbasethrdena::_1)
    }
}
#[doc = "Enables unprivileged software access to the STIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usersetmpend {
    #[doc = "0: disable"]
    _0 = 0,
    #[doc = "1: enable"]
    _1 = 1,
}
impl From<Usersetmpend> for bool {
    #[inline(always)]
    fn from(variant: Usersetmpend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USERSETMPEND` reader - Enables unprivileged software access to the STIR"]
pub type UsersetmpendR = crate::BitReader<Usersetmpend>;
impl UsersetmpendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usersetmpend {
        match self.bits {
            false => Usersetmpend::_0,
            true => Usersetmpend::_1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usersetmpend::_0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usersetmpend::_1
    }
}
#[doc = "Field `USERSETMPEND` writer - Enables unprivileged software access to the STIR"]
pub type UsersetmpendW<'a, REG> = crate::BitWriter<'a, REG, Usersetmpend>;
impl<'a, REG> UsersetmpendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usersetmpend::_0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usersetmpend::_1)
    }
}
#[doc = "Enables unaligned access traps\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnalignTrp {
    #[doc = "0: do not trap unaligned halfword and word accesses"]
    _0 = 0,
    #[doc = "1: trap unaligned halfword and word accesses"]
    _1 = 1,
}
impl From<UnalignTrp> for bool {
    #[inline(always)]
    fn from(variant: UnalignTrp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNALIGN_TRP` reader - Enables unaligned access traps"]
pub type UnalignTrpR = crate::BitReader<UnalignTrp>;
impl UnalignTrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UnalignTrp {
        match self.bits {
            false => UnalignTrp::_0,
            true => UnalignTrp::_1,
        }
    }
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UnalignTrp::_0
    }
    #[doc = "trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UnalignTrp::_1
    }
}
#[doc = "Field `UNALIGN_TRP` writer - Enables unaligned access traps"]
pub type UnalignTrpW<'a, REG> = crate::BitWriter<'a, REG, UnalignTrp>;
impl<'a, REG> UnalignTrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UnalignTrp::_0)
    }
    #[doc = "trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UnalignTrp::_1)
    }
}
#[doc = "Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Div0Trp {
    #[doc = "0: do not trap divide by 0"]
    _0 = 0,
    #[doc = "1: trap divide by 0"]
    _1 = 1,
}
impl From<Div0Trp> for bool {
    #[inline(always)]
    fn from(variant: Div0Trp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV_0_TRP` reader - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
pub type Div0TrpR = crate::BitReader<Div0Trp>;
impl Div0TrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Div0Trp {
        match self.bits {
            false => Div0Trp::_0,
            true => Div0Trp::_1,
        }
    }
    #[doc = "do not trap divide by 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Div0Trp::_0
    }
    #[doc = "trap divide by 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Div0Trp::_1
    }
}
#[doc = "Field `DIV_0_TRP` writer - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
pub type Div0TrpW<'a, REG> = crate::BitWriter<'a, REG, Div0Trp>;
impl<'a, REG> Div0TrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "do not trap divide by 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Div0Trp::_0)
    }
    #[doc = "trap divide by 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Div0Trp::_1)
    }
}
#[doc = "Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfhfnmign {
    #[doc = "0: data bus faults caused by load and store instructions cause a lock-up"]
    _0 = 0,
    #[doc = "1: handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    _1 = 1,
}
impl From<Bfhfnmign> for bool {
    #[inline(always)]
    fn from(variant: Bfhfnmign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFHFNMIGN` reader - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
pub type BfhfnmignR = crate::BitReader<Bfhfnmign>;
impl BfhfnmignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfhfnmign {
        match self.bits {
            false => Bfhfnmign::_0,
            true => Bfhfnmign::_1,
        }
    }
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfhfnmign::_0
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfhfnmign::_1
    }
}
#[doc = "Field `BFHFNMIGN` writer - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
pub type BfhfnmignW<'a, REG> = crate::BitWriter<'a, REG, Bfhfnmign>;
impl<'a, REG> BfhfnmignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfhfnmign::_0)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfhfnmign::_1)
    }
}
#[doc = "Indicates stack alignment on exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stkalign {
    #[doc = "0: 4-byte aligned"]
    _0 = 0,
    #[doc = "1: 8-byte aligned"]
    _1 = 1,
}
impl From<Stkalign> for bool {
    #[inline(always)]
    fn from(variant: Stkalign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STKALIGN` reader - Indicates stack alignment on exception entry"]
pub type StkalignR = crate::BitReader<Stkalign>;
impl StkalignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stkalign {
        match self.bits {
            false => Stkalign::_0,
            true => Stkalign::_1,
        }
    }
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stkalign::_0
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stkalign::_1
    }
}
#[doc = "Field `STKALIGN` writer - Indicates stack alignment on exception entry"]
pub type StkalignW<'a, REG> = crate::BitWriter<'a, REG, Stkalign>;
impl<'a, REG> StkalignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stkalign::_0)
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stkalign::_1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NonbasethrdenaR {
        NonbasethrdenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> UsersetmpendR {
        UsersetmpendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UnalignTrpR {
        UnalignTrpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> Div0TrpR {
        Div0TrpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BfhfnmignR {
        BfhfnmignR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> StkalignR {
        StkalignR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn nonbasethrdena(&mut self) -> NonbasethrdenaW<CcrSpec> {
        NonbasethrdenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline(always)]
    #[must_use]
    pub fn usersetmpend(&mut self) -> UsersetmpendW<CcrSpec> {
        UsersetmpendW::new(self, 1)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    #[must_use]
    pub fn unalign_trp(&mut self) -> UnalignTrpW<CcrSpec> {
        UnalignTrpW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    #[must_use]
    pub fn div_0_trp(&mut self) -> Div0TrpW<CcrSpec> {
        Div0TrpW::new(self, 4)
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmign(&mut self) -> BfhfnmignW<CcrSpec> {
        BfhfnmignW::new(self, 8)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn stkalign(&mut self) -> StkalignW<CcrSpec> {
        StkalignW::new(self, 9)
    }
}
#[doc = "Configuration and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
