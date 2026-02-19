#[doc = "Register `S4` reader"]
pub type R = crate::R<S4Spec>;
#[doc = "Register `S4` writer"]
pub type W = crate::W<S4Spec>;
#[doc = "Framing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: Received packet is byte bound."]
    _0 = 0,
    #[doc = "1: Received packet is not byte bound."]
    _1 = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error"]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::_0,
            true => Fe::_1,
        }
    }
    #[doc = "Received packet is byte bound."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fe::_0
    }
    #[doc = "Received packet is not byte bound."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fe::_1
    }
}
#[doc = "Field `FE` writer - Framing Error"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG, Fe>;
impl<'a, REG> FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Received packet is byte bound."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::_0)
    }
    #[doc = "Received packet is not byte bound."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::_1)
    }
}
#[doc = "Improper Line Code Violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilcv {
    #[doc = "0: Line code violation received is proper."]
    _0 = 0,
    #[doc = "1: Line code violation received is improper i.e less than 3-bit periods."]
    _1 = 1,
}
impl From<Ilcv> for bool {
    #[inline(always)]
    fn from(variant: Ilcv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILCV` reader - Improper Line Code Violation"]
pub type IlcvR = crate::BitReader<Ilcv>;
impl IlcvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilcv {
        match self.bits {
            false => Ilcv::_0,
            true => Ilcv::_1,
        }
    }
    #[doc = "Line code violation received is proper."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilcv::_0
    }
    #[doc = "Line code violation received is improper i.e less than 3-bit periods."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilcv::_1
    }
}
#[doc = "Field `ILCV` writer - Improper Line Code Violation"]
pub type IlcvW<'a, REG> = crate::BitWriter<'a, REG, Ilcv>;
impl<'a, REG> IlcvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Line code violation received is proper."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilcv::_0)
    }
    #[doc = "Line code violation received is improper i.e less than 3-bit periods."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilcv::_1)
    }
}
#[doc = "CDET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdet {
    #[doc = "0: No collision."]
    _00 = 0,
    #[doc = "1: Collision occurred during preamble."]
    _01 = 1,
    #[doc = "2: Collision occurred during data."]
    _10 = 2,
    #[doc = "3: Collision occurred during line code violation."]
    _11 = 3,
}
impl From<Cdet> for u8 {
    #[inline(always)]
    fn from(variant: Cdet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdet {
    type Ux = u8;
}
impl crate::IsEnum for Cdet {}
#[doc = "Field `CDET` reader - CDET"]
pub type CdetR = crate::FieldReader<Cdet>;
impl CdetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdet {
        match self.bits {
            0 => Cdet::_00,
            1 => Cdet::_01,
            2 => Cdet::_10,
            3 => Cdet::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No collision."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cdet::_00
    }
    #[doc = "Collision occurred during preamble."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cdet::_01
    }
    #[doc = "Collision occurred during data."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cdet::_10
    }
    #[doc = "Collision occurred during line code violation."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cdet::_11
    }
}
#[doc = "Field `CDET` writer - CDET"]
pub type CdetW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cdet, crate::Safe>;
impl<'a, REG> CdetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No collision."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cdet::_00)
    }
    #[doc = "Collision occurred during preamble."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cdet::_01)
    }
    #[doc = "Collision occurred during data."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cdet::_10)
    }
    #[doc = "Collision occurred during line code violation."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cdet::_11)
    }
}
#[doc = "Initial Synchronization Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initf {
    #[doc = "0: Initial synchronization is not failed."]
    _0 = 0,
    #[doc = "1: Initial synchronization is failed."]
    _1 = 1,
}
impl From<Initf> for bool {
    #[inline(always)]
    fn from(variant: Initf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initial Synchronization Fail Flag"]
pub type InitfR = crate::BitReader<Initf>;
impl InitfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initf {
        match self.bits {
            false => Initf::_0,
            true => Initf::_1,
        }
    }
    #[doc = "Initial synchronization is not failed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Initf::_0
    }
    #[doc = "Initial synchronization is failed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Initf::_1
    }
}
impl R {
    #[doc = "Bit 0 - Framing Error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Improper Line Code Violation"]
    #[inline(always)]
    pub fn ilcv(&self) -> IlcvR {
        IlcvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - CDET"]
    #[inline(always)]
    pub fn cdet(&self) -> CdetR {
        CdetR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Initial Synchronization Fail Flag"]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Framing Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<S4Spec> {
        FeW::new(self, 0)
    }
    #[doc = "Bit 1 - Improper Line Code Violation"]
    #[inline(always)]
    #[must_use]
    pub fn ilcv(&mut self) -> IlcvW<S4Spec> {
        IlcvW::new(self, 1)
    }
    #[doc = "Bits 2:3 - CDET"]
    #[inline(always)]
    #[must_use]
    pub fn cdet(&mut self) -> CdetW<S4Spec> {
        CdetW::new(self, 2)
    }
}
#[doc = "UART CEA709.1-B Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S4Spec;
impl crate::RegisterSpec for S4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s4::R`](R) reader structure"]
impl crate::Readable for S4Spec {}
#[doc = "`write(|w| ..)` method takes [`s4::W`](W) writer structure"]
impl crate::Writable for S4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets S4 to value 0"]
impl crate::Resettable for S4Spec {
    const RESET_VALUE: u8 = 0;
}
