#[doc = "Register `TRM` reader"]
pub type R = crate::R<TrmSpec>;
#[doc = "Register `TRM` writer"]
pub type W = crate::W<TrmSpec>;
#[doc = "Trim bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trim {
    #[doc = "0: Min"]
    _000000 = 0,
    #[doc = "63: Max"]
    _111111 = 63,
}
impl From<Trim> for u8 {
    #[inline(always)]
    fn from(variant: Trim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trim {
    type Ux = u8;
}
impl crate::IsEnum for Trim {}
#[doc = "Field `TRIM` reader - Trim bits"]
pub type TrimR = crate::FieldReader<Trim>;
impl TrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trim> {
        match self.bits {
            0 => Some(Trim::_000000),
            63 => Some(Trim::_111111),
            _ => None,
        }
    }
    #[doc = "Min"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        *self == Trim::_000000
    }
    #[doc = "Max"]
    #[inline(always)]
    pub fn is_111111(&self) -> bool {
        *self == Trim::_111111
    }
}
#[doc = "Field `TRIM` writer - Trim bits"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 6, Trim>;
impl<'a, REG> TrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Min"]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::_000000)
    }
    #[doc = "Max"]
    #[inline(always)]
    pub fn _111111(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::_111111)
    }
}
#[doc = "Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chopen {
    #[doc = "0: Chop oscillator is disabled."]
    _0 = 0,
    #[doc = "1: Chop oscillator is enabled."]
    _1 = 1,
}
impl From<Chopen> for bool {
    #[inline(always)]
    fn from(variant: Chopen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHOPEN` reader - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
pub type ChopenR = crate::BitReader<Chopen>;
impl ChopenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chopen {
        match self.bits {
            false => Chopen::_0,
            true => Chopen::_1,
        }
    }
    #[doc = "Chop oscillator is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chopen::_0
    }
    #[doc = "Chop oscillator is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chopen::_1
    }
}
#[doc = "Field `CHOPEN` writer - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
pub type ChopenW<'a, REG> = crate::BitWriter<'a, REG, Chopen>;
impl<'a, REG> ChopenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Chop oscillator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Chopen::_0)
    }
    #[doc = "Chop oscillator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Chopen::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - Trim bits"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline(always)]
    pub fn chopen(&self) -> ChopenR {
        ChopenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trim bits"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<TrmSpec> {
        TrimW::new(self, 0)
    }
    #[doc = "Bit 6 - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline(always)]
    #[must_use]
    pub fn chopen(&mut self) -> ChopenW<TrmSpec> {
        ChopenW::new(self, 6)
    }
}
#[doc = "VREF Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrmSpec;
impl crate::RegisterSpec for TrmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trm::R`](R) reader structure"]
impl crate::Readable for TrmSpec {}
#[doc = "`write(|w| ..)` method takes [`trm::W`](W) writer structure"]
impl crate::Writable for TrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TRM to value 0"]
impl crate::Resettable for TrmSpec {
    const RESET_VALUE: u8 = 0;
}
