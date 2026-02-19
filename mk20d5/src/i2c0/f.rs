#[doc = "Register `F` reader"]
pub type R = crate::R<FSpec>;
#[doc = "Register `F` writer"]
pub type W = crate::W<FSpec>;
#[doc = "Field `ICR` reader - Clock rate"]
pub type IcrR = crate::FieldReader;
#[doc = "Field `ICR` writer - Clock rate"]
pub type IcrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "The MULT bits define the multiplier factor mul\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mult {
    #[doc = "0: mul = 1"]
    _00 = 0,
    #[doc = "1: mul = 2"]
    _01 = 1,
    #[doc = "2: mul = 4"]
    _10 = 2,
}
impl From<Mult> for u8 {
    #[inline(always)]
    fn from(variant: Mult) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mult {
    type Ux = u8;
}
impl crate::IsEnum for Mult {}
#[doc = "Field `MULT` reader - The MULT bits define the multiplier factor mul"]
pub type MultR = crate::FieldReader<Mult>;
impl MultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mult> {
        match self.bits {
            0 => Some(Mult::_00),
            1 => Some(Mult::_01),
            2 => Some(Mult::_10),
            _ => None,
        }
    }
    #[doc = "mul = 1"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Mult::_00
    }
    #[doc = "mul = 2"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Mult::_01
    }
    #[doc = "mul = 4"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Mult::_10
    }
}
#[doc = "Field `MULT` writer - The MULT bits define the multiplier factor mul"]
pub type MultW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mult>;
impl<'a, REG> MultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "mul = 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::_00)
    }
    #[doc = "mul = 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::_01)
    }
    #[doc = "mul = 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::_10)
    }
}
impl R {
    #[doc = "Bits 0:5 - Clock rate"]
    #[inline(always)]
    pub fn icr(&self) -> IcrR {
        IcrR::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - The MULT bits define the multiplier factor mul"]
    #[inline(always)]
    pub fn mult(&self) -> MultR {
        MultR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clock rate"]
    #[inline(always)]
    #[must_use]
    pub fn icr(&mut self) -> IcrW<FSpec> {
        IcrW::new(self, 0)
    }
    #[doc = "Bits 6:7 - The MULT bits define the multiplier factor mul"]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MultW<FSpec> {
        MultW::new(self, 6)
    }
}
#[doc = "I2C Frequency Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`f::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSpec;
impl crate::RegisterSpec for FSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`f::R`](R) reader structure"]
impl crate::Readable for FSpec {}
#[doc = "`write(|w| ..)` method takes [`f::W`](W) writer structure"]
impl crate::Writable for FSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets F to value 0"]
impl crate::Resettable for FSpec {
    const RESET_VALUE: u8 = 0;
}
