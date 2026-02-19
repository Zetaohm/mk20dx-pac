#[doc = "Register `RMR` reader"]
pub type R = crate::R<RmrSpec>;
#[doc = "Register `RMR` writer"]
pub type W = crate::W<RmrSpec>;
#[doc = "Receive word mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Rwm {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<Rwm> for u32 {
    #[inline(always)]
    fn from(variant: Rwm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rwm {
    type Ux = u32;
}
impl crate::IsEnum for Rwm {}
#[doc = "Field `RWM` reader - Receive word mask"]
pub type RwmR = crate::FieldReader<Rwm>;
impl RwmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rwm> {
        match self.bits {
            0 => Some(Rwm::_0),
            1 => Some(Rwm::_1),
            _ => None,
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rwm::_0
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rwm::_1
    }
}
#[doc = "Field `RWM` writer - Receive word mask"]
pub type RwmW<'a, REG> = crate::FieldWriter<'a, REG, 32, Rwm>;
impl<'a, REG> RwmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwm::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwm::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive word mask"]
    #[inline(always)]
    pub fn rwm(&self) -> RwmR {
        RwmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive word mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm(&mut self) -> RwmW<RmrSpec> {
        RwmW::new(self, 0)
    }
}
#[doc = "SAI Receive Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmrSpec;
impl crate::RegisterSpec for RmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmr::R`](R) reader structure"]
impl crate::Readable for RmrSpec {}
#[doc = "`write(|w| ..)` method takes [`rmr::W`](W) writer structure"]
impl crate::Writable for RmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMR to value 0"]
impl crate::Resettable for RmrSpec {
    const RESET_VALUE: u32 = 0;
}
