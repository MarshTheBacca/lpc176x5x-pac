#[doc = "Register `I2C_STS` reader"]
pub type R = crate::R<I2cStsSpec>;
#[doc = "Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsTdiEnum {
    #[doc = "0: Transaction has not completed."]
    NotComplete = 0,
    #[doc = "1: Transaction completed."]
    Complete = 1,
}
impl From<UsbI2cStsTdiEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsTdiEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDI` reader - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
pub type TdiR = crate::BitReader<UsbI2cStsTdiEnum>;
impl TdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsTdiEnum {
        match self.bits {
            false => UsbI2cStsTdiEnum::NotComplete,
            true => UsbI2cStsTdiEnum::Complete,
        }
    }
    #[doc = "Transaction has not completed."]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == UsbI2cStsTdiEnum::NotComplete
    }
    #[doc = "Transaction completed."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == UsbI2cStsTdiEnum::Complete
    }
}
#[doc = "Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsAfiEnum {
    #[doc = "0: No arbitration failure on last transmission."]
    NoArbitrationFailu = 0,
    #[doc = "1: Arbitration failure occurred on last transmission."]
    ArbitrationFailure_ = 1,
}
impl From<UsbI2cStsAfiEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsAfiEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFI` reader - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
pub type AfiR = crate::BitReader<UsbI2cStsAfiEnum>;
impl AfiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsAfiEnum {
        match self.bits {
            false => UsbI2cStsAfiEnum::NoArbitrationFailu,
            true => UsbI2cStsAfiEnum::ArbitrationFailure_,
        }
    }
    #[doc = "No arbitration failure on last transmission."]
    #[inline(always)]
    pub fn is_no_arbitration_failu(&self) -> bool {
        *self == UsbI2cStsAfiEnum::NoArbitrationFailu
    }
    #[doc = "Arbitration failure occurred on last transmission."]
    #[inline(always)]
    pub fn is_arbitration_failure_(&self) -> bool {
        *self == UsbI2cStsAfiEnum::ArbitrationFailure_
    }
}
#[doc = "No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsNaiEnum {
    #[doc = "0: Last transmission received an acknowledge."]
    AcknowledgeRcvd = 0,
    #[doc = "1: Last transmission did not receive an acknowledge."]
    NoAcknowledgeRcvd = 1,
}
impl From<UsbI2cStsNaiEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsNaiEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAI` reader - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
pub type NaiR = crate::BitReader<UsbI2cStsNaiEnum>;
impl NaiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsNaiEnum {
        match self.bits {
            false => UsbI2cStsNaiEnum::AcknowledgeRcvd,
            true => UsbI2cStsNaiEnum::NoAcknowledgeRcvd,
        }
    }
    #[doc = "Last transmission received an acknowledge."]
    #[inline(always)]
    pub fn is_acknowledge_rcvd(&self) -> bool {
        *self == UsbI2cStsNaiEnum::AcknowledgeRcvd
    }
    #[doc = "Last transmission did not receive an acknowledge."]
    #[inline(always)]
    pub fn is_no_acknowledge_rcvd(&self) -> bool {
        *self == UsbI2cStsNaiEnum::NoAcknowledgeRcvd
    }
}
#[doc = "Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsDrmiEnum {
    #[doc = "0: Master transmitter does not need data."]
    Busy = 0,
    #[doc = "1: Master transmitter needs data."]
    NeedData = 1,
}
impl From<UsbI2cStsDrmiEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsDrmiEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRMI` reader - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
pub type DrmiR = crate::BitReader<UsbI2cStsDrmiEnum>;
impl DrmiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsDrmiEnum {
        match self.bits {
            false => UsbI2cStsDrmiEnum::Busy,
            true => UsbI2cStsDrmiEnum::NeedData,
        }
    }
    #[doc = "Master transmitter does not need data."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == UsbI2cStsDrmiEnum::Busy
    }
    #[doc = "Master transmitter needs data."]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == UsbI2cStsDrmiEnum::NeedData
    }
}
#[doc = "Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsDrsiEnum {
    #[doc = "0: Slave transmitter does not need data."]
    Busy = 0,
    #[doc = "1: Slave transmitter needs data."]
    NeedData = 1,
}
impl From<UsbI2cStsDrsiEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsDrsiEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRSI` reader - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
pub type DrsiR = crate::BitReader<UsbI2cStsDrsiEnum>;
impl DrsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsDrsiEnum {
        match self.bits {
            false => UsbI2cStsDrsiEnum::Busy,
            true => UsbI2cStsDrsiEnum::NeedData,
        }
    }
    #[doc = "Slave transmitter does not need data."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == UsbI2cStsDrsiEnum::Busy
    }
    #[doc = "Slave transmitter needs data."]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == UsbI2cStsDrsiEnum::NeedData
    }
}
#[doc = "Field `Active` reader - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
pub type ActiveR = crate::BitReader;
#[doc = "Field `SCL` reader - The current value of the SCL signal."]
pub type SclR = crate::BitReader;
#[doc = "Field `SDA` reader - The current value of the SDA signal."]
pub type SdaR = crate::BitReader;
#[doc = "Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsRffEnum {
    #[doc = "0: RX FIFO is not full"]
    RxFifoIsNotFull = 0,
    #[doc = "1: RX FIFO is full"]
    RxFifoIsFull = 1,
}
impl From<UsbI2cStsRffEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsRffEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
pub type RffR = crate::BitReader<UsbI2cStsRffEnum>;
impl RffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsRffEnum {
        match self.bits {
            false => UsbI2cStsRffEnum::RxFifoIsNotFull,
            true => UsbI2cStsRffEnum::RxFifoIsFull,
        }
    }
    #[doc = "RX FIFO is not full"]
    #[inline(always)]
    pub fn is_rx_fifo_is_not_full(&self) -> bool {
        *self == UsbI2cStsRffEnum::RxFifoIsNotFull
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn is_rx_fifo_is_full(&self) -> bool {
        *self == UsbI2cStsRffEnum::RxFifoIsFull
    }
}
#[doc = "Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsRfeEnum {
    #[doc = "0: RX FIFO contains data."]
    Data = 0,
    #[doc = "1: RX FIFO is empty"]
    Empty = 1,
}
impl From<UsbI2cStsRfeEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsRfeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
pub type RfeR = crate::BitReader<UsbI2cStsRfeEnum>;
impl RfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsRfeEnum {
        match self.bits {
            false => UsbI2cStsRfeEnum::Data,
            true => UsbI2cStsRfeEnum::Empty,
        }
    }
    #[doc = "RX FIFO contains data."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == UsbI2cStsRfeEnum::Data
    }
    #[doc = "RX FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == UsbI2cStsRfeEnum::Empty
    }
}
#[doc = "Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsTffEnum {
    #[doc = "0: TX FIFO is not full."]
    TxFifoIsNotFull_ = 0,
    #[doc = "1: TX FIFO is full"]
    TxFifoIsFull = 1,
}
impl From<UsbI2cStsTffEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsTffEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFF` reader - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
pub type TffR = crate::BitReader<UsbI2cStsTffEnum>;
impl TffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsTffEnum {
        match self.bits {
            false => UsbI2cStsTffEnum::TxFifoIsNotFull_,
            true => UsbI2cStsTffEnum::TxFifoIsFull,
        }
    }
    #[doc = "TX FIFO is not full."]
    #[inline(always)]
    pub fn is_tx_fifo_is_not_full_(&self) -> bool {
        *self == UsbI2cStsTffEnum::TxFifoIsNotFull_
    }
    #[doc = "TX FIFO is full"]
    #[inline(always)]
    pub fn is_tx_fifo_is_full(&self) -> bool {
        *self == UsbI2cStsTffEnum::TxFifoIsFull
    }
}
#[doc = "Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbI2cStsTfeEnum {
    #[doc = "0: TX FIFO contains valid data."]
    ValidData = 0,
    #[doc = "1: TX FIFO is empty"]
    Empty = 1,
}
impl From<UsbI2cStsTfeEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbI2cStsTfeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
pub type TfeR = crate::BitReader<UsbI2cStsTfeEnum>;
impl TfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbI2cStsTfeEnum {
        match self.bits {
            false => UsbI2cStsTfeEnum::ValidData,
            true => UsbI2cStsTfeEnum::Empty,
        }
    }
    #[doc = "TX FIFO contains valid data."]
    #[inline(always)]
    pub fn is_valid_data(&self) -> bool {
        *self == UsbI2cStsTfeEnum::ValidData
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == UsbI2cStsTfeEnum::Empty
    }
}
impl R {
    #[doc = "Bit 0 - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
    #[inline(always)]
    pub fn tdi(&self) -> TdiR {
        TdiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
    #[inline(always)]
    pub fn afi(&self) -> AfiR {
        AfiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn nai(&self) -> NaiR {
        NaiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn drmi(&self) -> DrmiR {
        DrmiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
    #[inline(always)]
    pub fn drsi(&self) -> DrsiR {
        DrsiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The current value of the SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The current value of the SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
    #[inline(always)]
    pub fn tff(&self) -> TffR {
        TffR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "I2C Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cStsSpec;
impl crate::RegisterSpec for I2cStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_sts::R`](R) reader structure"]
impl crate::Readable for I2cStsSpec {}
#[doc = "`reset()` method sets I2C_STS to value 0x0a00"]
impl crate::Resettable for I2cStsSpec {
    const RESET_VALUE: u32 = 0x0a00;
}
