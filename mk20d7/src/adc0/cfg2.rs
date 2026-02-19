#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Long sample time select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adlsts {
    #[doc = "0: Default longest sample time (20 extra ADCK cycles; 24 ADCK cycles total)."]
    _00 = 0,
    #[doc = "1: 12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    _01 = 1,
    #[doc = "2: 6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    _10 = 2,
    #[doc = "3: 2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    _11 = 3,
}
impl From<Adlsts> for u8 {
    #[inline(always)]
    fn from(variant: Adlsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adlsts {
    type Ux = u8;
}
impl crate::IsEnum for Adlsts {}
#[doc = "Field `ADLSTS` reader - Long sample time select"]
pub type AdlstsR = crate::FieldReader<Adlsts>;
impl AdlstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adlsts {
        match self.bits {
            0 => Adlsts::_00,
            1 => Adlsts::_01,
            2 => Adlsts::_10,
            3 => Adlsts::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Default longest sample time (20 extra ADCK cycles; 24 ADCK cycles total)."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adlsts::_00
    }
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Adlsts::_01
    }
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adlsts::_10
    }
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adlsts::_11
    }
}
#[doc = "Field `ADLSTS` writer - Long sample time select"]
pub type AdlstsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adlsts, crate::Safe>;
impl<'a, REG> AdlstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default longest sample time (20 extra ADCK cycles; 24 ADCK cycles total)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsts::_00)
    }
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsts::_01)
    }
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsts::_10)
    }
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsts::_11)
    }
}
#[doc = "High speed configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adhsc {
    #[doc = "0: Normal conversion sequence selected."]
    _0 = 0,
    #[doc = "1: High speed conversion sequence selected (2 additional ADCK cycles to total conversion time)."]
    _1 = 1,
}
impl From<Adhsc> for bool {
    #[inline(always)]
    fn from(variant: Adhsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADHSC` reader - High speed configuration"]
pub type AdhscR = crate::BitReader<Adhsc>;
impl AdhscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adhsc {
        match self.bits {
            false => Adhsc::_0,
            true => Adhsc::_1,
        }
    }
    #[doc = "Normal conversion sequence selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adhsc::_0
    }
    #[doc = "High speed conversion sequence selected (2 additional ADCK cycles to total conversion time)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adhsc::_1
    }
}
#[doc = "Field `ADHSC` writer - High speed configuration"]
pub type AdhscW<'a, REG> = crate::BitWriter<'a, REG, Adhsc>;
impl<'a, REG> AdhscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal conversion sequence selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adhsc::_0)
    }
    #[doc = "High speed conversion sequence selected (2 additional ADCK cycles to total conversion time)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adhsc::_1)
    }
}
#[doc = "Asynchronous clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adacken {
    #[doc = "0: Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    _0 = 0,
    #[doc = "1: Asynchronous clock and clock output enabled regardless of the state of the ADC."]
    _1 = 1,
}
impl From<Adacken> for bool {
    #[inline(always)]
    fn from(variant: Adacken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADACKEN` reader - Asynchronous clock output enable"]
pub type AdackenR = crate::BitReader<Adacken>;
impl AdackenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adacken {
        match self.bits {
            false => Adacken::_0,
            true => Adacken::_1,
        }
    }
    #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adacken::_0
    }
    #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adacken::_1
    }
}
#[doc = "Field `ADACKEN` writer - Asynchronous clock output enable"]
pub type AdackenW<'a, REG> = crate::BitWriter<'a, REG, Adacken>;
impl<'a, REG> AdackenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adacken::_0)
    }
    #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adacken::_1)
    }
}
#[doc = "ADC Mux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Muxsel {
    #[doc = "0: ADxxa channels are selected."]
    _0 = 0,
    #[doc = "1: ADxxb channels are selected."]
    _1 = 1,
}
impl From<Muxsel> for bool {
    #[inline(always)]
    fn from(variant: Muxsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUXSEL` reader - ADC Mux select"]
pub type MuxselR = crate::BitReader<Muxsel>;
impl MuxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Muxsel {
        match self.bits {
            false => Muxsel::_0,
            true => Muxsel::_1,
        }
    }
    #[doc = "ADxxa channels are selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Muxsel::_0
    }
    #[doc = "ADxxb channels are selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Muxsel::_1
    }
}
#[doc = "Field `MUXSEL` writer - ADC Mux select"]
pub type MuxselW<'a, REG> = crate::BitWriter<'a, REG, Muxsel>;
impl<'a, REG> MuxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADxxa channels are selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::_0)
    }
    #[doc = "ADxxb channels are selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Long sample time select"]
    #[inline(always)]
    pub fn adlsts(&self) -> AdlstsR {
        AdlstsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - High speed configuration"]
    #[inline(always)]
    pub fn adhsc(&self) -> AdhscR {
        AdhscR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous clock output enable"]
    #[inline(always)]
    pub fn adacken(&self) -> AdackenR {
        AdackenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Mux select"]
    #[inline(always)]
    pub fn muxsel(&self) -> MuxselR {
        MuxselR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Long sample time select"]
    #[inline(always)]
    #[must_use]
    pub fn adlsts(&mut self) -> AdlstsW<Cfg2Spec> {
        AdlstsW::new(self, 0)
    }
    #[doc = "Bit 2 - High speed configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adhsc(&mut self) -> AdhscW<Cfg2Spec> {
        AdhscW::new(self, 2)
    }
    #[doc = "Bit 3 - Asynchronous clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn adacken(&mut self) -> AdackenW<Cfg2Spec> {
        AdackenW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Mux select"]
    #[inline(always)]
    #[must_use]
    pub fn muxsel(&mut self) -> MuxselW<Cfg2Spec> {
        MuxselW::new(self, 4)
    }
}
#[doc = "Configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0;
}
