#[doc = "Register `PWMLOAD` reader"]
pub type R = crate::R<PwmloadSpec>;
#[doc = "Register `PWMLOAD` writer"]
pub type W = crate::W<PwmloadSpec>;
#[doc = "Channel 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0sel {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<Ch0sel> for bool {
    #[inline(always)]
    fn from(variant: Ch0sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0SEL` reader - Channel 0 Select"]
pub type Ch0selR = crate::BitReader<Ch0sel>;
impl Ch0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0sel {
        match self.bits {
            false => Ch0sel::_0,
            true => Ch0sel::_1,
        }
    }
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch0sel::_0
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch0sel::_1
    }
}
#[doc = "Field `CH0SEL` writer - Channel 0 Select"]
pub type Ch0selW<'a, REG> = crate::BitWriter<'a, REG, Ch0sel>;
impl<'a, REG> Ch0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0sel::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0sel::_1)
    }
}
#[doc = "Channel 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1sel {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<Ch1sel> for bool {
    #[inline(always)]
    fn from(variant: Ch1sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1SEL` reader - Channel 1 Select"]
pub type Ch1selR = crate::BitReader<Ch1sel>;
impl Ch1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1sel {
        match self.bits {
            false => Ch1sel::_0,
            true => Ch1sel::_1,
        }
    }
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch1sel::_0
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch1sel::_1
    }
}
#[doc = "Field `CH1SEL` writer - Channel 1 Select"]
pub type Ch1selW<'a, REG> = crate::BitWriter<'a, REG, Ch1sel>;
impl<'a, REG> Ch1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1sel::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1sel::_1)
    }
}
#[doc = "Channel 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2sel {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<Ch2sel> for bool {
    #[inline(always)]
    fn from(variant: Ch2sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2SEL` reader - Channel 2 Select"]
pub type Ch2selR = crate::BitReader<Ch2sel>;
impl Ch2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2sel {
        match self.bits {
            false => Ch2sel::_0,
            true => Ch2sel::_1,
        }
    }
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch2sel::_0
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch2sel::_1
    }
}
#[doc = "Field `CH2SEL` writer - Channel 2 Select"]
pub type Ch2selW<'a, REG> = crate::BitWriter<'a, REG, Ch2sel>;
impl<'a, REG> Ch2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2sel::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2sel::_1)
    }
}
#[doc = "Channel 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3sel {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<Ch3sel> for bool {
    #[inline(always)]
    fn from(variant: Ch3sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3SEL` reader - Channel 3 Select"]
pub type Ch3selR = crate::BitReader<Ch3sel>;
impl Ch3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3sel {
        match self.bits {
            false => Ch3sel::_0,
            true => Ch3sel::_1,
        }
    }
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch3sel::_0
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch3sel::_1
    }
}
#[doc = "Field `CH3SEL` writer - Channel 3 Select"]
pub type Ch3selW<'a, REG> = crate::BitWriter<'a, REG, Ch3sel>;
impl<'a, REG> Ch3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3sel::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3sel::_1)
    }
}
#[doc = "Channel 4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4sel {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<Ch4sel> for bool {
    #[inline(always)]
    fn from(variant: Ch4sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4SEL` reader - Channel 4 Select"]
pub type Ch4selR = crate::BitReader<Ch4sel>;
impl Ch4selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4sel {
        match self.bits {
            false => Ch4sel::_0,
            true => Ch4sel::_1,
        }
    }
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch4sel::_0
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch4sel::_1
    }
}
#[doc = "Field `CH4SEL` writer - Channel 4 Select"]
pub type Ch4selW<'a, REG> = crate::BitWriter<'a, REG, Ch4sel>;
impl<'a, REG> Ch4selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4sel::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4sel::_1)
    }
}
#[doc = "Channel 5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5sel {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<Ch5sel> for bool {
    #[inline(always)]
    fn from(variant: Ch5sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5SEL` reader - Channel 5 Select"]
pub type Ch5selR = crate::BitReader<Ch5sel>;
impl Ch5selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5sel {
        match self.bits {
            false => Ch5sel::_0,
            true => Ch5sel::_1,
        }
    }
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch5sel::_0
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch5sel::_1
    }
}
#[doc = "Field `CH5SEL` writer - Channel 5 Select"]
pub type Ch5selW<'a, REG> = crate::BitWriter<'a, REG, Ch5sel>;
impl<'a, REG> Ch5selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5sel::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5sel::_1)
    }
}
#[doc = "Channel 6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6sel {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<Ch6sel> for bool {
    #[inline(always)]
    fn from(variant: Ch6sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6SEL` reader - Channel 6 Select"]
pub type Ch6selR = crate::BitReader<Ch6sel>;
impl Ch6selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6sel {
        match self.bits {
            false => Ch6sel::_0,
            true => Ch6sel::_1,
        }
    }
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch6sel::_0
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch6sel::_1
    }
}
#[doc = "Field `CH6SEL` writer - Channel 6 Select"]
pub type Ch6selW<'a, REG> = crate::BitWriter<'a, REG, Ch6sel>;
impl<'a, REG> Ch6selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6sel::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6sel::_1)
    }
}
#[doc = "Channel 7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7sel {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<Ch7sel> for bool {
    #[inline(always)]
    fn from(variant: Ch7sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7SEL` reader - Channel 7 Select"]
pub type Ch7selR = crate::BitReader<Ch7sel>;
impl Ch7selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7sel {
        match self.bits {
            false => Ch7sel::_0,
            true => Ch7sel::_1,
        }
    }
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch7sel::_0
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch7sel::_1
    }
}
#[doc = "Field `CH7SEL` writer - Channel 7 Select"]
pub type Ch7selW<'a, REG> = crate::BitWriter<'a, REG, Ch7sel>;
impl<'a, REG> Ch7selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7sel::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7sel::_1)
    }
}
#[doc = "Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldok {
    #[doc = "0: Loading updated values is disabled."]
    _0 = 0,
    #[doc = "1: Loading updated values is enabled."]
    _1 = 1,
}
impl From<Ldok> for bool {
    #[inline(always)]
    fn from(variant: Ldok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOK` reader - Load Enable"]
pub type LdokR = crate::BitReader<Ldok>;
impl LdokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldok {
        match self.bits {
            false => Ldok::_0,
            true => Ldok::_1,
        }
    }
    #[doc = "Loading updated values is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ldok::_0
    }
    #[doc = "Loading updated values is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ldok::_1
    }
}
#[doc = "Field `LDOK` writer - Load Enable"]
pub type LdokW<'a, REG> = crate::BitWriter<'a, REG, Ldok>;
impl<'a, REG> LdokW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loading updated values is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ldok::_0)
    }
    #[doc = "Loading updated values is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ldok::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline(always)]
    pub fn ch0sel(&self) -> Ch0selR {
        Ch0selR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline(always)]
    pub fn ch1sel(&self) -> Ch1selR {
        Ch1selR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline(always)]
    pub fn ch2sel(&self) -> Ch2selR {
        Ch2selR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline(always)]
    pub fn ch3sel(&self) -> Ch3selR {
        Ch3selR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline(always)]
    pub fn ch4sel(&self) -> Ch4selR {
        Ch4selR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline(always)]
    pub fn ch5sel(&self) -> Ch5selR {
        Ch5selR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline(always)]
    pub fn ch6sel(&self) -> Ch6selR {
        Ch6selR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline(always)]
    pub fn ch7sel(&self) -> Ch7selR {
        Ch7selR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline(always)]
    pub fn ldok(&self) -> LdokR {
        LdokR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch0sel(&mut self) -> Ch0selW<PwmloadSpec> {
        Ch0selW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch1sel(&mut self) -> Ch1selW<PwmloadSpec> {
        Ch1selW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch2sel(&mut self) -> Ch2selW<PwmloadSpec> {
        Ch2selW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch3sel(&mut self) -> Ch3selW<PwmloadSpec> {
        Ch3selW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch4sel(&mut self) -> Ch4selW<PwmloadSpec> {
        Ch4selW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch5sel(&mut self) -> Ch5selW<PwmloadSpec> {
        Ch5selW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch6sel(&mut self) -> Ch6selW<PwmloadSpec> {
        Ch6selW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch7sel(&mut self) -> Ch7selW<PwmloadSpec> {
        Ch7selW::new(self, 7)
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ldok(&mut self) -> LdokW<PwmloadSpec> {
        LdokW::new(self, 9)
    }
}
#[doc = "FTM PWM Load\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmloadSpec;
impl crate::RegisterSpec for PwmloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmload::R`](R) reader structure"]
impl crate::Readable for PwmloadSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmload::W`](W) writer structure"]
impl crate::Writable for PwmloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMLOAD to value 0"]
impl crate::Resettable for PwmloadSpec {
    const RESET_VALUE: u32 = 0;
}
