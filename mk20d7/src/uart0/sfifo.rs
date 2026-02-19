#[doc = "Register `SFIFO` reader"]
pub type R = crate::R<SfifoSpec>;
#[doc = "Register `SFIFO` writer"]
pub type W = crate::W<SfifoSpec>;
#[doc = "Receiver Buffer Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxuf {
    #[doc = "0: No receive buffer underflow has occurred since the last time the flag was cleared."]
    _0 = 0,
    #[doc = "1: At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    _1 = 1,
}
impl From<Rxuf> for bool {
    #[inline(always)]
    fn from(variant: Rxuf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXUF` reader - Receiver Buffer Underflow Flag"]
pub type RxufR = crate::BitReader<Rxuf>;
impl RxufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxuf {
        match self.bits {
            false => Rxuf::_0,
            true => Rxuf::_1,
        }
    }
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxuf::_0
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxuf::_1
    }
}
#[doc = "Field `RXUF` writer - Receiver Buffer Underflow Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG, Rxuf>;
impl<'a, REG> RxufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxuf::_0)
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxuf::_1)
    }
}
#[doc = "Transmitter Buffer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txof {
    #[doc = "0: No transmit buffer overflow has occurred since the last time the flag was cleared."]
    _0 = 0,
    #[doc = "1: At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    _1 = 1,
}
impl From<Txof> for bool {
    #[inline(always)]
    fn from(variant: Txof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOF` reader - Transmitter Buffer Overflow Flag"]
pub type TxofR = crate::BitReader<Txof>;
impl TxofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txof {
        match self.bits {
            false => Txof::_0,
            true => Txof::_1,
        }
    }
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txof::_0
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txof::_1
    }
}
#[doc = "Field `TXOF` writer - Transmitter Buffer Overflow Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG, Txof>;
impl<'a, REG> TxofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txof::_0)
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txof::_1)
    }
}
#[doc = "Receive Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxempt {
    #[doc = "0: Receive buffer is not empty."]
    _0 = 0,
    #[doc = "1: Receive buffer is empty."]
    _1 = 1,
}
impl From<Rxempt> for bool {
    #[inline(always)]
    fn from(variant: Rxempt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPT` reader - Receive Buffer/FIFO Empty"]
pub type RxemptR = crate::BitReader<Rxempt>;
impl RxemptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxempt {
        match self.bits {
            false => Rxempt::_0,
            true => Rxempt::_1,
        }
    }
    #[doc = "Receive buffer is not empty."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxempt::_0
    }
    #[doc = "Receive buffer is empty."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxempt::_1
    }
}
#[doc = "Transmit Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempt {
    #[doc = "0: Transmit buffer is not empty."]
    _0 = 0,
    #[doc = "1: Transmit buffer is empty."]
    _1 = 1,
}
impl From<Txempt> for bool {
    #[inline(always)]
    fn from(variant: Txempt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPT` reader - Transmit Buffer/FIFO Empty"]
pub type TxemptR = crate::BitReader<Txempt>;
impl TxemptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txempt {
        match self.bits {
            false => Txempt::_0,
            true => Txempt::_1,
        }
    }
    #[doc = "Transmit buffer is not empty."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txempt::_0
    }
    #[doc = "Transmit buffer is empty."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txempt::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RxemptR {
        RxemptR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn txempt(&self) -> TxemptR {
        TxemptR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<SfifoSpec> {
        RxufW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<SfifoSpec> {
        TxofW::new(self, 1)
    }
}
#[doc = "UART FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfifoSpec;
impl crate::RegisterSpec for SfifoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sfifo::R`](R) reader structure"]
impl crate::Readable for SfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`sfifo::W`](W) writer structure"]
impl crate::Writable for SfifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SFIFO to value 0xc0"]
impl crate::Resettable for SfifoSpec {
    const RESET_VALUE: u8 = 0xc0;
}
