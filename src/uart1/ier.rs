#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1IerRbrieEnum {
    #[doc = "0: Disable the RDA interrupts."]
    DisableTheRdaInte = 0,
    #[doc = "1: Enable the RDA interrupts."]
    EnableTheRdaInter = 1,
}
impl From<Uart1IerRbrieEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1IerRbrieEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
pub type RbrieR = crate::BitReader<Uart1IerRbrieEnum>;
impl RbrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1IerRbrieEnum {
        match self.bits {
            false => Uart1IerRbrieEnum::DisableTheRdaInte,
            true => Uart1IerRbrieEnum::EnableTheRdaInter,
        }
    }
    #[doc = "Disable the RDA interrupts."]
    #[inline(always)]
    pub fn is_disable_the_rda_inte(&self) -> bool {
        *self == Uart1IerRbrieEnum::DisableTheRdaInte
    }
    #[doc = "Enable the RDA interrupts."]
    #[inline(always)]
    pub fn is_enable_the_rda_inter(&self) -> bool {
        *self == Uart1IerRbrieEnum::EnableTheRdaInter
    }
}
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
pub type RbrieW<'a, REG> = crate::BitWriter<'a, REG, Uart1IerRbrieEnum>;
impl<'a, REG> RbrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RDA interrupts."]
    #[inline(always)]
    pub fn disable_the_rda_inte(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerRbrieEnum::DisableTheRdaInte)
    }
    #[doc = "Enable the RDA interrupts."]
    #[inline(always)]
    pub fn enable_the_rda_inter(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerRbrieEnum::EnableTheRdaInter)
    }
}
#[doc = "THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1IerThreieEnum {
    #[doc = "0: Disable the THRE interrupts."]
    DisableTheThreInt = 0,
    #[doc = "1: Enable the THRE interrupts."]
    EnableTheThreInte = 1,
}
impl From<Uart1IerThreieEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1IerThreieEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREIE` reader - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
pub type ThreieR = crate::BitReader<Uart1IerThreieEnum>;
impl ThreieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1IerThreieEnum {
        match self.bits {
            false => Uart1IerThreieEnum::DisableTheThreInt,
            true => Uart1IerThreieEnum::EnableTheThreInte,
        }
    }
    #[doc = "Disable the THRE interrupts."]
    #[inline(always)]
    pub fn is_disable_the_thre_int(&self) -> bool {
        *self == Uart1IerThreieEnum::DisableTheThreInt
    }
    #[doc = "Enable the THRE interrupts."]
    #[inline(always)]
    pub fn is_enable_the_thre_inte(&self) -> bool {
        *self == Uart1IerThreieEnum::EnableTheThreInte
    }
}
#[doc = "Field `THREIE` writer - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
pub type ThreieW<'a, REG> = crate::BitWriter<'a, REG, Uart1IerThreieEnum>;
impl<'a, REG> ThreieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the THRE interrupts."]
    #[inline(always)]
    pub fn disable_the_thre_int(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerThreieEnum::DisableTheThreInt)
    }
    #[doc = "Enable the THRE interrupts."]
    #[inline(always)]
    pub fn enable_the_thre_inte(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerThreieEnum::EnableTheThreInte)
    }
}
#[doc = "RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1IerRxieEnum {
    #[doc = "0: Disable the RX line status interrupts."]
    DisableTheRxLine_ = 0,
    #[doc = "1: Enable the RX line status interrupts."]
    EnableTheRxLineS = 1,
}
impl From<Uart1IerRxieEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1IerRxieEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RxieR = crate::BitReader<Uart1IerRxieEnum>;
impl RxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1IerRxieEnum {
        match self.bits {
            false => Uart1IerRxieEnum::DisableTheRxLine_,
            true => Uart1IerRxieEnum::EnableTheRxLineS,
        }
    }
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn is_disable_the_rx_line_(&self) -> bool {
        *self == Uart1IerRxieEnum::DisableTheRxLine_
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn is_enable_the_rx_line_s(&self) -> bool {
        *self == Uart1IerRxieEnum::EnableTheRxLineS
    }
}
#[doc = "Field `RXIE` writer - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG, Uart1IerRxieEnum>;
impl<'a, REG> RxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn disable_the_rx_line_(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerRxieEnum::DisableTheRxLine_)
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn enable_the_rx_line_s(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerRxieEnum::EnableTheRxLineS)
    }
}
#[doc = "Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1IerMsieEnum {
    #[doc = "0: Disable the modem interrupt."]
    DisableTheModemIn = 0,
    #[doc = "1: Enable the modem interrupt."]
    EnableTheModemInt = 1,
}
impl From<Uart1IerMsieEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1IerMsieEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIE` reader - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
pub type MsieR = crate::BitReader<Uart1IerMsieEnum>;
impl MsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1IerMsieEnum {
        match self.bits {
            false => Uart1IerMsieEnum::DisableTheModemIn,
            true => Uart1IerMsieEnum::EnableTheModemInt,
        }
    }
    #[doc = "Disable the modem interrupt."]
    #[inline(always)]
    pub fn is_disable_the_modem_in(&self) -> bool {
        *self == Uart1IerMsieEnum::DisableTheModemIn
    }
    #[doc = "Enable the modem interrupt."]
    #[inline(always)]
    pub fn is_enable_the_modem_int(&self) -> bool {
        *self == Uart1IerMsieEnum::EnableTheModemInt
    }
}
#[doc = "Field `MSIE` writer - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
pub type MsieW<'a, REG> = crate::BitWriter<'a, REG, Uart1IerMsieEnum>;
impl<'a, REG> MsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the modem interrupt."]
    #[inline(always)]
    pub fn disable_the_modem_in(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerMsieEnum::DisableTheModemIn)
    }
    #[doc = "Enable the modem interrupt."]
    #[inline(always)]
    pub fn enable_the_modem_int(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerMsieEnum::EnableTheModemInt)
    }
}
#[doc = "CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\] and IER\\[7\\] bits are set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1IerCtsieEnum {
    #[doc = "0: Disable the CTS interrupt."]
    DisableTheCtsInte = 0,
    #[doc = "1: Enable the CTS interrupt."]
    EnableTheCtsInter = 1,
}
impl From<Uart1IerCtsieEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1IerCtsieEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\] and IER\\[7\\] bits are set."]
pub type CtsieR = crate::BitReader<Uart1IerCtsieEnum>;
impl CtsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1IerCtsieEnum {
        match self.bits {
            false => Uart1IerCtsieEnum::DisableTheCtsInte,
            true => Uart1IerCtsieEnum::EnableTheCtsInter,
        }
    }
    #[doc = "Disable the CTS interrupt."]
    #[inline(always)]
    pub fn is_disable_the_cts_inte(&self) -> bool {
        *self == Uart1IerCtsieEnum::DisableTheCtsInte
    }
    #[doc = "Enable the CTS interrupt."]
    #[inline(always)]
    pub fn is_enable_the_cts_inter(&self) -> bool {
        *self == Uart1IerCtsieEnum::EnableTheCtsInter
    }
}
#[doc = "Field `CTSIE` writer - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\] and IER\\[7\\] bits are set."]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG, Uart1IerCtsieEnum>;
impl<'a, REG> CtsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the CTS interrupt."]
    #[inline(always)]
    pub fn disable_the_cts_inte(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerCtsieEnum::DisableTheCtsInte)
    }
    #[doc = "Enable the CTS interrupt."]
    #[inline(always)]
    pub fn enable_the_cts_inter(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerCtsieEnum::EnableTheCtsInter)
    }
}
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1IerAbeoieEnum {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    DisableEndOfAuto_ = 0,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    EnableEndOfAutoB = 1,
}
impl From<Uart1IerAbeoieEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1IerAbeoieEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABEOIE` reader - Enables the end of auto-baud interrupt."]
pub type AbeoieR = crate::BitReader<Uart1IerAbeoieEnum>;
impl AbeoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1IerAbeoieEnum {
        match self.bits {
            false => Uart1IerAbeoieEnum::DisableEndOfAuto_,
            true => Uart1IerAbeoieEnum::EnableEndOfAutoB,
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn is_disable_end_of_auto_(&self) -> bool {
        *self == Uart1IerAbeoieEnum::DisableEndOfAuto_
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn is_enable_end_of_auto_b(&self) -> bool {
        *self == Uart1IerAbeoieEnum::EnableEndOfAutoB
    }
}
#[doc = "Field `ABEOIE` writer - Enables the end of auto-baud interrupt."]
pub type AbeoieW<'a, REG> = crate::BitWriter<'a, REG, Uart1IerAbeoieEnum>;
impl<'a, REG> AbeoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disable_end_of_auto_(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerAbeoieEnum::DisableEndOfAuto_)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enable_end_of_auto_b(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerAbeoieEnum::EnableEndOfAutoB)
    }
}
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1IerAbtoieEnum {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    DisableAutoBaudTi = 0,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    EnableAutoBaudTim = 1,
}
impl From<Uart1IerAbtoieEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1IerAbtoieEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTOIE` reader - Enables the auto-baud time-out interrupt."]
pub type AbtoieR = crate::BitReader<Uart1IerAbtoieEnum>;
impl AbtoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1IerAbtoieEnum {
        match self.bits {
            false => Uart1IerAbtoieEnum::DisableAutoBaudTi,
            true => Uart1IerAbtoieEnum::EnableAutoBaudTim,
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn is_disable_auto_baud_ti(&self) -> bool {
        *self == Uart1IerAbtoieEnum::DisableAutoBaudTi
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn is_enable_auto_baud_tim(&self) -> bool {
        *self == Uart1IerAbtoieEnum::EnableAutoBaudTim
    }
}
#[doc = "Field `ABTOIE` writer - Enables the auto-baud time-out interrupt."]
pub type AbtoieW<'a, REG> = crate::BitWriter<'a, REG, Uart1IerAbtoieEnum>;
impl<'a, REG> AbtoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disable_auto_baud_ti(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerAbtoieEnum::DisableAutoBaudTi)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enable_auto_baud_tim(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1IerAbtoieEnum::EnableAutoBaudTim)
    }
}
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&self) -> RbrieR {
        RbrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&self) -> ThreieR {
        ThreieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
    #[inline(always)]
    pub fn msie(&self) -> MsieR {
        MsieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\] and IER\\[7\\] bits are set."]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeoie(&self) -> AbeoieR {
        AbeoieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtoie(&self) -> AbtoieR {
        AbtoieR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&mut self) -> RbrieW<'_, IerSpec> {
        RbrieW::new(self, 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&mut self) -> ThreieW<'_, IerSpec> {
        ThreieW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxie(&mut self) -> RxieW<'_, IerSpec> {
        RxieW::new(self, 2)
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
    #[inline(always)]
    pub fn msie(&mut self) -> MsieW<'_, IerSpec> {
        MsieW::new(self, 3)
    }
    #[doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\] and IER\\[7\\] bits are set."]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CtsieW<'_, IerSpec> {
        CtsieW::new(self, 7)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeoie(&mut self) -> AbeoieW<'_, IerSpec> {
        AbeoieW::new(self, 8)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtoie(&mut self) -> AbtoieW<'_, IerSpec> {
        AbtoieW::new(self, 9)
    }
}
#[doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
