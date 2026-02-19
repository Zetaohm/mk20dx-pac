#[doc = "Register `HRS` reader"]
pub type R = crate::R<HrsSpec>;
#[doc = "Register `HRS` writer"]
pub type W = crate::W<HrsSpec>;
#[doc = "Hardware Request Status Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs0 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs0> for bool {
    #[inline(always)]
    fn from(variant: Hrs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS0` reader - Hardware Request Status Channel 0"]
pub type Hrs0R = crate::BitReader<Hrs0>;
impl Hrs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs0 {
        match self.bits {
            false => Hrs0::_0,
            true => Hrs0::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs0::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs0::_1
    }
}
#[doc = "Field `HRS0` writer - Hardware Request Status Channel 0"]
pub type Hrs0W<'a, REG> = crate::BitWriter<'a, REG, Hrs0>;
impl<'a, REG> Hrs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs0::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs0::_1)
    }
}
#[doc = "Hardware Request Status Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs1 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs1> for bool {
    #[inline(always)]
    fn from(variant: Hrs1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS1` reader - Hardware Request Status Channel 1"]
pub type Hrs1R = crate::BitReader<Hrs1>;
impl Hrs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs1 {
        match self.bits {
            false => Hrs1::_0,
            true => Hrs1::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs1::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs1::_1
    }
}
#[doc = "Field `HRS1` writer - Hardware Request Status Channel 1"]
pub type Hrs1W<'a, REG> = crate::BitWriter<'a, REG, Hrs1>;
impl<'a, REG> Hrs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs1::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs1::_1)
    }
}
#[doc = "Hardware Request Status Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs2 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs2> for bool {
    #[inline(always)]
    fn from(variant: Hrs2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS2` reader - Hardware Request Status Channel 2"]
pub type Hrs2R = crate::BitReader<Hrs2>;
impl Hrs2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs2 {
        match self.bits {
            false => Hrs2::_0,
            true => Hrs2::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs2::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs2::_1
    }
}
#[doc = "Field `HRS2` writer - Hardware Request Status Channel 2"]
pub type Hrs2W<'a, REG> = crate::BitWriter<'a, REG, Hrs2>;
impl<'a, REG> Hrs2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs2::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs2::_1)
    }
}
#[doc = "Hardware Request Status Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs3 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs3> for bool {
    #[inline(always)]
    fn from(variant: Hrs3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS3` reader - Hardware Request Status Channel 3"]
pub type Hrs3R = crate::BitReader<Hrs3>;
impl Hrs3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs3 {
        match self.bits {
            false => Hrs3::_0,
            true => Hrs3::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs3::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs3::_1
    }
}
#[doc = "Field `HRS3` writer - Hardware Request Status Channel 3"]
pub type Hrs3W<'a, REG> = crate::BitWriter<'a, REG, Hrs3>;
impl<'a, REG> Hrs3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs3::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs3::_1)
    }
}
#[doc = "Hardware Request Status Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs4 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs4> for bool {
    #[inline(always)]
    fn from(variant: Hrs4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS4` reader - Hardware Request Status Channel 4"]
pub type Hrs4R = crate::BitReader<Hrs4>;
impl Hrs4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs4 {
        match self.bits {
            false => Hrs4::_0,
            true => Hrs4::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs4::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs4::_1
    }
}
#[doc = "Field `HRS4` writer - Hardware Request Status Channel 4"]
pub type Hrs4W<'a, REG> = crate::BitWriter<'a, REG, Hrs4>;
impl<'a, REG> Hrs4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs4::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs4::_1)
    }
}
#[doc = "Hardware Request Status Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs5 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs5> for bool {
    #[inline(always)]
    fn from(variant: Hrs5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS5` reader - Hardware Request Status Channel 5"]
pub type Hrs5R = crate::BitReader<Hrs5>;
impl Hrs5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs5 {
        match self.bits {
            false => Hrs5::_0,
            true => Hrs5::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs5::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs5::_1
    }
}
#[doc = "Field `HRS5` writer - Hardware Request Status Channel 5"]
pub type Hrs5W<'a, REG> = crate::BitWriter<'a, REG, Hrs5>;
impl<'a, REG> Hrs5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs5::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs5::_1)
    }
}
#[doc = "Hardware Request Status Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs6 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs6> for bool {
    #[inline(always)]
    fn from(variant: Hrs6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS6` reader - Hardware Request Status Channel 6"]
pub type Hrs6R = crate::BitReader<Hrs6>;
impl Hrs6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs6 {
        match self.bits {
            false => Hrs6::_0,
            true => Hrs6::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs6::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs6::_1
    }
}
#[doc = "Field `HRS6` writer - Hardware Request Status Channel 6"]
pub type Hrs6W<'a, REG> = crate::BitWriter<'a, REG, Hrs6>;
impl<'a, REG> Hrs6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs6::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs6::_1)
    }
}
#[doc = "Hardware Request Status Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs7 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs7> for bool {
    #[inline(always)]
    fn from(variant: Hrs7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS7` reader - Hardware Request Status Channel 7"]
pub type Hrs7R = crate::BitReader<Hrs7>;
impl Hrs7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs7 {
        match self.bits {
            false => Hrs7::_0,
            true => Hrs7::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs7::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs7::_1
    }
}
#[doc = "Field `HRS7` writer - Hardware Request Status Channel 7"]
pub type Hrs7W<'a, REG> = crate::BitWriter<'a, REG, Hrs7>;
impl<'a, REG> Hrs7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs7::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs7::_1)
    }
}
#[doc = "Hardware Request Status Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs8 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs8> for bool {
    #[inline(always)]
    fn from(variant: Hrs8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS8` reader - Hardware Request Status Channel 8"]
pub type Hrs8R = crate::BitReader<Hrs8>;
impl Hrs8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs8 {
        match self.bits {
            false => Hrs8::_0,
            true => Hrs8::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs8::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs8::_1
    }
}
#[doc = "Field `HRS8` writer - Hardware Request Status Channel 8"]
pub type Hrs8W<'a, REG> = crate::BitWriter<'a, REG, Hrs8>;
impl<'a, REG> Hrs8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs8::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs8::_1)
    }
}
#[doc = "Hardware Request Status Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs9 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs9> for bool {
    #[inline(always)]
    fn from(variant: Hrs9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS9` reader - Hardware Request Status Channel 9"]
pub type Hrs9R = crate::BitReader<Hrs9>;
impl Hrs9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs9 {
        match self.bits {
            false => Hrs9::_0,
            true => Hrs9::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs9::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs9::_1
    }
}
#[doc = "Field `HRS9` writer - Hardware Request Status Channel 9"]
pub type Hrs9W<'a, REG> = crate::BitWriter<'a, REG, Hrs9>;
impl<'a, REG> Hrs9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs9::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs9::_1)
    }
}
#[doc = "Hardware Request Status Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs10 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs10> for bool {
    #[inline(always)]
    fn from(variant: Hrs10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS10` reader - Hardware Request Status Channel 10"]
pub type Hrs10R = crate::BitReader<Hrs10>;
impl Hrs10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs10 {
        match self.bits {
            false => Hrs10::_0,
            true => Hrs10::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs10::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs10::_1
    }
}
#[doc = "Field `HRS10` writer - Hardware Request Status Channel 10"]
pub type Hrs10W<'a, REG> = crate::BitWriter<'a, REG, Hrs10>;
impl<'a, REG> Hrs10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs10::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs10::_1)
    }
}
#[doc = "Hardware Request Status Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs11 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs11> for bool {
    #[inline(always)]
    fn from(variant: Hrs11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS11` reader - Hardware Request Status Channel 11"]
pub type Hrs11R = crate::BitReader<Hrs11>;
impl Hrs11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs11 {
        match self.bits {
            false => Hrs11::_0,
            true => Hrs11::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs11::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs11::_1
    }
}
#[doc = "Field `HRS11` writer - Hardware Request Status Channel 11"]
pub type Hrs11W<'a, REG> = crate::BitWriter<'a, REG, Hrs11>;
impl<'a, REG> Hrs11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs11::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs11::_1)
    }
}
#[doc = "Hardware Request Status Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs12 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs12> for bool {
    #[inline(always)]
    fn from(variant: Hrs12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS12` reader - Hardware Request Status Channel 12"]
pub type Hrs12R = crate::BitReader<Hrs12>;
impl Hrs12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs12 {
        match self.bits {
            false => Hrs12::_0,
            true => Hrs12::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs12::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs12::_1
    }
}
#[doc = "Field `HRS12` writer - Hardware Request Status Channel 12"]
pub type Hrs12W<'a, REG> = crate::BitWriter<'a, REG, Hrs12>;
impl<'a, REG> Hrs12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs12::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs12::_1)
    }
}
#[doc = "Hardware Request Status Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs13 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs13> for bool {
    #[inline(always)]
    fn from(variant: Hrs13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS13` reader - Hardware Request Status Channel 13"]
pub type Hrs13R = crate::BitReader<Hrs13>;
impl Hrs13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs13 {
        match self.bits {
            false => Hrs13::_0,
            true => Hrs13::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs13::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs13::_1
    }
}
#[doc = "Field `HRS13` writer - Hardware Request Status Channel 13"]
pub type Hrs13W<'a, REG> = crate::BitWriter<'a, REG, Hrs13>;
impl<'a, REG> Hrs13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs13::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs13::_1)
    }
}
#[doc = "Hardware Request Status Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs14 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs14> for bool {
    #[inline(always)]
    fn from(variant: Hrs14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS14` reader - Hardware Request Status Channel 14"]
pub type Hrs14R = crate::BitReader<Hrs14>;
impl Hrs14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs14 {
        match self.bits {
            false => Hrs14::_0,
            true => Hrs14::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs14::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs14::_1
    }
}
#[doc = "Field `HRS14` writer - Hardware Request Status Channel 14"]
pub type Hrs14W<'a, REG> = crate::BitWriter<'a, REG, Hrs14>;
impl<'a, REG> Hrs14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs14::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs14::_1)
    }
}
#[doc = "Hardware Request Status Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs15 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs15> for bool {
    #[inline(always)]
    fn from(variant: Hrs15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS15` reader - Hardware Request Status Channel 15"]
pub type Hrs15R = crate::BitReader<Hrs15>;
impl Hrs15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs15 {
        match self.bits {
            false => Hrs15::_0,
            true => Hrs15::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs15::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs15::_1
    }
}
#[doc = "Field `HRS15` writer - Hardware Request Status Channel 15"]
pub type Hrs15W<'a, REG> = crate::BitWriter<'a, REG, Hrs15>;
impl<'a, REG> Hrs15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs15::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs15::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&self) -> Hrs0R {
        Hrs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&self) -> Hrs1R {
        Hrs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&self) -> Hrs2R {
        Hrs2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&self) -> Hrs3R {
        Hrs3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    pub fn hrs4(&self) -> Hrs4R {
        Hrs4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    pub fn hrs5(&self) -> Hrs5R {
        Hrs5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    pub fn hrs6(&self) -> Hrs6R {
        Hrs6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    pub fn hrs7(&self) -> Hrs7R {
        Hrs7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    pub fn hrs8(&self) -> Hrs8R {
        Hrs8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    pub fn hrs9(&self) -> Hrs9R {
        Hrs9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    pub fn hrs10(&self) -> Hrs10R {
        Hrs10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    pub fn hrs11(&self) -> Hrs11R {
        Hrs11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    pub fn hrs12(&self) -> Hrs12R {
        Hrs12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    pub fn hrs13(&self) -> Hrs13R {
        Hrs13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    pub fn hrs14(&self) -> Hrs14R {
        Hrs14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    pub fn hrs15(&self) -> Hrs15R {
        Hrs15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn hrs0(&mut self) -> Hrs0W<HrsSpec> {
        Hrs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn hrs1(&mut self) -> Hrs1W<HrsSpec> {
        Hrs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn hrs2(&mut self) -> Hrs2W<HrsSpec> {
        Hrs2W::new(self, 2)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn hrs3(&mut self) -> Hrs3W<HrsSpec> {
        Hrs3W::new(self, 3)
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn hrs4(&mut self) -> Hrs4W<HrsSpec> {
        Hrs4W::new(self, 4)
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn hrs5(&mut self) -> Hrs5W<HrsSpec> {
        Hrs5W::new(self, 5)
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn hrs6(&mut self) -> Hrs6W<HrsSpec> {
        Hrs6W::new(self, 6)
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn hrs7(&mut self) -> Hrs7W<HrsSpec> {
        Hrs7W::new(self, 7)
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn hrs8(&mut self) -> Hrs8W<HrsSpec> {
        Hrs8W::new(self, 8)
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn hrs9(&mut self) -> Hrs9W<HrsSpec> {
        Hrs9W::new(self, 9)
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn hrs10(&mut self) -> Hrs10W<HrsSpec> {
        Hrs10W::new(self, 10)
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn hrs11(&mut self) -> Hrs11W<HrsSpec> {
        Hrs11W::new(self, 11)
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    #[must_use]
    pub fn hrs12(&mut self) -> Hrs12W<HrsSpec> {
        Hrs12W::new(self, 12)
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    #[must_use]
    pub fn hrs13(&mut self) -> Hrs13W<HrsSpec> {
        Hrs13W::new(self, 13)
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    #[must_use]
    pub fn hrs14(&mut self) -> Hrs14W<HrsSpec> {
        Hrs14W::new(self, 14)
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    #[must_use]
    pub fn hrs15(&mut self) -> Hrs15W<HrsSpec> {
        Hrs15W::new(self, 15)
    }
}
#[doc = "Hardware Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrsSpec;
impl crate::RegisterSpec for HrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrs::R`](R) reader structure"]
impl crate::Readable for HrsSpec {}
#[doc = "`write(|w| ..)` method takes [`hrs::W`](W) writer structure"]
impl crate::Writable for HrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRS to value 0"]
impl crate::Resettable for HrsSpec {
    const RESET_VALUE: u32 = 0;
}
