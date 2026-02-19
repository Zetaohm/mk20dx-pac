#[doc = "Register `RX14MASK` reader"]
pub type R = crate::R<Rx14maskSpec>;
#[doc = "Register `RX14MASK` writer"]
pub type W = crate::W<Rx14maskSpec>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Rx14m {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<Rx14m> for u32 {
    #[inline(always)]
    fn from(variant: Rx14m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rx14m {
    type Ux = u32;
}
impl crate::IsEnum for Rx14m {}
#[doc = "Field `RX14M` reader - Rx Buffer 14 Mask Bits"]
pub type Rx14mR = crate::FieldReader<Rx14m>;
impl Rx14mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rx14m> {
        match self.bits {
            0 => Some(Rx14m::_0),
            1 => Some(Rx14m::_1),
            _ => None,
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rx14m::_0
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rx14m::_1
    }
}
#[doc = "Field `RX14M` writer - Rx Buffer 14 Mask Bits"]
pub type Rx14mW<'a, REG> = crate::FieldWriter<'a, REG, 32, Rx14m>;
impl<'a, REG> Rx14mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rx14m::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rx14m::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m(&self) -> Rx14mR {
        Rx14mR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m(&mut self) -> Rx14mW<Rx14maskSpec> {
        Rx14mW::new(self, 0)
    }
}
#[doc = "Rx 14 Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx14mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx14mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx14maskSpec;
impl crate::RegisterSpec for Rx14maskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx14mask::R`](R) reader structure"]
impl crate::Readable for Rx14maskSpec {}
#[doc = "`write(|w| ..)` method takes [`rx14mask::W`](W) writer structure"]
impl crate::Writable for Rx14maskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX14MASK to value 0xffff_ffff"]
impl crate::Resettable for Rx14maskSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
