#[doc = "Register `C4` reader"]
pub type R = crate::R<C4Spec>;
#[doc = "Register `C4` writer"]
pub type W = crate::W<C4Spec>;
#[doc = "Field `BRFA` reader - Baud Rate Fine Adjust"]
pub type BrfaR = crate::FieldReader;
#[doc = "Field `BRFA` writer - Baud Rate Fine Adjust"]
pub type BrfaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M10 {
    #[doc = "0: The parity bit is the ninth bit in the serial transmission."]
    _0 = 0,
    #[doc = "1: The parity bit is the tenth bit in the serial transmission."]
    _1 = 1,
}
impl From<M10> for bool {
    #[inline(always)]
    fn from(variant: M10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M10` reader - 10-bit Mode select"]
pub type M10R = crate::BitReader<M10>;
impl M10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M10 {
        match self.bits {
            false => M10::_0,
            true => M10::_1,
        }
    }
    #[doc = "The parity bit is the ninth bit in the serial transmission."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M10::_0
    }
    #[doc = "The parity bit is the tenth bit in the serial transmission."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M10::_1
    }
}
#[doc = "Field `M10` writer - 10-bit Mode select"]
pub type M10W<'a, REG> = crate::BitWriter<'a, REG, M10>;
impl<'a, REG> M10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The parity bit is the ninth bit in the serial transmission."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(M10::_0)
    }
    #[doc = "The parity bit is the tenth bit in the serial transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(M10::_1)
    }
}
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maen2 {
    #[doc = "0: All data received is transferred to the data buffer if MAEN1 is cleared."]
    _0 = 0,
    #[doc = "1: All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer.This bit must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    _1 = 1,
}
impl From<Maen2> for bool {
    #[inline(always)]
    fn from(variant: Maen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAEN2` reader - Match Address Mode Enable 2"]
pub type Maen2R = crate::BitReader<Maen2>;
impl Maen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maen2 {
        match self.bits {
            false => Maen2::_0,
            true => Maen2::_1,
        }
    }
    #[doc = "All data received is transferred to the data buffer if MAEN1 is cleared."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Maen2::_0
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer.This bit must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Maen2::_1
    }
}
#[doc = "Field `MAEN2` writer - Match Address Mode Enable 2"]
pub type Maen2W<'a, REG> = crate::BitWriter<'a, REG, Maen2>;
impl<'a, REG> Maen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All data received is transferred to the data buffer if MAEN1 is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Maen2::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer.This bit must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Maen2::_1)
    }
}
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maen1 {
    #[doc = "0: All data received is transferred to the data buffer if MAEN2 is cleared."]
    _0 = 0,
    #[doc = "1: All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer.This bit must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    _1 = 1,
}
impl From<Maen1> for bool {
    #[inline(always)]
    fn from(variant: Maen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAEN1` reader - Match Address Mode Enable 1"]
pub type Maen1R = crate::BitReader<Maen1>;
impl Maen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maen1 {
        match self.bits {
            false => Maen1::_0,
            true => Maen1::_1,
        }
    }
    #[doc = "All data received is transferred to the data buffer if MAEN2 is cleared."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Maen1::_0
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer.This bit must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Maen1::_1
    }
}
#[doc = "Field `MAEN1` writer - Match Address Mode Enable 1"]
pub type Maen1W<'a, REG> = crate::BitWriter<'a, REG, Maen1>;
impl<'a, REG> Maen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All data received is transferred to the data buffer if MAEN2 is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Maen1::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer.This bit must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Maen1::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline(always)]
    pub fn brfa(&self) -> BrfaR {
        BrfaR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10R {
        M10R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> Maen2R {
        Maen2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> Maen1R {
        Maen1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn brfa(&mut self) -> BrfaW<C4Spec> {
        BrfaW::new(self, 0)
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    #[must_use]
    pub fn m10(&mut self) -> M10W<C4Spec> {
        M10W::new(self, 5)
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn maen2(&mut self) -> Maen2W<C4Spec> {
        Maen2W::new(self, 6)
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn maen1(&mut self) -> Maen1W<C4Spec> {
        Maen1W::new(self, 7)
    }
}
#[doc = "UART Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4Spec;
impl crate::RegisterSpec for C4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c4::R`](R) reader structure"]
impl crate::Readable for C4Spec {}
#[doc = "`write(|w| ..)` method takes [`c4::W`](W) writer structure"]
impl crate::Writable for C4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C4 to value 0"]
impl crate::Resettable for C4Spec {
    const RESET_VALUE: u8 = 0;
}
