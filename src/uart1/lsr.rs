#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Receiver Data Ready. LSR\\[0\\] is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LsrRdrEnum {
    #[doc = "0: The UART1 receiver FIFO is empty."]
    Empty = 0,
    #[doc = "1: The UART1 receiver FIFO is not empty."]
    Notempty = 1,
}
impl From<Uart1LsrRdrEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LsrRdrEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDR` reader - Receiver Data Ready. LSR\\[0\\] is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
pub type RdrR = crate::BitReader<Uart1LsrRdrEnum>;
impl RdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LsrRdrEnum {
        match self.bits {
            false => Uart1LsrRdrEnum::Empty,
            true => Uart1LsrRdrEnum::Notempty,
        }
    }
    #[doc = "The UART1 receiver FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Uart1LsrRdrEnum::Empty
    }
    #[doc = "The UART1 receiver FIFO is not empty."]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == Uart1LsrRdrEnum::Notempty
    }
}
#[doc = "Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\] is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LsrOeEnum {
    #[doc = "0: Overrun error status is inactive."]
    Inactive = 0,
    #[doc = "1: Overrun error status is active."]
    Active = 1,
}
impl From<Uart1LsrOeEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LsrOeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\] is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
pub type OeR = crate::BitReader<Uart1LsrOeEnum>;
impl OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LsrOeEnum {
        match self.bits {
            false => Uart1LsrOeEnum::Inactive,
            true => Uart1LsrOeEnum::Active,
        }
    }
    #[doc = "Overrun error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Uart1LsrOeEnum::Inactive
    }
    #[doc = "Overrun error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Uart1LsrOeEnum::Active
    }
}
#[doc = "Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LsrPeEnum {
    #[doc = "0: Parity error status is inactive."]
    Inactive = 0,
    #[doc = "1: Parity error status is active."]
    Active = 1,
}
impl From<Uart1LsrPeEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LsrPeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
pub type PeR = crate::BitReader<Uart1LsrPeEnum>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LsrPeEnum {
        match self.bits {
            false => Uart1LsrPeEnum::Inactive,
            true => Uart1LsrPeEnum::Active,
        }
    }
    #[doc = "Parity error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Uart1LsrPeEnum::Inactive
    }
    #[doc = "Parity error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Uart1LsrPeEnum::Active
    }
}
#[doc = "Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LsrFeEnum {
    #[doc = "0: Framing error status is inactive."]
    Inactive = 0,
    #[doc = "1: Framing error status is active."]
    Active = 1,
}
impl From<Uart1LsrFeEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LsrFeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
pub type FeR = crate::BitReader<Uart1LsrFeEnum>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LsrFeEnum {
        match self.bits {
            false => Uart1LsrFeEnum::Inactive,
            true => Uart1LsrFeEnum::Active,
        }
    }
    #[doc = "Framing error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Uart1LsrFeEnum::Inactive
    }
    #[doc = "Framing error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Uart1LsrFeEnum::Active
    }
}
#[doc = "Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LsrBiEnum {
    #[doc = "0: Break interrupt status is inactive."]
    Inactive = 0,
    #[doc = "1: Break interrupt status is active."]
    Active = 1,
}
impl From<Uart1LsrBiEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LsrBiEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BI` reader - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
pub type BiR = crate::BitReader<Uart1LsrBiEnum>;
impl BiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LsrBiEnum {
        match self.bits {
            false => Uart1LsrBiEnum::Inactive,
            true => Uart1LsrBiEnum::Active,
        }
    }
    #[doc = "Break interrupt status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Uart1LsrBiEnum::Inactive
    }
    #[doc = "Break interrupt status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Uart1LsrBiEnum::Active
    }
}
#[doc = "Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LsrThreEnum {
    #[doc = "0: THR contains valid data."]
    Valid = 0,
    #[doc = "1: THR is empty."]
    ThrIsEmpty_ = 1,
}
impl From<Uart1LsrThreEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LsrThreEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THRE` reader - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
pub type ThreR = crate::BitReader<Uart1LsrThreEnum>;
impl ThreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LsrThreEnum {
        match self.bits {
            false => Uart1LsrThreEnum::Valid,
            true => Uart1LsrThreEnum::ThrIsEmpty_,
        }
    }
    #[doc = "THR contains valid data."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Uart1LsrThreEnum::Valid
    }
    #[doc = "THR is empty."]
    #[inline(always)]
    pub fn is_thr_is_empty_(&self) -> bool {
        *self == Uart1LsrThreEnum::ThrIsEmpty_
    }
}
#[doc = "Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LsrTemtEnum {
    #[doc = "0: THR and/or the TSR contains valid data."]
    Valid = 0,
    #[doc = "1: THR and the TSR are empty."]
    Empty = 1,
}
impl From<Uart1LsrTemtEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LsrTemtEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMT` reader - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
pub type TemtR = crate::BitReader<Uart1LsrTemtEnum>;
impl TemtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LsrTemtEnum {
        match self.bits {
            false => Uart1LsrTemtEnum::Valid,
            true => Uart1LsrTemtEnum::Empty,
        }
    }
    #[doc = "THR and/or the TSR contains valid data."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Uart1LsrTemtEnum::Valid
    }
    #[doc = "THR and the TSR are empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Uart1LsrTemtEnum::Empty
    }
}
#[doc = "Error in RX FIFO. LSR\\[7\\] is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LsrRxfeEnum {
    #[doc = "0: RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    Noerror = 0,
    #[doc = "1: UART1 RBR contains at least one UART1 RX error."]
    Errors = 1,
}
impl From<Uart1LsrRxfeEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LsrRxfeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Error in RX FIFO. LSR\\[7\\] is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
pub type RxfeR = crate::BitReader<Uart1LsrRxfeEnum>;
impl RxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LsrRxfeEnum {
        match self.bits {
            false => Uart1LsrRxfeEnum::Noerror,
            true => Uart1LsrRxfeEnum::Errors,
        }
    }
    #[doc = "RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Uart1LsrRxfeEnum::Noerror
    }
    #[doc = "UART1 RBR contains at least one UART1 RX error."]
    #[inline(always)]
    pub fn is_errors(&self) -> bool {
        *self == Uart1LsrRxfeEnum::Errors
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Data Ready. LSR\\[0\\] is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\] is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO. LSR\\[7\\] is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0x60"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0x60;
}
