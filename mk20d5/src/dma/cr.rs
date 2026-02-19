#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Enable Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edbg {
    #[doc = "0: When in debug mode, the DMA continues to operate."]
    _0 = 0,
    #[doc = "1: When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared."]
    _1 = 1,
}
impl From<Edbg> for bool {
    #[inline(always)]
    fn from(variant: Edbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDBG` reader - Enable Debug"]
pub type EdbgR = crate::BitReader<Edbg>;
impl EdbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edbg {
        match self.bits {
            false => Edbg::_0,
            true => Edbg::_1,
        }
    }
    #[doc = "When in debug mode, the DMA continues to operate."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Edbg::_0
    }
    #[doc = "When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Edbg::_1
    }
}
#[doc = "Field `EDBG` writer - Enable Debug"]
pub type EdbgW<'a, REG> = crate::BitWriter<'a, REG, Edbg>;
impl<'a, REG> EdbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When in debug mode, the DMA continues to operate."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Edbg::_0)
    }
    #[doc = "When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Edbg::_1)
    }
}
#[doc = "Enable Round Robin Channel Arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erca {
    #[doc = "0: Fixed priority arbitration is used for channel selection."]
    _0 = 0,
    #[doc = "1: Round robin arbitration is used for channel selection."]
    _1 = 1,
}
impl From<Erca> for bool {
    #[inline(always)]
    fn from(variant: Erca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERCA` reader - Enable Round Robin Channel Arbitration"]
pub type ErcaR = crate::BitReader<Erca>;
impl ErcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erca {
        match self.bits {
            false => Erca::_0,
            true => Erca::_1,
        }
    }
    #[doc = "Fixed priority arbitration is used for channel selection."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Erca::_0
    }
    #[doc = "Round robin arbitration is used for channel selection."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Erca::_1
    }
}
#[doc = "Field `ERCA` writer - Enable Round Robin Channel Arbitration"]
pub type ErcaW<'a, REG> = crate::BitWriter<'a, REG, Erca>;
impl<'a, REG> ErcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed priority arbitration is used for channel selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Erca::_0)
    }
    #[doc = "Round robin arbitration is used for channel selection."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Erca::_1)
    }
}
#[doc = "Halt On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hoe {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    _1 = 1,
}
impl From<Hoe> for bool {
    #[inline(always)]
    fn from(variant: Hoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOE` reader - Halt On Error"]
pub type HoeR = crate::BitReader<Hoe>;
impl HoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hoe {
        match self.bits {
            false => Hoe::_0,
            true => Hoe::_1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hoe::_0
    }
    #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hoe::_1
    }
}
#[doc = "Field `HOE` writer - Halt On Error"]
pub type HoeW<'a, REG> = crate::BitWriter<'a, REG, Hoe>;
impl<'a, REG> HoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hoe::_0)
    }
    #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hoe::_1)
    }
}
#[doc = "Halt DMA Operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halt {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
    _1 = 1,
}
impl From<Halt> for bool {
    #[inline(always)]
    fn from(variant: Halt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halt DMA Operations"]
pub type HaltR = crate::BitReader<Halt>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halt {
        match self.bits {
            false => Halt::_0,
            true => Halt::_1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Halt::_0
    }
    #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Halt::_1
    }
}
#[doc = "Field `HALT` writer - Halt DMA Operations"]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG, Halt>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::_0)
    }
    #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::_1)
    }
}
#[doc = "Continuous Link Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clm {
    #[doc = "0: A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    _0 = 0,
    #[doc = "1: A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    _1 = 1,
}
impl From<Clm> for bool {
    #[inline(always)]
    fn from(variant: Clm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLM` reader - Continuous Link Mode"]
pub type ClmR = crate::BitReader<Clm>;
impl ClmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clm {
        match self.bits {
            false => Clm::_0,
            true => Clm::_1,
        }
    }
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clm::_0
    }
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clm::_1
    }
}
#[doc = "Field `CLM` writer - Continuous Link Mode"]
pub type ClmW<'a, REG> = crate::BitWriter<'a, REG, Clm>;
impl<'a, REG> ClmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clm::_0)
    }
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clm::_1)
    }
}
#[doc = "Enable Minor Loop Mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emlm {
    #[doc = "0: Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    _0 = 0,
    #[doc = "1: Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    _1 = 1,
}
impl From<Emlm> for bool {
    #[inline(always)]
    fn from(variant: Emlm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMLM` reader - Enable Minor Loop Mapping"]
pub type EmlmR = crate::BitReader<Emlm>;
impl EmlmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emlm {
        match self.bits {
            false => Emlm::_0,
            true => Emlm::_1,
        }
    }
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Emlm::_0
    }
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Emlm::_1
    }
}
#[doc = "Field `EMLM` writer - Enable Minor Loop Mapping"]
pub type EmlmW<'a, REG> = crate::BitWriter<'a, REG, Emlm>;
impl<'a, REG> EmlmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Emlm::_0)
    }
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Emlm::_1)
    }
}
#[doc = "Error Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecx {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the ES register and generating an optional error interrupt."]
    _1 = 1,
}
impl From<Ecx> for bool {
    #[inline(always)]
    fn from(variant: Ecx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECX` reader - Error Cancel Transfer"]
pub type EcxR = crate::BitReader<Ecx>;
impl EcxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecx {
        match self.bits {
            false => Ecx::_0,
            true => Ecx::_1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecx::_0
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the ES register and generating an optional error interrupt."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecx::_1
    }
}
#[doc = "Field `ECX` writer - Error Cancel Transfer"]
pub type EcxW<'a, REG> = crate::BitWriter<'a, REG, Ecx>;
impl<'a, REG> EcxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecx::_0)
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the ES register and generating an optional error interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecx::_1)
    }
}
#[doc = "Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cx {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    _1 = 1,
}
impl From<Cx> for bool {
    #[inline(always)]
    fn from(variant: Cx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CX` reader - Cancel Transfer"]
pub type CxR = crate::BitReader<Cx>;
impl CxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cx {
        match self.bits {
            false => Cx::_0,
            true => Cx::_1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cx::_0
    }
    #[doc = "Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cx::_1
    }
}
#[doc = "Field `CX` writer - Cancel Transfer"]
pub type CxW<'a, REG> = crate::BitWriter<'a, REG, Cx>;
impl<'a, REG> CxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cx::_0)
    }
    #[doc = "Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cx::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    pub fn edbg(&self) -> EdbgR {
        EdbgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub fn erca(&self) -> ErcaR {
        ErcaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    pub fn hoe(&self) -> HoeR {
        HoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    pub fn clm(&self) -> ClmR {
        ClmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    pub fn emlm(&self) -> EmlmR {
        EmlmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    pub fn ecx(&self) -> EcxR {
        EcxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    pub fn cx(&self) -> CxR {
        CxR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    #[must_use]
    pub fn edbg(&mut self) -> EdbgW<CrSpec> {
        EdbgW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn erca(&mut self) -> ErcaW<CrSpec> {
        ErcaW::new(self, 2)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    #[must_use]
    pub fn hoe(&mut self) -> HoeW<CrSpec> {
        HoeW::new(self, 4)
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<CrSpec> {
        HaltW::new(self, 5)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    #[must_use]
    pub fn clm(&mut self) -> ClmW<CrSpec> {
        ClmW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    #[must_use]
    pub fn emlm(&mut self) -> EmlmW<CrSpec> {
        EmlmW::new(self, 7)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ecx(&mut self) -> EcxW<CrSpec> {
        EcxW::new(self, 16)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn cx(&mut self) -> CxW<CrSpec> {
        CxW::new(self, 17)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
