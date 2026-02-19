#[doc = "Register `F3` reader"]
pub type R = crate::R<F3Spec>;
#[doc = "Wakeup flag for module 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwuf0 {
    #[doc = "0: Module 0 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 0 input was a wakeup source"]
    _1 = 1,
}
impl From<Mwuf0> for bool {
    #[inline(always)]
    fn from(variant: Mwuf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF0` reader - Wakeup flag for module 0"]
pub type Mwuf0R = crate::BitReader<Mwuf0>;
impl Mwuf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwuf0 {
        match self.bits {
            false => Mwuf0::_0,
            true => Mwuf0::_1,
        }
    }
    #[doc = "Module 0 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwuf0::_0
    }
    #[doc = "Module 0 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwuf0::_1
    }
}
#[doc = "Wakeup flag for module 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwuf1 {
    #[doc = "0: Module 1 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 1 input was a wakeup source"]
    _1 = 1,
}
impl From<Mwuf1> for bool {
    #[inline(always)]
    fn from(variant: Mwuf1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF1` reader - Wakeup flag for module 1"]
pub type Mwuf1R = crate::BitReader<Mwuf1>;
impl Mwuf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwuf1 {
        match self.bits {
            false => Mwuf1::_0,
            true => Mwuf1::_1,
        }
    }
    #[doc = "Module 1 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwuf1::_0
    }
    #[doc = "Module 1 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwuf1::_1
    }
}
#[doc = "Wakeup flag for module 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwuf2 {
    #[doc = "0: Module 2 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 2 input was a wakeup source"]
    _1 = 1,
}
impl From<Mwuf2> for bool {
    #[inline(always)]
    fn from(variant: Mwuf2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF2` reader - Wakeup flag for module 2"]
pub type Mwuf2R = crate::BitReader<Mwuf2>;
impl Mwuf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwuf2 {
        match self.bits {
            false => Mwuf2::_0,
            true => Mwuf2::_1,
        }
    }
    #[doc = "Module 2 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwuf2::_0
    }
    #[doc = "Module 2 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwuf2::_1
    }
}
#[doc = "Wakeup flag for module 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwuf3 {
    #[doc = "0: Module 3 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 3 input was a wakeup source"]
    _1 = 1,
}
impl From<Mwuf3> for bool {
    #[inline(always)]
    fn from(variant: Mwuf3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF3` reader - Wakeup flag for module 3"]
pub type Mwuf3R = crate::BitReader<Mwuf3>;
impl Mwuf3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwuf3 {
        match self.bits {
            false => Mwuf3::_0,
            true => Mwuf3::_1,
        }
    }
    #[doc = "Module 3 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwuf3::_0
    }
    #[doc = "Module 3 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwuf3::_1
    }
}
#[doc = "Wakeup flag for module 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwuf4 {
    #[doc = "0: Module 4 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 4 input was a wakeup source"]
    _1 = 1,
}
impl From<Mwuf4> for bool {
    #[inline(always)]
    fn from(variant: Mwuf4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF4` reader - Wakeup flag for module 4"]
pub type Mwuf4R = crate::BitReader<Mwuf4>;
impl Mwuf4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwuf4 {
        match self.bits {
            false => Mwuf4::_0,
            true => Mwuf4::_1,
        }
    }
    #[doc = "Module 4 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwuf4::_0
    }
    #[doc = "Module 4 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwuf4::_1
    }
}
#[doc = "Wakeup flag for module 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwuf5 {
    #[doc = "0: Module 5 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 5 input was a wakeup source"]
    _1 = 1,
}
impl From<Mwuf5> for bool {
    #[inline(always)]
    fn from(variant: Mwuf5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF5` reader - Wakeup flag for module 5"]
pub type Mwuf5R = crate::BitReader<Mwuf5>;
impl Mwuf5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwuf5 {
        match self.bits {
            false => Mwuf5::_0,
            true => Mwuf5::_1,
        }
    }
    #[doc = "Module 5 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwuf5::_0
    }
    #[doc = "Module 5 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwuf5::_1
    }
}
#[doc = "Wakeup flag for module 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwuf6 {
    #[doc = "0: Module 6 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 6 input was a wakeup source"]
    _1 = 1,
}
impl From<Mwuf6> for bool {
    #[inline(always)]
    fn from(variant: Mwuf6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF6` reader - Wakeup flag for module 6"]
pub type Mwuf6R = crate::BitReader<Mwuf6>;
impl Mwuf6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwuf6 {
        match self.bits {
            false => Mwuf6::_0,
            true => Mwuf6::_1,
        }
    }
    #[doc = "Module 6 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwuf6::_0
    }
    #[doc = "Module 6 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwuf6::_1
    }
}
#[doc = "Wakeup flag for module 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwuf7 {
    #[doc = "0: Module 7 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 7 input was a wakeup source"]
    _1 = 1,
}
impl From<Mwuf7> for bool {
    #[inline(always)]
    fn from(variant: Mwuf7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF7` reader - Wakeup flag for module 7"]
pub type Mwuf7R = crate::BitReader<Mwuf7>;
impl Mwuf7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwuf7 {
        match self.bits {
            false => Mwuf7::_0,
            true => Mwuf7::_1,
        }
    }
    #[doc = "Module 7 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwuf7::_0
    }
    #[doc = "Module 7 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwuf7::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag for module 0"]
    #[inline(always)]
    pub fn mwuf0(&self) -> Mwuf0R {
        Mwuf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag for module 1"]
    #[inline(always)]
    pub fn mwuf1(&self) -> Mwuf1R {
        Mwuf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag for module 2"]
    #[inline(always)]
    pub fn mwuf2(&self) -> Mwuf2R {
        Mwuf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag for module 3"]
    #[inline(always)]
    pub fn mwuf3(&self) -> Mwuf3R {
        Mwuf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag for module 4"]
    #[inline(always)]
    pub fn mwuf4(&self) -> Mwuf4R {
        Mwuf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup flag for module 5"]
    #[inline(always)]
    pub fn mwuf5(&self) -> Mwuf5R {
        Mwuf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup flag for module 6"]
    #[inline(always)]
    pub fn mwuf6(&self) -> Mwuf6R {
        Mwuf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup flag for module 7"]
    #[inline(always)]
    pub fn mwuf7(&self) -> Mwuf7R {
        Mwuf7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "LLWU Flag 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`f3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F3Spec;
impl crate::RegisterSpec for F3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`f3::R`](R) reader structure"]
impl crate::Readable for F3Spec {}
#[doc = "`reset()` method sets F3 to value 0"]
impl crate::Resettable for F3Spec {
    const RESET_VALUE: u8 = 0;
}
