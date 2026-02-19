#[doc = "Register `DCHPRI%s` reader"]
pub type R = crate::R<DchpriSpec>;
#[doc = "Register `DCHPRI%s` writer"]
pub type W = crate::W<DchpriSpec>;
#[doc = "Field `CHPRI` reader - Channel n Arbitration Priority"]
pub type ChpriR = crate::FieldReader;
#[doc = "Field `CHPRI` writer - Channel n Arbitration Priority"]
pub type ChpriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Disable Preempt Ability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpa {
    #[doc = "0: Channel n can suspend a lower priority channel"]
    _0 = 0,
    #[doc = "1: Channel n cannot suspend any channel, regardless of channel priority"]
    _1 = 1,
}
impl From<Dpa> for bool {
    #[inline(always)]
    fn from(variant: Dpa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPA` reader - Disable Preempt Ability"]
pub type DpaR = crate::BitReader<Dpa>;
impl DpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpa {
        match self.bits {
            false => Dpa::_0,
            true => Dpa::_1,
        }
    }
    #[doc = "Channel n can suspend a lower priority channel"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpa::_0
    }
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpa::_1
    }
}
#[doc = "Field `DPA` writer - Disable Preempt Ability"]
pub type DpaW<'a, REG> = crate::BitWriter<'a, REG, Dpa>;
impl<'a, REG> DpaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel n can suspend a lower priority channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpa::_0)
    }
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpa::_1)
    }
}
#[doc = "Enable Channel Preemption\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecp {
    #[doc = "0: Channel n cannot be suspended by a higher priority channel's service request"]
    _0 = 0,
    #[doc = "1: Channel n can be temporarily suspended by the service request of a higher priority channel"]
    _1 = 1,
}
impl From<Ecp> for bool {
    #[inline(always)]
    fn from(variant: Ecp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECP` reader - Enable Channel Preemption"]
pub type EcpR = crate::BitReader<Ecp>;
impl EcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecp {
        match self.bits {
            false => Ecp::_0,
            true => Ecp::_1,
        }
    }
    #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecp::_0
    }
    #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecp::_1
    }
}
#[doc = "Field `ECP` writer - Enable Channel Preemption"]
pub type EcpW<'a, REG> = crate::BitWriter<'a, REG, Ecp>;
impl<'a, REG> EcpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecp::_0)
    }
    #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecp::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline(always)]
    pub fn chpri(&self) -> ChpriR {
        ChpriR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 6 - Disable Preempt Ability"]
    #[inline(always)]
    pub fn dpa(&self) -> DpaR {
        DpaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Channel Preemption"]
    #[inline(always)]
    pub fn ecp(&self) -> EcpR {
        EcpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline(always)]
    #[must_use]
    pub fn chpri(&mut self) -> ChpriW<DchpriSpec> {
        ChpriW::new(self, 0)
    }
    #[doc = "Bit 6 - Disable Preempt Ability"]
    #[inline(always)]
    #[must_use]
    pub fn dpa(&mut self) -> DpaW<DchpriSpec> {
        DpaW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Channel Preemption"]
    #[inline(always)]
    #[must_use]
    pub fn ecp(&mut self) -> EcpW<DchpriSpec> {
        EcpW::new(self, 7)
    }
}
#[doc = "Channel n Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dchpri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dchpri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DchpriSpec;
impl crate::RegisterSpec for DchpriSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dchpri::R`](R) reader structure"]
impl crate::Readable for DchpriSpec {}
#[doc = "`write(|w| ..)` method takes [`dchpri::W`](W) writer structure"]
impl crate::Writable for DchpriSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DCHPRI%s to value 0"]
impl crate::Resettable for DchpriSpec {
    const RESET_VALUE: u8 = 0;
}
