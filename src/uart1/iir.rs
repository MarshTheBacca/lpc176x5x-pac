#[doc = "Register `IIR` reader"]
pub type R = crate::R<IirSpec>;
#[doc = "Interrupt status. Note that IIR\\[0\\] is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1IirIntstatusEnum {
    #[doc = "0: At least one interrupt is pending."]
    AtLeastOneInterru = 0,
    #[doc = "1: No interrupt is pending."]
    NoInterruptIsPend = 1,
}
impl From<Uart1IirIntstatusEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1IirIntstatusEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSTATUS` reader - Interrupt status. Note that IIR\\[0\\] is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
pub type IntstatusR = crate::BitReader<Uart1IirIntstatusEnum>;
impl IntstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1IirIntstatusEnum {
        match self.bits {
            false => Uart1IirIntstatusEnum::AtLeastOneInterru,
            true => Uart1IirIntstatusEnum::NoInterruptIsPend,
        }
    }
    #[doc = "At least one interrupt is pending."]
    #[inline(always)]
    pub fn is_at_least_one_interru(&self) -> bool {
        *self == Uart1IirIntstatusEnum::AtLeastOneInterru
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_no_interrupt_is_pend(&self) -> bool {
        *self == Uart1IirIntstatusEnum::NoInterruptIsPend
    }
}
#[doc = "Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER\\[3:1\\] not listed below are reserved (100,101,111).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart1IirIntidEnum {
    #[doc = "3: 1 - Receive Line Status (RLS)."]
    Rls = 3,
    #[doc = "2: 2a - Receive Data Available (RDA)."]
    Rda = 2,
    #[doc = "6: 2b - Character Time-out Indicator (CTI)."]
    Cti = 6,
    #[doc = "1: 3 - THRE Interrupt."]
    Thre = 1,
    #[doc = "0: 4 - Modem Interrupt."]
    Modem = 0,
}
impl From<Uart1IirIntidEnum> for u8 {
    #[inline(always)]
    fn from(variant: Uart1IirIntidEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart1IirIntidEnum {
    type Ux = u8;
}
impl crate::IsEnum for Uart1IirIntidEnum {}
#[doc = "Field `INTID` reader - Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER\\[3:1\\] not listed below are reserved (100,101,111)."]
pub type IntidR = crate::FieldReader<Uart1IirIntidEnum>;
impl IntidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uart1IirIntidEnum> {
        match self.bits {
            3 => Some(Uart1IirIntidEnum::Rls),
            2 => Some(Uart1IirIntidEnum::Rda),
            6 => Some(Uart1IirIntidEnum::Cti),
            1 => Some(Uart1IirIntidEnum::Thre),
            0 => Some(Uart1IirIntidEnum::Modem),
            _ => None,
        }
    }
    #[doc = "1 - Receive Line Status (RLS)."]
    #[inline(always)]
    pub fn is_rls(&self) -> bool {
        *self == Uart1IirIntidEnum::Rls
    }
    #[doc = "2a - Receive Data Available (RDA)."]
    #[inline(always)]
    pub fn is_rda(&self) -> bool {
        *self == Uart1IirIntidEnum::Rda
    }
    #[doc = "2b - Character Time-out Indicator (CTI)."]
    #[inline(always)]
    pub fn is_cti(&self) -> bool {
        *self == Uart1IirIntidEnum::Cti
    }
    #[doc = "3 - THRE Interrupt."]
    #[inline(always)]
    pub fn is_thre(&self) -> bool {
        *self == Uart1IirIntidEnum::Thre
    }
    #[doc = "4 - Modem Interrupt."]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == Uart1IirIntidEnum::Modem
    }
}
#[doc = "Field `FIFOENABLE` reader - Copies of FCR\\[0\\]."]
pub type FifoenableR = crate::FieldReader;
#[doc = "Field `ABEOINT` reader - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
pub type AbeointR = crate::BitReader;
#[doc = "Field `ABTOINT` reader - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
pub type AbtointR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt status. Note that IIR\\[0\\] is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> IntstatusR {
        IntstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER\\[3:1\\] not listed below are reserved (100,101,111)."]
    #[inline(always)]
    pub fn intid(&self) -> IntidR {
        IntidR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Copies of FCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FifoenableR {
        FifoenableR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> AbeointR {
        AbeointR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> AbtointR {
        AbtointR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IirSpec {
    const RESET_VALUE: u32 = 0x01;
}
