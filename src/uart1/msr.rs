#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Delta CTS. Set upon state change of input CTS. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1MsrDctsEnum {
    #[doc = "0: No change detected on modem input, CTS."]
    NoChangeDetectedO = 0,
    #[doc = "1: State change detected on modem input, CTS."]
    StateChangeDetecte = 1,
}
impl From<Uart1MsrDctsEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1MsrDctsEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCTS` reader - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
pub type DctsR = crate::BitReader<Uart1MsrDctsEnum>;
impl DctsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1MsrDctsEnum {
        match self.bits {
            false => Uart1MsrDctsEnum::NoChangeDetectedO,
            true => Uart1MsrDctsEnum::StateChangeDetecte,
        }
    }
    #[doc = "No change detected on modem input, CTS."]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == Uart1MsrDctsEnum::NoChangeDetectedO
    }
    #[doc = "State change detected on modem input, CTS."]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == Uart1MsrDctsEnum::StateChangeDetecte
    }
}
#[doc = "Delta DSR. Set upon state change of input DSR. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1MsrDdsrEnum {
    #[doc = "0: No change detected on modem input, DSR."]
    NoChangeDetectedO = 0,
    #[doc = "1: State change detected on modem input, DSR."]
    StateChangeDetecte = 1,
}
impl From<Uart1MsrDdsrEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1MsrDdsrEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDSR` reader - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
pub type DdsrR = crate::BitReader<Uart1MsrDdsrEnum>;
impl DdsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1MsrDdsrEnum {
        match self.bits {
            false => Uart1MsrDdsrEnum::NoChangeDetectedO,
            true => Uart1MsrDdsrEnum::StateChangeDetecte,
        }
    }
    #[doc = "No change detected on modem input, DSR."]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == Uart1MsrDdsrEnum::NoChangeDetectedO
    }
    #[doc = "State change detected on modem input, DSR."]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == Uart1MsrDdsrEnum::StateChangeDetecte
    }
}
#[doc = "Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1MsrTeriEnum {
    #[doc = "0: No change detected on modem input, RI."]
    NoChangeDetectedO = 0,
    #[doc = "1: Low-to-high transition detected on RI."]
    LowToHighTransiti = 1,
}
impl From<Uart1MsrTeriEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1MsrTeriEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERI` reader - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
pub type TeriR = crate::BitReader<Uart1MsrTeriEnum>;
impl TeriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1MsrTeriEnum {
        match self.bits {
            false => Uart1MsrTeriEnum::NoChangeDetectedO,
            true => Uart1MsrTeriEnum::LowToHighTransiti,
        }
    }
    #[doc = "No change detected on modem input, RI."]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == Uart1MsrTeriEnum::NoChangeDetectedO
    }
    #[doc = "Low-to-high transition detected on RI."]
    #[inline(always)]
    pub fn is_low_to_high_transiti(&self) -> bool {
        *self == Uart1MsrTeriEnum::LowToHighTransiti
    }
}
#[doc = "Delta DCD. Set upon state change of input DCD. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1MsrDdcdEnum {
    #[doc = "0: No change detected on modem input, DCD."]
    NoChangeDetectedO = 0,
    #[doc = "1: State change detected on modem input, DCD."]
    StateChangeDetecte = 1,
}
impl From<Uart1MsrDdcdEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1MsrDdcdEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDCD` reader - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
pub type DdcdR = crate::BitReader<Uart1MsrDdcdEnum>;
impl DdcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1MsrDdcdEnum {
        match self.bits {
            false => Uart1MsrDdcdEnum::NoChangeDetectedO,
            true => Uart1MsrDdcdEnum::StateChangeDetecte,
        }
    }
    #[doc = "No change detected on modem input, DCD."]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == Uart1MsrDdcdEnum::NoChangeDetectedO
    }
    #[doc = "State change detected on modem input, DCD."]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == Uart1MsrDdcdEnum::StateChangeDetecte
    }
}
#[doc = "Field `CTS` reader - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\] in modem loopback mode."]
pub type CtsR = crate::BitReader;
#[doc = "Field `DSR` reader - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\] in modem loopback mode."]
pub type DsrR = crate::BitReader;
#[doc = "Field `RI` reader - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\] in modem loopback mode."]
pub type RiR = crate::BitReader;
#[doc = "Field `DCD` reader - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\] in modem loopback mode."]
pub type DcdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
    #[inline(always)]
    pub fn dcts(&self) -> DctsR {
        DctsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddsr(&self) -> DdsrR {
        DdsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
    #[inline(always)]
    pub fn teri(&self) -> TeriR {
        TeriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddcd(&self) -> DdcdR {
        DdcdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\] in modem loopback mode."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\] in modem loopback mode."]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\] in modem loopback mode."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\] in modem loopback mode."]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Modem Status Register. Contains handshake signal status flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {}
