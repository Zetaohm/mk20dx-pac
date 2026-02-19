#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "Interrupt Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int0 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int0> for bool {
    #[inline(always)]
    fn from(variant: Int0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT0` reader - Interrupt Request 0"]
pub type Int0R = crate::BitReader<Int0>;
impl Int0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int0 {
        match self.bits {
            false => Int0::_0,
            true => Int0::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int0::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int0::_1
    }
}
#[doc = "Field `INT0` writer - Interrupt Request 0"]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG, Int0>;
impl<'a, REG> Int0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int0::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int0::_1)
    }
}
#[doc = "Interrupt Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int1 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int1> for bool {
    #[inline(always)]
    fn from(variant: Int1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT1` reader - Interrupt Request 1"]
pub type Int1R = crate::BitReader<Int1>;
impl Int1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int1 {
        match self.bits {
            false => Int1::_0,
            true => Int1::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int1::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int1::_1
    }
}
#[doc = "Field `INT1` writer - Interrupt Request 1"]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG, Int1>;
impl<'a, REG> Int1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int1::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int1::_1)
    }
}
#[doc = "Interrupt Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int2 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int2> for bool {
    #[inline(always)]
    fn from(variant: Int2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT2` reader - Interrupt Request 2"]
pub type Int2R = crate::BitReader<Int2>;
impl Int2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int2 {
        match self.bits {
            false => Int2::_0,
            true => Int2::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int2::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int2::_1
    }
}
#[doc = "Field `INT2` writer - Interrupt Request 2"]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG, Int2>;
impl<'a, REG> Int2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int2::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int2::_1)
    }
}
#[doc = "Interrupt Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int3 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int3> for bool {
    #[inline(always)]
    fn from(variant: Int3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT3` reader - Interrupt Request 3"]
pub type Int3R = crate::BitReader<Int3>;
impl Int3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int3 {
        match self.bits {
            false => Int3::_0,
            true => Int3::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int3::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int3::_1
    }
}
#[doc = "Field `INT3` writer - Interrupt Request 3"]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG, Int3>;
impl<'a, REG> Int3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int3::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int3::_1)
    }
}
#[doc = "Interrupt Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int4 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int4> for bool {
    #[inline(always)]
    fn from(variant: Int4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT4` reader - Interrupt Request 4"]
pub type Int4R = crate::BitReader<Int4>;
impl Int4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int4 {
        match self.bits {
            false => Int4::_0,
            true => Int4::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int4::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int4::_1
    }
}
#[doc = "Field `INT4` writer - Interrupt Request 4"]
pub type Int4W<'a, REG> = crate::BitWriter<'a, REG, Int4>;
impl<'a, REG> Int4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int4::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int4::_1)
    }
}
#[doc = "Interrupt Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int5 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int5> for bool {
    #[inline(always)]
    fn from(variant: Int5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT5` reader - Interrupt Request 5"]
pub type Int5R = crate::BitReader<Int5>;
impl Int5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int5 {
        match self.bits {
            false => Int5::_0,
            true => Int5::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int5::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int5::_1
    }
}
#[doc = "Field `INT5` writer - Interrupt Request 5"]
pub type Int5W<'a, REG> = crate::BitWriter<'a, REG, Int5>;
impl<'a, REG> Int5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int5::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int5::_1)
    }
}
#[doc = "Interrupt Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int6 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int6> for bool {
    #[inline(always)]
    fn from(variant: Int6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT6` reader - Interrupt Request 6"]
pub type Int6R = crate::BitReader<Int6>;
impl Int6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int6 {
        match self.bits {
            false => Int6::_0,
            true => Int6::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int6::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int6::_1
    }
}
#[doc = "Field `INT6` writer - Interrupt Request 6"]
pub type Int6W<'a, REG> = crate::BitWriter<'a, REG, Int6>;
impl<'a, REG> Int6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int6::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int6::_1)
    }
}
#[doc = "Interrupt Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int7 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int7> for bool {
    #[inline(always)]
    fn from(variant: Int7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT7` reader - Interrupt Request 7"]
pub type Int7R = crate::BitReader<Int7>;
impl Int7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int7 {
        match self.bits {
            false => Int7::_0,
            true => Int7::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int7::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int7::_1
    }
}
#[doc = "Field `INT7` writer - Interrupt Request 7"]
pub type Int7W<'a, REG> = crate::BitWriter<'a, REG, Int7>;
impl<'a, REG> Int7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int7::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int7::_1)
    }
}
#[doc = "Interrupt Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int8 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int8> for bool {
    #[inline(always)]
    fn from(variant: Int8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT8` reader - Interrupt Request 8"]
pub type Int8R = crate::BitReader<Int8>;
impl Int8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int8 {
        match self.bits {
            false => Int8::_0,
            true => Int8::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int8::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int8::_1
    }
}
#[doc = "Field `INT8` writer - Interrupt Request 8"]
pub type Int8W<'a, REG> = crate::BitWriter<'a, REG, Int8>;
impl<'a, REG> Int8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int8::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int8::_1)
    }
}
#[doc = "Interrupt Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int9 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int9> for bool {
    #[inline(always)]
    fn from(variant: Int9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT9` reader - Interrupt Request 9"]
pub type Int9R = crate::BitReader<Int9>;
impl Int9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int9 {
        match self.bits {
            false => Int9::_0,
            true => Int9::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int9::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int9::_1
    }
}
#[doc = "Field `INT9` writer - Interrupt Request 9"]
pub type Int9W<'a, REG> = crate::BitWriter<'a, REG, Int9>;
impl<'a, REG> Int9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int9::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int9::_1)
    }
}
#[doc = "Interrupt Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int10 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int10> for bool {
    #[inline(always)]
    fn from(variant: Int10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT10` reader - Interrupt Request 10"]
pub type Int10R = crate::BitReader<Int10>;
impl Int10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int10 {
        match self.bits {
            false => Int10::_0,
            true => Int10::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int10::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int10::_1
    }
}
#[doc = "Field `INT10` writer - Interrupt Request 10"]
pub type Int10W<'a, REG> = crate::BitWriter<'a, REG, Int10>;
impl<'a, REG> Int10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int10::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int10::_1)
    }
}
#[doc = "Interrupt Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int11 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int11> for bool {
    #[inline(always)]
    fn from(variant: Int11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT11` reader - Interrupt Request 11"]
pub type Int11R = crate::BitReader<Int11>;
impl Int11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int11 {
        match self.bits {
            false => Int11::_0,
            true => Int11::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int11::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int11::_1
    }
}
#[doc = "Field `INT11` writer - Interrupt Request 11"]
pub type Int11W<'a, REG> = crate::BitWriter<'a, REG, Int11>;
impl<'a, REG> Int11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int11::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int11::_1)
    }
}
#[doc = "Interrupt Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int12 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int12> for bool {
    #[inline(always)]
    fn from(variant: Int12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT12` reader - Interrupt Request 12"]
pub type Int12R = crate::BitReader<Int12>;
impl Int12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int12 {
        match self.bits {
            false => Int12::_0,
            true => Int12::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int12::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int12::_1
    }
}
#[doc = "Field `INT12` writer - Interrupt Request 12"]
pub type Int12W<'a, REG> = crate::BitWriter<'a, REG, Int12>;
impl<'a, REG> Int12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int12::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int12::_1)
    }
}
#[doc = "Interrupt Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int13 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int13> for bool {
    #[inline(always)]
    fn from(variant: Int13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT13` reader - Interrupt Request 13"]
pub type Int13R = crate::BitReader<Int13>;
impl Int13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int13 {
        match self.bits {
            false => Int13::_0,
            true => Int13::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int13::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int13::_1
    }
}
#[doc = "Field `INT13` writer - Interrupt Request 13"]
pub type Int13W<'a, REG> = crate::BitWriter<'a, REG, Int13>;
impl<'a, REG> Int13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int13::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int13::_1)
    }
}
#[doc = "Interrupt Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int14 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int14> for bool {
    #[inline(always)]
    fn from(variant: Int14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT14` reader - Interrupt Request 14"]
pub type Int14R = crate::BitReader<Int14>;
impl Int14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int14 {
        match self.bits {
            false => Int14::_0,
            true => Int14::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int14::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int14::_1
    }
}
#[doc = "Field `INT14` writer - Interrupt Request 14"]
pub type Int14W<'a, REG> = crate::BitWriter<'a, REG, Int14>;
impl<'a, REG> Int14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int14::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int14::_1)
    }
}
#[doc = "Interrupt Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int15 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int15> for bool {
    #[inline(always)]
    fn from(variant: Int15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT15` reader - Interrupt Request 15"]
pub type Int15R = crate::BitReader<Int15>;
impl Int15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int15 {
        match self.bits {
            false => Int15::_0,
            true => Int15::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int15::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int15::_1
    }
}
#[doc = "Field `INT15` writer - Interrupt Request 15"]
pub type Int15W<'a, REG> = crate::BitWriter<'a, REG, Int15>;
impl<'a, REG> Int15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int15::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int15::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    pub fn int4(&self) -> Int4R {
        Int4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    pub fn int6(&self) -> Int6R {
        Int6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    pub fn int7(&self) -> Int7R {
        Int7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    pub fn int8(&self) -> Int8R {
        Int8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    pub fn int9(&self) -> Int9R {
        Int9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    pub fn int10(&self) -> Int10R {
        Int10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    pub fn int11(&self) -> Int11R {
        Int11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    pub fn int12(&self) -> Int12R {
        Int12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    pub fn int14(&self) -> Int14R {
        Int14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    pub fn int15(&self) -> Int15R {
        Int15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<IntSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<IntSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<IntSpec> {
        Int2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<IntSpec> {
        Int3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> Int4W<IntSpec> {
        Int4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> Int5W<IntSpec> {
        Int5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> Int6W<IntSpec> {
        Int6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> Int7W<IntSpec> {
        Int7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> Int8W<IntSpec> {
        Int8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> Int9W<IntSpec> {
        Int9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> Int10W<IntSpec> {
        Int10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> Int11W<IntSpec> {
        Int11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> Int12W<IntSpec> {
        Int12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> Int13W<IntSpec> {
        Int13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> Int14W<IntSpec> {
        Int14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> Int15W<IntSpec> {
        Int15W::new(self, 15)
    }
}
#[doc = "Interrupt Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {
    const RESET_VALUE: u32 = 0;
}
