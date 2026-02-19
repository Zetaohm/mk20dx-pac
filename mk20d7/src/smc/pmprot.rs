#[doc = "Register `PMPROT` reader"]
pub type R = crate::R<PmprotSpec>;
#[doc = "Register `PMPROT` writer"]
pub type W = crate::W<PmprotSpec>;
#[doc = "Allow very low leakage stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avlls {
    #[doc = "0: Any VLLSx mode is not allowed"]
    _0 = 0,
    #[doc = "1: Any VLLSx mode is allowed"]
    _1 = 1,
}
impl From<Avlls> for bool {
    #[inline(always)]
    fn from(variant: Avlls) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLLS` reader - Allow very low leakage stop mode"]
pub type AvllsR = crate::BitReader<Avlls>;
impl AvllsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avlls {
        match self.bits {
            false => Avlls::_0,
            true => Avlls::_1,
        }
    }
    #[doc = "Any VLLSx mode is not allowed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Avlls::_0
    }
    #[doc = "Any VLLSx mode is allowed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Avlls::_1
    }
}
#[doc = "Field `AVLLS` writer - Allow very low leakage stop mode"]
pub type AvllsW<'a, REG> = crate::BitWriter<'a, REG, Avlls>;
impl<'a, REG> AvllsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any VLLSx mode is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Avlls::_0)
    }
    #[doc = "Any VLLSx mode is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Avlls::_1)
    }
}
#[doc = "Allow low leakage stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alls {
    #[doc = "0: LLS is not allowed"]
    _0 = 0,
    #[doc = "1: LLS is allowed"]
    _1 = 1,
}
impl From<Alls> for bool {
    #[inline(always)]
    fn from(variant: Alls) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLS` reader - Allow low leakage stop mode"]
pub type AllsR = crate::BitReader<Alls>;
impl AllsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alls {
        match self.bits {
            false => Alls::_0,
            true => Alls::_1,
        }
    }
    #[doc = "LLS is not allowed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Alls::_0
    }
    #[doc = "LLS is allowed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Alls::_1
    }
}
#[doc = "Field `ALLS` writer - Allow low leakage stop mode"]
pub type AllsW<'a, REG> = crate::BitWriter<'a, REG, Alls>;
impl<'a, REG> AllsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLS is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Alls::_0)
    }
    #[doc = "LLS is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Alls::_1)
    }
}
#[doc = "Allow very low power modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avlp {
    #[doc = "0: VLPR, VLPW and VLPS are not allowed"]
    _0 = 0,
    #[doc = "1: VLPR, VLPW and VLPS are allowed"]
    _1 = 1,
}
impl From<Avlp> for bool {
    #[inline(always)]
    fn from(variant: Avlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLP` reader - Allow very low power modes"]
pub type AvlpR = crate::BitReader<Avlp>;
impl AvlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avlp {
        match self.bits {
            false => Avlp::_0,
            true => Avlp::_1,
        }
    }
    #[doc = "VLPR, VLPW and VLPS are not allowed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Avlp::_0
    }
    #[doc = "VLPR, VLPW and VLPS are allowed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Avlp::_1
    }
}
#[doc = "Field `AVLP` writer - Allow very low power modes"]
pub type AvlpW<'a, REG> = crate::BitWriter<'a, REG, Avlp>;
impl<'a, REG> AvlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VLPR, VLPW and VLPS are not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Avlp::_0)
    }
    #[doc = "VLPR, VLPW and VLPS are allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Avlp::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Allow very low leakage stop mode"]
    #[inline(always)]
    pub fn avlls(&self) -> AvllsR {
        AvllsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow low leakage stop mode"]
    #[inline(always)]
    pub fn alls(&self) -> AllsR {
        AllsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow very low power modes"]
    #[inline(always)]
    pub fn avlp(&self) -> AvlpR {
        AvlpR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allow very low leakage stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn avlls(&mut self) -> AvllsW<PmprotSpec> {
        AvllsW::new(self, 1)
    }
    #[doc = "Bit 3 - Allow low leakage stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn alls(&mut self) -> AllsW<PmprotSpec> {
        AllsW::new(self, 3)
    }
    #[doc = "Bit 5 - Allow very low power modes"]
    #[inline(always)]
    #[must_use]
    pub fn avlp(&mut self) -> AvlpW<PmprotSpec> {
        AvlpW::new(self, 5)
    }
}
#[doc = "Power Mode Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmprotSpec;
impl crate::RegisterSpec for PmprotSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmprot::R`](R) reader structure"]
impl crate::Readable for PmprotSpec {}
#[doc = "`write(|w| ..)` method takes [`pmprot::W`](W) writer structure"]
impl crate::Writable for PmprotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PMPROT to value 0"]
impl crate::Resettable for PmprotSpec {
    const RESET_VALUE: u8 = 0;
}
