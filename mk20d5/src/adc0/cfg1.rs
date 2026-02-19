#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Input clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adiclk {
    #[doc = "0: Bus clock."]
    _00 = 0,
    #[doc = "1: Bus clock divided by 2."]
    _01 = 1,
    #[doc = "2: Alternate clock (ALTCLK)."]
    _10 = 2,
    #[doc = "3: Asynchronous clock (ADACK)."]
    _11 = 3,
}
impl From<Adiclk> for u8 {
    #[inline(always)]
    fn from(variant: Adiclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adiclk {
    type Ux = u8;
}
impl crate::IsEnum for Adiclk {}
#[doc = "Field `ADICLK` reader - Input clock select"]
pub type AdiclkR = crate::FieldReader<Adiclk>;
impl AdiclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adiclk {
        match self.bits {
            0 => Adiclk::_00,
            1 => Adiclk::_01,
            2 => Adiclk::_10,
            3 => Adiclk::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Bus clock."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adiclk::_00
    }
    #[doc = "Bus clock divided by 2."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Adiclk::_01
    }
    #[doc = "Alternate clock (ALTCLK)."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adiclk::_10
    }
    #[doc = "Asynchronous clock (ADACK)."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adiclk::_11
    }
}
#[doc = "Field `ADICLK` writer - Input clock select"]
pub type AdiclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adiclk, crate::Safe>;
impl<'a, REG> AdiclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adiclk::_00)
    }
    #[doc = "Bus clock divided by 2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Adiclk::_01)
    }
    #[doc = "Alternate clock (ALTCLK)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adiclk::_10)
    }
    #[doc = "Asynchronous clock (ADACK)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adiclk::_11)
    }
}
#[doc = "Conversion mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: When DIFF=0: It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    _00 = 0,
    #[doc = "1: When DIFF=0: It is single-ended 12-bit conversion; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    _01 = 1,
    #[doc = "2: When DIFF=0: It is single-ended 10-bit conversion; when DIFF=1, it is differential 11-bit conversion with 2's complement output."]
    _10 = 2,
    #[doc = "3: When DIFF=0: It is single-ended 16-bit conversion; when DIFF=1, it is differential 16-bit conversion with 2's complement output ."]
    _11 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Conversion mode selection"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::_00,
            1 => Mode::_01,
            2 => Mode::_10,
            3 => Mode::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "When DIFF=0: It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Mode::_00
    }
    #[doc = "When DIFF=0: It is single-ended 12-bit conversion; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Mode::_01
    }
    #[doc = "When DIFF=0: It is single-ended 10-bit conversion; when DIFF=1, it is differential 11-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Mode::_10
    }
    #[doc = "When DIFF=0: It is single-ended 16-bit conversion; when DIFF=1, it is differential 16-bit conversion with 2's complement output ."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Mode::_11
    }
}
#[doc = "Field `MODE` writer - Conversion mode selection"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When DIFF=0: It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_00)
    }
    #[doc = "When DIFF=0: It is single-ended 12-bit conversion; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_01)
    }
    #[doc = "When DIFF=0: It is single-ended 10-bit conversion; when DIFF=1, it is differential 11-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_10)
    }
    #[doc = "When DIFF=0: It is single-ended 16-bit conversion; when DIFF=1, it is differential 16-bit conversion with 2's complement output ."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_11)
    }
}
#[doc = "Sample time configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adlsmp {
    #[doc = "0: Short sample time."]
    _0 = 0,
    #[doc = "1: Long sample time."]
    _1 = 1,
}
impl From<Adlsmp> for bool {
    #[inline(always)]
    fn from(variant: Adlsmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADLSMP` reader - Sample time configuration"]
pub type AdlsmpR = crate::BitReader<Adlsmp>;
impl AdlsmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adlsmp {
        match self.bits {
            false => Adlsmp::_0,
            true => Adlsmp::_1,
        }
    }
    #[doc = "Short sample time."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adlsmp::_0
    }
    #[doc = "Long sample time."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adlsmp::_1
    }
}
#[doc = "Field `ADLSMP` writer - Sample time configuration"]
pub type AdlsmpW<'a, REG> = crate::BitWriter<'a, REG, Adlsmp>;
impl<'a, REG> AdlsmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short sample time."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsmp::_0)
    }
    #[doc = "Long sample time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsmp::_1)
    }
}
#[doc = "Clock divide select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adiv {
    #[doc = "0: The divide ratio is 1 and the clock rate is input clock."]
    _00 = 0,
    #[doc = "1: The divide ratio is 2 and the clock rate is (input clock)/2."]
    _01 = 1,
    #[doc = "2: The divide ratio is 4 and the clock rate is (input clock)/4."]
    _10 = 2,
    #[doc = "3: The divide ratio is 8 and the clock rate is (input clock)/8."]
    _11 = 3,
}
impl From<Adiv> for u8 {
    #[inline(always)]
    fn from(variant: Adiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adiv {
    type Ux = u8;
}
impl crate::IsEnum for Adiv {}
#[doc = "Field `ADIV` reader - Clock divide select"]
pub type AdivR = crate::FieldReader<Adiv>;
impl AdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adiv {
        match self.bits {
            0 => Adiv::_00,
            1 => Adiv::_01,
            2 => Adiv::_10,
            3 => Adiv::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adiv::_00
    }
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Adiv::_01
    }
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adiv::_10
    }
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adiv::_11
    }
}
#[doc = "Field `ADIV` writer - Clock divide select"]
pub type AdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adiv, crate::Safe>;
impl<'a, REG> AdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adiv::_00)
    }
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Adiv::_01)
    }
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adiv::_10)
    }
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adiv::_11)
    }
}
#[doc = "Low-power configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adlpc {
    #[doc = "0: Normal power configuration."]
    _0 = 0,
    #[doc = "1: Low power configuration. The power is reduced at the expense of maximum clock speed."]
    _1 = 1,
}
impl From<Adlpc> for bool {
    #[inline(always)]
    fn from(variant: Adlpc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADLPC` reader - Low-power configuration"]
pub type AdlpcR = crate::BitReader<Adlpc>;
impl AdlpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adlpc {
        match self.bits {
            false => Adlpc::_0,
            true => Adlpc::_1,
        }
    }
    #[doc = "Normal power configuration."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adlpc::_0
    }
    #[doc = "Low power configuration. The power is reduced at the expense of maximum clock speed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adlpc::_1
    }
}
#[doc = "Field `ADLPC` writer - Low-power configuration"]
pub type AdlpcW<'a, REG> = crate::BitWriter<'a, REG, Adlpc>;
impl<'a, REG> AdlpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal power configuration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adlpc::_0)
    }
    #[doc = "Low power configuration. The power is reduced at the expense of maximum clock speed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adlpc::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Input clock select"]
    #[inline(always)]
    pub fn adiclk(&self) -> AdiclkR {
        AdiclkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Sample time configuration"]
    #[inline(always)]
    pub fn adlsmp(&self) -> AdlsmpR {
        AdlsmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Clock divide select"]
    #[inline(always)]
    pub fn adiv(&self) -> AdivR {
        AdivR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Low-power configuration"]
    #[inline(always)]
    pub fn adlpc(&self) -> AdlpcR {
        AdlpcR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input clock select"]
    #[inline(always)]
    #[must_use]
    pub fn adiclk(&mut self) -> AdiclkW<Cfg1Spec> {
        AdiclkW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Cfg1Spec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 4 - Sample time configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adlsmp(&mut self) -> AdlsmpW<Cfg1Spec> {
        AdlsmpW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Clock divide select"]
    #[inline(always)]
    #[must_use]
    pub fn adiv(&mut self) -> AdivW<Cfg1Spec> {
        AdivW::new(self, 5)
    }
    #[doc = "Bit 7 - Low-power configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adlpc(&mut self) -> AdlpcW<Cfg1Spec> {
        AdlpcW::new(self, 7)
    }
}
#[doc = "ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
