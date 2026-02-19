#[doc = "Register `DEADTIME` reader"]
pub type R = crate::R<DeadtimeSpec>;
#[doc = "Register `DEADTIME` writer"]
pub type W = crate::W<DeadtimeSpec>;
#[doc = "Field `DTVAL` reader - Deadtime Value"]
pub type DtvalR = crate::FieldReader;
#[doc = "Field `DTVAL` writer - Deadtime Value"]
pub type DtvalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Deadtime Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtps {
    #[doc = "0: Divide the system clock by 1."]
    _0x = 0,
    #[doc = "2: Divide the system clock by 4."]
    _10 = 2,
    #[doc = "3: Divide the system clock by 16."]
    _11 = 3,
}
impl From<Dtps> for u8 {
    #[inline(always)]
    fn from(variant: Dtps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtps {
    type Ux = u8;
}
impl crate::IsEnum for Dtps {}
#[doc = "Field `DTPS` reader - Deadtime Prescaler Value"]
pub type DtpsR = crate::FieldReader<Dtps>;
impl DtpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtps> {
        match self.bits {
            0 => Some(Dtps::_0x),
            2 => Some(Dtps::_10),
            3 => Some(Dtps::_11),
            _ => None,
        }
    }
    #[doc = "Divide the system clock by 1."]
    #[inline(always)]
    pub fn is_0x(&self) -> bool {
        *self == Dtps::_0x
    }
    #[doc = "Divide the system clock by 4."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dtps::_10
    }
    #[doc = "Divide the system clock by 16."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dtps::_11
    }
}
#[doc = "Field `DTPS` writer - Deadtime Prescaler Value"]
pub type DtpsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dtps>;
impl<'a, REG> DtpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide the system clock by 1."]
    #[inline(always)]
    pub fn _0x(self) -> &'a mut crate::W<REG> {
        self.variant(Dtps::_0x)
    }
    #[doc = "Divide the system clock by 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dtps::_10)
    }
    #[doc = "Divide the system clock by 16."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dtps::_11)
    }
}
impl R {
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline(always)]
    pub fn dtval(&self) -> DtvalR {
        DtvalR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline(always)]
    pub fn dtps(&self) -> DtpsR {
        DtpsR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline(always)]
    #[must_use]
    pub fn dtval(&mut self) -> DtvalW<DeadtimeSpec> {
        DtvalW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn dtps(&mut self) -> DtpsW<DeadtimeSpec> {
        DtpsW::new(self, 6)
    }
}
#[doc = "Deadtime Insertion Control\n\nYou can [`read`](crate::Reg::read) this register and get [`deadtime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deadtime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeadtimeSpec;
impl crate::RegisterSpec for DeadtimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deadtime::R`](R) reader structure"]
impl crate::Readable for DeadtimeSpec {}
#[doc = "`write(|w| ..)` method takes [`deadtime::W`](W) writer structure"]
impl crate::Writable for DeadtimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEADTIME to value 0"]
impl crate::Resettable for DeadtimeSpec {
    const RESET_VALUE: u32 = 0;
}
