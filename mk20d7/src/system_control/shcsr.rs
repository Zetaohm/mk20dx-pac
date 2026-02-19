#[doc = "Register `SHCSR` reader"]
pub type R = crate::R<ShcsrSpec>;
#[doc = "Register `SHCSR` writer"]
pub type W = crate::W<ShcsrSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memfaultact {
    #[doc = "0: exception is not active"]
    _0 = 0,
    #[doc = "1: exception is active"]
    _1 = 1,
}
impl From<Memfaultact> for bool {
    #[inline(always)]
    fn from(variant: Memfaultact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTACT` reader - no description available"]
pub type MemfaultactR = crate::BitReader<Memfaultact>;
impl MemfaultactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memfaultact {
        match self.bits {
            false => Memfaultact::_0,
            true => Memfaultact::_1,
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Memfaultact::_0
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Memfaultact::_1
    }
}
#[doc = "Field `MEMFAULTACT` writer - no description available"]
pub type MemfaultactW<'a, REG> = crate::BitWriter<'a, REG, Memfaultact>;
impl<'a, REG> MemfaultactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Memfaultact::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Memfaultact::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busfaultact {
    #[doc = "0: exception is not active"]
    _0 = 0,
    #[doc = "1: exception is active"]
    _1 = 1,
}
impl From<Busfaultact> for bool {
    #[inline(always)]
    fn from(variant: Busfaultact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTACT` reader - no description available"]
pub type BusfaultactR = crate::BitReader<Busfaultact>;
impl BusfaultactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busfaultact {
        match self.bits {
            false => Busfaultact::_0,
            true => Busfaultact::_1,
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busfaultact::_0
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busfaultact::_1
    }
}
#[doc = "Field `BUSFAULTACT` writer - no description available"]
pub type BusfaultactW<'a, REG> = crate::BitWriter<'a, REG, Busfaultact>;
impl<'a, REG> BusfaultactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busfaultact::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busfaultact::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgfaultact {
    #[doc = "0: exception is not active"]
    _0 = 0,
    #[doc = "1: exception is active"]
    _1 = 1,
}
impl From<Usgfaultact> for bool {
    #[inline(always)]
    fn from(variant: Usgfaultact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTACT` reader - no description available"]
pub type UsgfaultactR = crate::BitReader<Usgfaultact>;
impl UsgfaultactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usgfaultact {
        match self.bits {
            false => Usgfaultact::_0,
            true => Usgfaultact::_1,
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgfaultact::_0
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgfaultact::_1
    }
}
#[doc = "Field `USGFAULTACT` writer - no description available"]
pub type UsgfaultactW<'a, REG> = crate::BitWriter<'a, REG, Usgfaultact>;
impl<'a, REG> UsgfaultactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgfaultact::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgfaultact::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcallact {
    #[doc = "0: exception is not active"]
    _0 = 0,
    #[doc = "1: exception is active"]
    _1 = 1,
}
impl From<Svcallact> for bool {
    #[inline(always)]
    fn from(variant: Svcallact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCALLACT` reader - no description available"]
pub type SvcallactR = crate::BitReader<Svcallact>;
impl SvcallactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svcallact {
        match self.bits {
            false => Svcallact::_0,
            true => Svcallact::_1,
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Svcallact::_0
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Svcallact::_1
    }
}
#[doc = "Field `SVCALLACT` writer - no description available"]
pub type SvcallactW<'a, REG> = crate::BitWriter<'a, REG, Svcallact>;
impl<'a, REG> SvcallactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Svcallact::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Svcallact::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monitoract {
    #[doc = "0: exception is not active"]
    _0 = 0,
    #[doc = "1: exception is active"]
    _1 = 1,
}
impl From<Monitoract> for bool {
    #[inline(always)]
    fn from(variant: Monitoract) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONITORACT` reader - no description available"]
pub type MonitoractR = crate::BitReader<Monitoract>;
impl MonitoractR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monitoract {
        match self.bits {
            false => Monitoract::_0,
            true => Monitoract::_1,
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Monitoract::_0
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Monitoract::_1
    }
}
#[doc = "Field `MONITORACT` writer - no description available"]
pub type MonitoractW<'a, REG> = crate::BitWriter<'a, REG, Monitoract>;
impl<'a, REG> MonitoractW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Monitoract::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Monitoract::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendsvact {
    #[doc = "0: exception is not active"]
    _0 = 0,
    #[doc = "1: exception is active"]
    _1 = 1,
}
impl From<Pendsvact> for bool {
    #[inline(always)]
    fn from(variant: Pendsvact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVACT` reader - no description available"]
pub type PendsvactR = crate::BitReader<Pendsvact>;
impl PendsvactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pendsvact {
        match self.bits {
            false => Pendsvact::_0,
            true => Pendsvact::_1,
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pendsvact::_0
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pendsvact::_1
    }
}
#[doc = "Field `PENDSVACT` writer - no description available"]
pub type PendsvactW<'a, REG> = crate::BitWriter<'a, REG, Pendsvact>;
impl<'a, REG> PendsvactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvact::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvact::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systickact {
    #[doc = "0: exception is not active"]
    _0 = 0,
    #[doc = "1: exception is active"]
    _1 = 1,
}
impl From<Systickact> for bool {
    #[inline(always)]
    fn from(variant: Systickact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTICKACT` reader - no description available"]
pub type SystickactR = crate::BitReader<Systickact>;
impl SystickactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systickact {
        match self.bits {
            false => Systickact::_0,
            true => Systickact::_1,
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Systickact::_0
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Systickact::_1
    }
}
#[doc = "Field `SYSTICKACT` writer - no description available"]
pub type SystickactW<'a, REG> = crate::BitWriter<'a, REG, Systickact>;
impl<'a, REG> SystickactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Systickact::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Systickact::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgfaultpended {
    #[doc = "0: exception is not pending"]
    _0 = 0,
    #[doc = "1: exception is pending"]
    _1 = 1,
}
impl From<Usgfaultpended> for bool {
    #[inline(always)]
    fn from(variant: Usgfaultpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTPENDED` reader - no description available"]
pub type UsgfaultpendedR = crate::BitReader<Usgfaultpended>;
impl UsgfaultpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usgfaultpended {
        match self.bits {
            false => Usgfaultpended::_0,
            true => Usgfaultpended::_1,
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgfaultpended::_0
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgfaultpended::_1
    }
}
#[doc = "Field `USGFAULTPENDED` writer - no description available"]
pub type UsgfaultpendedW<'a, REG> = crate::BitWriter<'a, REG, Usgfaultpended>;
impl<'a, REG> UsgfaultpendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgfaultpended::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgfaultpended::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memfaultpended {
    #[doc = "0: exception is not pending"]
    _0 = 0,
    #[doc = "1: exception is pending"]
    _1 = 1,
}
impl From<Memfaultpended> for bool {
    #[inline(always)]
    fn from(variant: Memfaultpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTPENDED` reader - no description available"]
pub type MemfaultpendedR = crate::BitReader<Memfaultpended>;
impl MemfaultpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memfaultpended {
        match self.bits {
            false => Memfaultpended::_0,
            true => Memfaultpended::_1,
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Memfaultpended::_0
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Memfaultpended::_1
    }
}
#[doc = "Field `MEMFAULTPENDED` writer - no description available"]
pub type MemfaultpendedW<'a, REG> = crate::BitWriter<'a, REG, Memfaultpended>;
impl<'a, REG> MemfaultpendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Memfaultpended::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Memfaultpended::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busfaultpended {
    #[doc = "0: exception is not pending"]
    _0 = 0,
    #[doc = "1: exception is pending"]
    _1 = 1,
}
impl From<Busfaultpended> for bool {
    #[inline(always)]
    fn from(variant: Busfaultpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTPENDED` reader - no description available"]
pub type BusfaultpendedR = crate::BitReader<Busfaultpended>;
impl BusfaultpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busfaultpended {
        match self.bits {
            false => Busfaultpended::_0,
            true => Busfaultpended::_1,
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busfaultpended::_0
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busfaultpended::_1
    }
}
#[doc = "Field `BUSFAULTPENDED` writer - no description available"]
pub type BusfaultpendedW<'a, REG> = crate::BitWriter<'a, REG, Busfaultpended>;
impl<'a, REG> BusfaultpendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busfaultpended::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busfaultpended::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcallpended {
    #[doc = "0: exception is not pending"]
    _0 = 0,
    #[doc = "1: exception is pending"]
    _1 = 1,
}
impl From<Svcallpended> for bool {
    #[inline(always)]
    fn from(variant: Svcallpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCALLPENDED` reader - no description available"]
pub type SvcallpendedR = crate::BitReader<Svcallpended>;
impl SvcallpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svcallpended {
        match self.bits {
            false => Svcallpended::_0,
            true => Svcallpended::_1,
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Svcallpended::_0
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Svcallpended::_1
    }
}
#[doc = "Field `SVCALLPENDED` writer - no description available"]
pub type SvcallpendedW<'a, REG> = crate::BitWriter<'a, REG, Svcallpended>;
impl<'a, REG> SvcallpendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Svcallpended::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Svcallpended::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memfaultena {
    #[doc = "0: disable the exception"]
    _0 = 0,
    #[doc = "1: enable the exception"]
    _1 = 1,
}
impl From<Memfaultena> for bool {
    #[inline(always)]
    fn from(variant: Memfaultena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTENA` reader - no description available"]
pub type MemfaultenaR = crate::BitReader<Memfaultena>;
impl MemfaultenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memfaultena {
        match self.bits {
            false => Memfaultena::_0,
            true => Memfaultena::_1,
        }
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Memfaultena::_0
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Memfaultena::_1
    }
}
#[doc = "Field `MEMFAULTENA` writer - no description available"]
pub type MemfaultenaW<'a, REG> = crate::BitWriter<'a, REG, Memfaultena>;
impl<'a, REG> MemfaultenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Memfaultena::_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Memfaultena::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busfaultena {
    #[doc = "0: disable the exception"]
    _0 = 0,
    #[doc = "1: enable the exception"]
    _1 = 1,
}
impl From<Busfaultena> for bool {
    #[inline(always)]
    fn from(variant: Busfaultena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTENA` reader - no description available"]
pub type BusfaultenaR = crate::BitReader<Busfaultena>;
impl BusfaultenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busfaultena {
        match self.bits {
            false => Busfaultena::_0,
            true => Busfaultena::_1,
        }
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busfaultena::_0
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busfaultena::_1
    }
}
#[doc = "Field `BUSFAULTENA` writer - no description available"]
pub type BusfaultenaW<'a, REG> = crate::BitWriter<'a, REG, Busfaultena>;
impl<'a, REG> BusfaultenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busfaultena::_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busfaultena::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgfaultena {
    #[doc = "0: disable the exception"]
    _0 = 0,
    #[doc = "1: enable the exception"]
    _1 = 1,
}
impl From<Usgfaultena> for bool {
    #[inline(always)]
    fn from(variant: Usgfaultena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTENA` reader - no description available"]
pub type UsgfaultenaR = crate::BitReader<Usgfaultena>;
impl UsgfaultenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usgfaultena {
        match self.bits {
            false => Usgfaultena::_0,
            true => Usgfaultena::_1,
        }
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgfaultena::_0
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgfaultena::_1
    }
}
#[doc = "Field `USGFAULTENA` writer - no description available"]
pub type UsgfaultenaW<'a, REG> = crate::BitWriter<'a, REG, Usgfaultena>;
impl<'a, REG> UsgfaultenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgfaultena::_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgfaultena::_1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MemfaultactR {
        MemfaultactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BusfaultactR {
        BusfaultactR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> UsgfaultactR {
        UsgfaultactR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn svcallact(&self) -> SvcallactR {
        SvcallactR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn monitoract(&self) -> MonitoractR {
        MonitoractR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PendsvactR {
        PendsvactR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn systickact(&self) -> SystickactR {
        SystickactR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> UsgfaultpendedR {
        UsgfaultpendedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MemfaultpendedR {
        MemfaultpendedR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BusfaultpendedR {
        BusfaultpendedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SvcallpendedR {
        SvcallpendedR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MemfaultenaR {
        MemfaultenaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BusfaultenaR {
        BusfaultenaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> UsgfaultenaR {
        UsgfaultenaR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultact(&mut self) -> MemfaultactW<ShcsrSpec> {
        MemfaultactW::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultact(&mut self) -> BusfaultactW<ShcsrSpec> {
        BusfaultactW::new(self, 1)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultact(&mut self) -> UsgfaultactW<ShcsrSpec> {
        UsgfaultactW::new(self, 3)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn svcallact(&mut self) -> SvcallactW<ShcsrSpec> {
        SvcallactW::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn monitoract(&mut self) -> MonitoractW<ShcsrSpec> {
        MonitoractW::new(self, 8)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvact(&mut self) -> PendsvactW<ShcsrSpec> {
        PendsvactW::new(self, 10)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn systickact(&mut self) -> SystickactW<ShcsrSpec> {
        SystickactW::new(self, 11)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultpended(&mut self) -> UsgfaultpendedW<ShcsrSpec> {
        UsgfaultpendedW::new(self, 12)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultpended(&mut self) -> MemfaultpendedW<ShcsrSpec> {
        MemfaultpendedW::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultpended(&mut self) -> BusfaultpendedW<ShcsrSpec> {
        BusfaultpendedW::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn svcallpended(&mut self) -> SvcallpendedW<ShcsrSpec> {
        SvcallpendedW::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultena(&mut self) -> MemfaultenaW<ShcsrSpec> {
        MemfaultenaW::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultena(&mut self) -> BusfaultenaW<ShcsrSpec> {
        BusfaultenaW::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultena(&mut self) -> UsgfaultenaW<ShcsrSpec> {
        UsgfaultenaW::new(self, 18)
    }
}
#[doc = "System Handler Control and State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShcsrSpec;
impl crate::RegisterSpec for ShcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcsr::R`](R) reader structure"]
impl crate::Readable for ShcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`shcsr::W`](W) writer structure"]
impl crate::Writable for ShcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for ShcsrSpec {
    const RESET_VALUE: u32 = 0;
}
