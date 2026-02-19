#[doc = "Register `PACRP` reader"]
pub type R = crate::R<PacrpSpec>;
#[doc = "Register `PACRP` writer"]
pub type W = crate::W<PacrpSpec>;
#[doc = "Trusted protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp7 {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<Tp7> for bool {
    #[inline(always)]
    fn from(variant: Tp7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP7` reader - Trusted protect"]
pub type Tp7R = crate::BitReader<Tp7>;
impl Tp7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tp7 {
        match self.bits {
            false => Tp7::_0,
            true => Tp7::_1,
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tp7::_0
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tp7::_1
    }
}
#[doc = "Field `TP7` writer - Trusted protect"]
pub type Tp7W<'a, REG> = crate::BitWriter<'a, REG, Tp7>;
impl<'a, REG> Tp7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tp7::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tp7::_1)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp7 {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<Wp7> for bool {
    #[inline(always)]
    fn from(variant: Wp7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP7` reader - Write protect"]
pub type Wp7R = crate::BitReader<Wp7>;
impl Wp7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp7 {
        match self.bits {
            false => Wp7::_0,
            true => Wp7::_1,
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp7::_0
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp7::_1
    }
}
#[doc = "Field `WP7` writer - Write protect"]
pub type Wp7W<'a, REG> = crate::BitWriter<'a, REG, Wp7>;
impl<'a, REG> Wp7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7::_1)
    }
}
#[doc = "Supervisor protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp7 {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<Sp7> for bool {
    #[inline(always)]
    fn from(variant: Sp7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP7` reader - Supervisor protect"]
pub type Sp7R = crate::BitReader<Sp7>;
impl Sp7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp7 {
        match self.bits {
            false => Sp7::_0,
            true => Sp7::_1,
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp7::_0
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp7::_1
    }
}
#[doc = "Field `SP7` writer - Supervisor protect"]
pub type Sp7W<'a, REG> = crate::BitWriter<'a, REG, Sp7>;
impl<'a, REG> Sp7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp7::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp7::_1)
    }
}
#[doc = "Trusted protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp6 {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<Tp6> for bool {
    #[inline(always)]
    fn from(variant: Tp6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP6` reader - Trusted protect"]
pub type Tp6R = crate::BitReader<Tp6>;
impl Tp6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tp6 {
        match self.bits {
            false => Tp6::_0,
            true => Tp6::_1,
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tp6::_0
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tp6::_1
    }
}
#[doc = "Field `TP6` writer - Trusted protect"]
pub type Tp6W<'a, REG> = crate::BitWriter<'a, REG, Tp6>;
impl<'a, REG> Tp6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tp6::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tp6::_1)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp6 {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<Wp6> for bool {
    #[inline(always)]
    fn from(variant: Wp6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP6` reader - Write protect"]
pub type Wp6R = crate::BitReader<Wp6>;
impl Wp6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp6 {
        match self.bits {
            false => Wp6::_0,
            true => Wp6::_1,
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp6::_0
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp6::_1
    }
}
#[doc = "Field `WP6` writer - Write protect"]
pub type Wp6W<'a, REG> = crate::BitWriter<'a, REG, Wp6>;
impl<'a, REG> Wp6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6::_1)
    }
}
#[doc = "Supervisor protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp6 {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<Sp6> for bool {
    #[inline(always)]
    fn from(variant: Sp6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP6` reader - Supervisor protect"]
pub type Sp6R = crate::BitReader<Sp6>;
impl Sp6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp6 {
        match self.bits {
            false => Sp6::_0,
            true => Sp6::_1,
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp6::_0
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp6::_1
    }
}
#[doc = "Field `SP6` writer - Supervisor protect"]
pub type Sp6W<'a, REG> = crate::BitWriter<'a, REG, Sp6>;
impl<'a, REG> Sp6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp6::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp6::_1)
    }
}
#[doc = "Trusted protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp5 {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<Tp5> for bool {
    #[inline(always)]
    fn from(variant: Tp5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP5` reader - Trusted protect"]
pub type Tp5R = crate::BitReader<Tp5>;
impl Tp5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tp5 {
        match self.bits {
            false => Tp5::_0,
            true => Tp5::_1,
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tp5::_0
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tp5::_1
    }
}
#[doc = "Field `TP5` writer - Trusted protect"]
pub type Tp5W<'a, REG> = crate::BitWriter<'a, REG, Tp5>;
impl<'a, REG> Tp5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tp5::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tp5::_1)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp5 {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<Wp5> for bool {
    #[inline(always)]
    fn from(variant: Wp5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP5` reader - Write protect"]
pub type Wp5R = crate::BitReader<Wp5>;
impl Wp5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp5 {
        match self.bits {
            false => Wp5::_0,
            true => Wp5::_1,
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp5::_0
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp5::_1
    }
}
#[doc = "Field `WP5` writer - Write protect"]
pub type Wp5W<'a, REG> = crate::BitWriter<'a, REG, Wp5>;
impl<'a, REG> Wp5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5::_1)
    }
}
#[doc = "Supervisor protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp5 {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<Sp5> for bool {
    #[inline(always)]
    fn from(variant: Sp5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP5` reader - Supervisor protect"]
pub type Sp5R = crate::BitReader<Sp5>;
impl Sp5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp5 {
        match self.bits {
            false => Sp5::_0,
            true => Sp5::_1,
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp5::_0
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp5::_1
    }
}
#[doc = "Field `SP5` writer - Supervisor protect"]
pub type Sp5W<'a, REG> = crate::BitWriter<'a, REG, Sp5>;
impl<'a, REG> Sp5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp5::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp5::_1)
    }
}
#[doc = "Trusted protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp4 {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<Tp4> for bool {
    #[inline(always)]
    fn from(variant: Tp4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP4` reader - Trusted protect"]
pub type Tp4R = crate::BitReader<Tp4>;
impl Tp4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tp4 {
        match self.bits {
            false => Tp4::_0,
            true => Tp4::_1,
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tp4::_0
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tp4::_1
    }
}
#[doc = "Field `TP4` writer - Trusted protect"]
pub type Tp4W<'a, REG> = crate::BitWriter<'a, REG, Tp4>;
impl<'a, REG> Tp4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tp4::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tp4::_1)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp4 {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<Wp4> for bool {
    #[inline(always)]
    fn from(variant: Wp4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP4` reader - Write protect"]
pub type Wp4R = crate::BitReader<Wp4>;
impl Wp4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp4 {
        match self.bits {
            false => Wp4::_0,
            true => Wp4::_1,
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp4::_0
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp4::_1
    }
}
#[doc = "Field `WP4` writer - Write protect"]
pub type Wp4W<'a, REG> = crate::BitWriter<'a, REG, Wp4>;
impl<'a, REG> Wp4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4::_1)
    }
}
#[doc = "Supervisor protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp4 {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<Sp4> for bool {
    #[inline(always)]
    fn from(variant: Sp4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP4` reader - Supervisor protect"]
pub type Sp4R = crate::BitReader<Sp4>;
impl Sp4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp4 {
        match self.bits {
            false => Sp4::_0,
            true => Sp4::_1,
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp4::_0
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp4::_1
    }
}
#[doc = "Field `SP4` writer - Supervisor protect"]
pub type Sp4W<'a, REG> = crate::BitWriter<'a, REG, Sp4>;
impl<'a, REG> Sp4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp4::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp4::_1)
    }
}
#[doc = "Trusted protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp3 {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<Tp3> for bool {
    #[inline(always)]
    fn from(variant: Tp3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP3` reader - Trusted protect"]
pub type Tp3R = crate::BitReader<Tp3>;
impl Tp3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tp3 {
        match self.bits {
            false => Tp3::_0,
            true => Tp3::_1,
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tp3::_0
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tp3::_1
    }
}
#[doc = "Field `TP3` writer - Trusted protect"]
pub type Tp3W<'a, REG> = crate::BitWriter<'a, REG, Tp3>;
impl<'a, REG> Tp3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tp3::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tp3::_1)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp3 {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<Wp3> for bool {
    #[inline(always)]
    fn from(variant: Wp3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP3` reader - Write protect"]
pub type Wp3R = crate::BitReader<Wp3>;
impl Wp3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp3 {
        match self.bits {
            false => Wp3::_0,
            true => Wp3::_1,
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp3::_0
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp3::_1
    }
}
#[doc = "Field `WP3` writer - Write protect"]
pub type Wp3W<'a, REG> = crate::BitWriter<'a, REG, Wp3>;
impl<'a, REG> Wp3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3::_1)
    }
}
#[doc = "Supervisor protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp3 {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<Sp3> for bool {
    #[inline(always)]
    fn from(variant: Sp3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP3` reader - Supervisor protect"]
pub type Sp3R = crate::BitReader<Sp3>;
impl Sp3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp3 {
        match self.bits {
            false => Sp3::_0,
            true => Sp3::_1,
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp3::_0
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp3::_1
    }
}
#[doc = "Field `SP3` writer - Supervisor protect"]
pub type Sp3W<'a, REG> = crate::BitWriter<'a, REG, Sp3>;
impl<'a, REG> Sp3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp3::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp3::_1)
    }
}
#[doc = "Trusted protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp2 {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<Tp2> for bool {
    #[inline(always)]
    fn from(variant: Tp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP2` reader - Trusted protect"]
pub type Tp2R = crate::BitReader<Tp2>;
impl Tp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tp2 {
        match self.bits {
            false => Tp2::_0,
            true => Tp2::_1,
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tp2::_0
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tp2::_1
    }
}
#[doc = "Field `TP2` writer - Trusted protect"]
pub type Tp2W<'a, REG> = crate::BitWriter<'a, REG, Tp2>;
impl<'a, REG> Tp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tp2::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tp2::_1)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp2 {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<Wp2> for bool {
    #[inline(always)]
    fn from(variant: Wp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP2` reader - Write protect"]
pub type Wp2R = crate::BitReader<Wp2>;
impl Wp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp2 {
        match self.bits {
            false => Wp2::_0,
            true => Wp2::_1,
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp2::_0
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp2::_1
    }
}
#[doc = "Field `WP2` writer - Write protect"]
pub type Wp2W<'a, REG> = crate::BitWriter<'a, REG, Wp2>;
impl<'a, REG> Wp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2::_1)
    }
}
#[doc = "Supervisor protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp2 {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<Sp2> for bool {
    #[inline(always)]
    fn from(variant: Sp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP2` reader - Supervisor protect"]
pub type Sp2R = crate::BitReader<Sp2>;
impl Sp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp2 {
        match self.bits {
            false => Sp2::_0,
            true => Sp2::_1,
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp2::_0
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp2::_1
    }
}
#[doc = "Field `SP2` writer - Supervisor protect"]
pub type Sp2W<'a, REG> = crate::BitWriter<'a, REG, Sp2>;
impl<'a, REG> Sp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp2::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp2::_1)
    }
}
#[doc = "Trusted protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp1 {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<Tp1> for bool {
    #[inline(always)]
    fn from(variant: Tp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP1` reader - Trusted protect"]
pub type Tp1R = crate::BitReader<Tp1>;
impl Tp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tp1 {
        match self.bits {
            false => Tp1::_0,
            true => Tp1::_1,
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tp1::_0
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tp1::_1
    }
}
#[doc = "Field `TP1` writer - Trusted protect"]
pub type Tp1W<'a, REG> = crate::BitWriter<'a, REG, Tp1>;
impl<'a, REG> Tp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tp1::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tp1::_1)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp1 {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<Wp1> for bool {
    #[inline(always)]
    fn from(variant: Wp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP1` reader - Write protect"]
pub type Wp1R = crate::BitReader<Wp1>;
impl Wp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp1 {
        match self.bits {
            false => Wp1::_0,
            true => Wp1::_1,
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp1::_0
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp1::_1
    }
}
#[doc = "Field `WP1` writer - Write protect"]
pub type Wp1W<'a, REG> = crate::BitWriter<'a, REG, Wp1>;
impl<'a, REG> Wp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1::_1)
    }
}
#[doc = "Supervisor protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp1 {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<Sp1> for bool {
    #[inline(always)]
    fn from(variant: Sp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP1` reader - Supervisor protect"]
pub type Sp1R = crate::BitReader<Sp1>;
impl Sp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp1 {
        match self.bits {
            false => Sp1::_0,
            true => Sp1::_1,
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp1::_0
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp1::_1
    }
}
#[doc = "Field `SP1` writer - Supervisor protect"]
pub type Sp1W<'a, REG> = crate::BitWriter<'a, REG, Sp1>;
impl<'a, REG> Sp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp1::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp1::_1)
    }
}
#[doc = "Trusted protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp0 {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<Tp0> for bool {
    #[inline(always)]
    fn from(variant: Tp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP0` reader - Trusted protect"]
pub type Tp0R = crate::BitReader<Tp0>;
impl Tp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tp0 {
        match self.bits {
            false => Tp0::_0,
            true => Tp0::_1,
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tp0::_0
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tp0::_1
    }
}
#[doc = "Field `TP0` writer - Trusted protect"]
pub type Tp0W<'a, REG> = crate::BitWriter<'a, REG, Tp0>;
impl<'a, REG> Tp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tp0::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tp0::_1)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp0 {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<Wp0> for bool {
    #[inline(always)]
    fn from(variant: Wp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP0` reader - Write protect"]
pub type Wp0R = crate::BitReader<Wp0>;
impl Wp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp0 {
        match self.bits {
            false => Wp0::_0,
            true => Wp0::_1,
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp0::_0
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp0::_1
    }
}
#[doc = "Field `WP0` writer - Write protect"]
pub type Wp0W<'a, REG> = crate::BitWriter<'a, REG, Wp0>;
impl<'a, REG> Wp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0::_1)
    }
}
#[doc = "Supervisor protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp0 {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<Sp0> for bool {
    #[inline(always)]
    fn from(variant: Sp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP0` reader - Supervisor protect"]
pub type Sp0R = crate::BitReader<Sp0>;
impl Sp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp0 {
        match self.bits {
            false => Sp0::_0,
            true => Sp0::_1,
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp0::_0
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp0::_1
    }
}
#[doc = "Field `SP0` writer - Supervisor protect"]
pub type Sp0W<'a, REG> = crate::BitWriter<'a, REG, Sp0>;
impl<'a, REG> Sp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp0::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp0::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Trusted protect"]
    #[inline(always)]
    pub fn tp7(&self) -> Tp7R {
        Tp7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write protect"]
    #[inline(always)]
    pub fn wp7(&self) -> Wp7R {
        Wp7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Supervisor protect"]
    #[inline(always)]
    pub fn sp7(&self) -> Sp7R {
        Sp7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Trusted protect"]
    #[inline(always)]
    pub fn tp6(&self) -> Tp6R {
        Tp6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write protect"]
    #[inline(always)]
    pub fn wp6(&self) -> Wp6R {
        Wp6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Supervisor protect"]
    #[inline(always)]
    pub fn sp6(&self) -> Sp6R {
        Sp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Trusted protect"]
    #[inline(always)]
    pub fn tp5(&self) -> Tp5R {
        Tp5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write protect"]
    #[inline(always)]
    pub fn wp5(&self) -> Wp5R {
        Wp5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Supervisor protect"]
    #[inline(always)]
    pub fn sp5(&self) -> Sp5R {
        Sp5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Trusted protect"]
    #[inline(always)]
    pub fn tp4(&self) -> Tp4R {
        Tp4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write protect"]
    #[inline(always)]
    pub fn wp4(&self) -> Wp4R {
        Wp4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Supervisor protect"]
    #[inline(always)]
    pub fn sp4(&self) -> Sp4R {
        Sp4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Trusted protect"]
    #[inline(always)]
    pub fn tp3(&self) -> Tp3R {
        Tp3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write protect"]
    #[inline(always)]
    pub fn wp3(&self) -> Wp3R {
        Wp3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Supervisor protect"]
    #[inline(always)]
    pub fn sp3(&self) -> Sp3R {
        Sp3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Trusted protect"]
    #[inline(always)]
    pub fn tp2(&self) -> Tp2R {
        Tp2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect"]
    #[inline(always)]
    pub fn wp2(&self) -> Wp2R {
        Wp2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Supervisor protect"]
    #[inline(always)]
    pub fn sp2(&self) -> Sp2R {
        Sp2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Trusted protect"]
    #[inline(always)]
    pub fn tp1(&self) -> Tp1R {
        Tp1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write protect"]
    #[inline(always)]
    pub fn wp1(&self) -> Wp1R {
        Wp1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Supervisor protect"]
    #[inline(always)]
    pub fn sp1(&self) -> Sp1R {
        Sp1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Trusted protect"]
    #[inline(always)]
    pub fn tp0(&self) -> Tp0R {
        Tp0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write protect"]
    #[inline(always)]
    pub fn wp0(&self) -> Wp0R {
        Wp0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Supervisor protect"]
    #[inline(always)]
    pub fn sp0(&self) -> Sp0R {
        Sp0R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trusted protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp7(&mut self) -> Tp7W<PacrpSpec> {
        Tp7W::new(self, 0)
    }
    #[doc = "Bit 1 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp7(&mut self) -> Wp7W<PacrpSpec> {
        Wp7W::new(self, 1)
    }
    #[doc = "Bit 2 - Supervisor protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp7(&mut self) -> Sp7W<PacrpSpec> {
        Sp7W::new(self, 2)
    }
    #[doc = "Bit 4 - Trusted protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp6(&mut self) -> Tp6W<PacrpSpec> {
        Tp6W::new(self, 4)
    }
    #[doc = "Bit 5 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp6(&mut self) -> Wp6W<PacrpSpec> {
        Wp6W::new(self, 5)
    }
    #[doc = "Bit 6 - Supervisor protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp6(&mut self) -> Sp6W<PacrpSpec> {
        Sp6W::new(self, 6)
    }
    #[doc = "Bit 8 - Trusted protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp5(&mut self) -> Tp5W<PacrpSpec> {
        Tp5W::new(self, 8)
    }
    #[doc = "Bit 9 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp5(&mut self) -> Wp5W<PacrpSpec> {
        Wp5W::new(self, 9)
    }
    #[doc = "Bit 10 - Supervisor protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp5(&mut self) -> Sp5W<PacrpSpec> {
        Sp5W::new(self, 10)
    }
    #[doc = "Bit 12 - Trusted protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp4(&mut self) -> Tp4W<PacrpSpec> {
        Tp4W::new(self, 12)
    }
    #[doc = "Bit 13 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> Wp4W<PacrpSpec> {
        Wp4W::new(self, 13)
    }
    #[doc = "Bit 14 - Supervisor protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp4(&mut self) -> Sp4W<PacrpSpec> {
        Sp4W::new(self, 14)
    }
    #[doc = "Bit 16 - Trusted protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp3(&mut self) -> Tp3W<PacrpSpec> {
        Tp3W::new(self, 16)
    }
    #[doc = "Bit 17 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> Wp3W<PacrpSpec> {
        Wp3W::new(self, 17)
    }
    #[doc = "Bit 18 - Supervisor protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp3(&mut self) -> Sp3W<PacrpSpec> {
        Sp3W::new(self, 18)
    }
    #[doc = "Bit 20 - Trusted protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp2(&mut self) -> Tp2W<PacrpSpec> {
        Tp2W::new(self, 20)
    }
    #[doc = "Bit 21 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> Wp2W<PacrpSpec> {
        Wp2W::new(self, 21)
    }
    #[doc = "Bit 22 - Supervisor protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp2(&mut self) -> Sp2W<PacrpSpec> {
        Sp2W::new(self, 22)
    }
    #[doc = "Bit 24 - Trusted protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp1(&mut self) -> Tp1W<PacrpSpec> {
        Tp1W::new(self, 24)
    }
    #[doc = "Bit 25 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> Wp1W<PacrpSpec> {
        Wp1W::new(self, 25)
    }
    #[doc = "Bit 26 - Supervisor protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp1(&mut self) -> Sp1W<PacrpSpec> {
        Sp1W::new(self, 26)
    }
    #[doc = "Bit 28 - Trusted protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp0(&mut self) -> Tp0W<PacrpSpec> {
        Tp0W::new(self, 28)
    }
    #[doc = "Bit 29 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp0(&mut self) -> Wp0W<PacrpSpec> {
        Wp0W::new(self, 29)
    }
    #[doc = "Bit 30 - Supervisor protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp0(&mut self) -> Sp0W<PacrpSpec> {
        Sp0W::new(self, 30)
    }
}
#[doc = "Peripheral Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pacrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pacrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PacrpSpec;
impl crate::RegisterSpec for PacrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pacrp::R`](R) reader structure"]
impl crate::Readable for PacrpSpec {}
#[doc = "`write(|w| ..)` method takes [`pacrp::W`](W) writer structure"]
impl crate::Writable for PacrpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PACRP to value 0"]
impl crate::Resettable for PacrpSpec {
    const RESET_VALUE: u32 = 0;
}
