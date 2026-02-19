#[doc = "Register `NBYTES_MLOFFNO` reader"]
pub type R = crate::R<NbytesMloffnoSpec>;
#[doc = "Register `NBYTES_MLOFFNO` writer"]
pub type W = crate::W<NbytesMloffnoSpec>;
#[doc = "Field `NBYTES` reader - Minor Byte Transfer Count"]
pub type NbytesR = crate::FieldReader<u32>;
#[doc = "Field `NBYTES` writer - Minor Byte Transfer Count"]
pub type NbytesW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Destination Minor Loop Offset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmloe {
    #[doc = "0: The minor loop offset is not applied to the DADDR"]
    _0 = 0,
    #[doc = "1: The minor loop offset is applied to the DADDR"]
    _1 = 1,
}
impl From<Dmloe> for bool {
    #[inline(always)]
    fn from(variant: Dmloe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMLOE` reader - Destination Minor Loop Offset enable"]
pub type DmloeR = crate::BitReader<Dmloe>;
impl DmloeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmloe {
        match self.bits {
            false => Dmloe::_0,
            true => Dmloe::_1,
        }
    }
    #[doc = "The minor loop offset is not applied to the DADDR"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmloe::_0
    }
    #[doc = "The minor loop offset is applied to the DADDR"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmloe::_1
    }
}
#[doc = "Field `DMLOE` writer - Destination Minor Loop Offset enable"]
pub type DmloeW<'a, REG> = crate::BitWriter<'a, REG, Dmloe>;
impl<'a, REG> DmloeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The minor loop offset is not applied to the DADDR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmloe::_0)
    }
    #[doc = "The minor loop offset is applied to the DADDR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmloe::_1)
    }
}
#[doc = "Source Minor Loop Offset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smloe {
    #[doc = "0: The minor loop offset is not applied to the SADDR"]
    _0 = 0,
    #[doc = "1: The minor loop offset is applied to the SADDR"]
    _1 = 1,
}
impl From<Smloe> for bool {
    #[inline(always)]
    fn from(variant: Smloe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMLOE` reader - Source Minor Loop Offset Enable"]
pub type SmloeR = crate::BitReader<Smloe>;
impl SmloeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smloe {
        match self.bits {
            false => Smloe::_0,
            true => Smloe::_1,
        }
    }
    #[doc = "The minor loop offset is not applied to the SADDR"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Smloe::_0
    }
    #[doc = "The minor loop offset is applied to the SADDR"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Smloe::_1
    }
}
#[doc = "Field `SMLOE` writer - Source Minor Loop Offset Enable"]
pub type SmloeW<'a, REG> = crate::BitWriter<'a, REG, Smloe>;
impl<'a, REG> SmloeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The minor loop offset is not applied to the SADDR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Smloe::_0)
    }
    #[doc = "The minor loop offset is applied to the SADDR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Smloe::_1)
    }
}
impl R {
    #[doc = "Bits 0:29 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&self) -> NbytesR {
        NbytesR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline(always)]
    pub fn dmloe(&self) -> DmloeR {
        DmloeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub fn smloe(&self) -> SmloeR {
        SmloeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - Minor Byte Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NbytesW<NbytesMloffnoSpec> {
        NbytesW::new(self, 0)
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmloe(&mut self) -> DmloeW<NbytesMloffnoSpec> {
        DmloeW::new(self, 30)
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smloe(&mut self) -> SmloeW<NbytesMloffnoSpec> {
        SmloeW::new(self, 31)
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`nbytes_mloffno::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbytes_mloffno::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NbytesMloffnoSpec;
impl crate::RegisterSpec for NbytesMloffnoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbytes_mloffno::R`](R) reader structure"]
impl crate::Readable for NbytesMloffnoSpec {}
#[doc = "`write(|w| ..)` method takes [`nbytes_mloffno::W`](W) writer structure"]
impl crate::Writable for NbytesMloffnoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NBYTES_MLOFFNO to value 0"]
impl crate::Resettable for NbytesMloffnoSpec {
    const RESET_VALUE: u32 = 0;
}
