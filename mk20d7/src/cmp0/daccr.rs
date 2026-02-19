#[doc = "Register `DACCR` reader"]
pub type R = crate::R<DaccrSpec>;
#[doc = "Register `DACCR` writer"]
pub type W = crate::W<DaccrSpec>;
#[doc = "Field `VOSEL` reader - DAC Output Voltage Select"]
pub type VoselR = crate::FieldReader;
#[doc = "Field `VOSEL` writer - DAC Output Voltage Select"]
pub type VoselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrsel {
    #[doc = "0: Vin1 is selected as resistor ladder network supply reference Vin."]
    _0 = 0,
    #[doc = "1: Vin2 is selected as resistor ladder network supply reference Vin."]
    _1 = 1,
}
impl From<Vrsel> for bool {
    #[inline(always)]
    fn from(variant: Vrsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRSEL` reader - Supply Voltage Reference Source Select"]
pub type VrselR = crate::BitReader<Vrsel>;
impl VrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrsel {
        match self.bits {
            false => Vrsel::_0,
            true => Vrsel::_1,
        }
    }
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vrsel::_0
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vrsel::_1
    }
}
#[doc = "Field `VRSEL` writer - Supply Voltage Reference Source Select"]
pub type VrselW<'a, REG> = crate::BitWriter<'a, REG, Vrsel>;
impl<'a, REG> VrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::_0)
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::_1)
    }
}
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacen {
    #[doc = "0: DAC is disabled."]
    _0 = 0,
    #[doc = "1: DAC is enabled."]
    _1 = 1,
}
impl From<Dacen> for bool {
    #[inline(always)]
    fn from(variant: Dacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub type DacenR = crate::BitReader<Dacen>;
impl DacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacen {
        match self.bits {
            false => Dacen::_0,
            true => Dacen::_1,
        }
    }
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dacen::_0
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dacen::_1
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG, Dacen>;
impl<'a, REG> DacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacen::_0)
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacen::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&self) -> VoselR {
        VoselR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&self) -> VrselR {
        VrselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn vosel(&mut self) -> VoselW<DaccrSpec> {
        VoselW::new(self, 0)
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn vrsel(&mut self) -> VrselW<DaccrSpec> {
        VrselW::new(self, 6)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DacenW<DaccrSpec> {
        DacenW::new(self, 7)
    }
}
#[doc = "DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaccrSpec;
impl crate::RegisterSpec for DaccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`daccr::R`](R) reader structure"]
impl crate::Readable for DaccrSpec {}
#[doc = "`write(|w| ..)` method takes [`daccr::W`](W) writer structure"]
impl crate::Writable for DaccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DACCR to value 0"]
impl crate::Resettable for DaccrSpec {
    const RESET_VALUE: u8 = 0;
}
