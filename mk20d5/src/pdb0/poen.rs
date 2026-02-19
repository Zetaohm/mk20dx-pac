#[doc = "Register `POEN` reader"]
pub type R = crate::R<PoenSpec>;
#[doc = "Register `POEN` writer"]
pub type W = crate::W<PoenSpec>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Poen {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<Poen> for u8 {
    #[inline(always)]
    fn from(variant: Poen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Poen {
    type Ux = u8;
}
impl crate::IsEnum for Poen {}
#[doc = "Field `POEN` reader - PDB Pulse-Out Enable"]
pub type PoenR = crate::FieldReader<Poen>;
impl PoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Poen> {
        match self.bits {
            0 => Some(Poen::_0),
            1 => Some(Poen::_1),
            _ => None,
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Poen::_0
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Poen::_1
    }
}
#[doc = "Field `POEN` writer - PDB Pulse-Out Enable"]
pub type PoenW<'a, REG> = crate::FieldWriter<'a, REG, 8, Poen>;
impl<'a, REG> PoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Poen::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Poen::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen(&self) -> PoenR {
        PoenR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen(&mut self) -> PoenW<PoenSpec> {
        PoenW::new(self, 0)
    }
}
#[doc = "Pulse-Out n Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`poen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PoenSpec;
impl crate::RegisterSpec for PoenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poen::R`](R) reader structure"]
impl crate::Readable for PoenSpec {}
#[doc = "`write(|w| ..)` method takes [`poen::W`](W) writer structure"]
impl crate::Writable for PoenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POEN to value 0"]
impl crate::Resettable for PoenSpec {
    const RESET_VALUE: u32 = 0;
}
