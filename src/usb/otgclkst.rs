#[doc = "Register `OTGCLKST` reader"]
pub type R = crate::R<OtgclkstSpec>;
#[doc = "Host clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbOtgclkstHostClkOnEnum {
    #[doc = "0: Host clock is not available."]
    HostClockIsNotAv = 0,
    #[doc = "1: Host clock is available."]
    HostClockIsAvaila = 1,
}
impl From<UsbOtgclkstHostClkOnEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbOtgclkstHostClkOnEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_CLK_ON` reader - Host clock status."]
pub type HostClkOnR = crate::BitReader<UsbOtgclkstHostClkOnEnum>;
impl HostClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbOtgclkstHostClkOnEnum {
        match self.bits {
            false => UsbOtgclkstHostClkOnEnum::HostClockIsNotAv,
            true => UsbOtgclkstHostClkOnEnum::HostClockIsAvaila,
        }
    }
    #[doc = "Host clock is not available."]
    #[inline(always)]
    pub fn is_host_clock_is_not_av(&self) -> bool {
        *self == UsbOtgclkstHostClkOnEnum::HostClockIsNotAv
    }
    #[doc = "Host clock is available."]
    #[inline(always)]
    pub fn is_host_clock_is_availa(&self) -> bool {
        *self == UsbOtgclkstHostClkOnEnum::HostClockIsAvaila
    }
}
#[doc = "Device clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbOtgclkstDevClkOnEnum {
    #[doc = "0: Device clock is not available."]
    DeviceClockIsNot_ = 0,
    #[doc = "1: Device clock is available."]
    DeviceClockIsAvai = 1,
}
impl From<UsbOtgclkstDevClkOnEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbOtgclkstDevClkOnEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_CLK_ON` reader - Device clock status."]
pub type DevClkOnR = crate::BitReader<UsbOtgclkstDevClkOnEnum>;
impl DevClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbOtgclkstDevClkOnEnum {
        match self.bits {
            false => UsbOtgclkstDevClkOnEnum::DeviceClockIsNot_,
            true => UsbOtgclkstDevClkOnEnum::DeviceClockIsAvai,
        }
    }
    #[doc = "Device clock is not available."]
    #[inline(always)]
    pub fn is_device_clock_is_not_(&self) -> bool {
        *self == UsbOtgclkstDevClkOnEnum::DeviceClockIsNot_
    }
    #[doc = "Device clock is available."]
    #[inline(always)]
    pub fn is_device_clock_is_avai(&self) -> bool {
        *self == UsbOtgclkstDevClkOnEnum::DeviceClockIsAvai
    }
}
#[doc = "I2C clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbOtgclkstI2cClkOnEnum {
    #[doc = "0: I2C clock is not available."]
    I2cClockIsNotAva = 0,
    #[doc = "1: I2C clock is available."]
    I2cClockIsAvailab = 1,
}
impl From<UsbOtgclkstI2cClkOnEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbOtgclkstI2cClkOnEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_CLK_ON` reader - I2C clock status."]
pub type I2cClkOnR = crate::BitReader<UsbOtgclkstI2cClkOnEnum>;
impl I2cClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbOtgclkstI2cClkOnEnum {
        match self.bits {
            false => UsbOtgclkstI2cClkOnEnum::I2cClockIsNotAva,
            true => UsbOtgclkstI2cClkOnEnum::I2cClockIsAvailab,
        }
    }
    #[doc = "I2C clock is not available."]
    #[inline(always)]
    pub fn is_i2c_clock_is_not_ava(&self) -> bool {
        *self == UsbOtgclkstI2cClkOnEnum::I2cClockIsNotAva
    }
    #[doc = "I2C clock is available."]
    #[inline(always)]
    pub fn is_i2c_clock_is_availab(&self) -> bool {
        *self == UsbOtgclkstI2cClkOnEnum::I2cClockIsAvailab
    }
}
#[doc = "OTG clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbOtgclkstOtgClkOnEnum {
    #[doc = "0: OTG clock is not available."]
    OtgClockIsNotAva = 0,
    #[doc = "1: OTG clock is available."]
    OtgClockIsAvailab = 1,
}
impl From<UsbOtgclkstOtgClkOnEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbOtgclkstOtgClkOnEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTG_CLK_ON` reader - OTG clock status."]
pub type OtgClkOnR = crate::BitReader<UsbOtgclkstOtgClkOnEnum>;
impl OtgClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbOtgclkstOtgClkOnEnum {
        match self.bits {
            false => UsbOtgclkstOtgClkOnEnum::OtgClockIsNotAva,
            true => UsbOtgclkstOtgClkOnEnum::OtgClockIsAvailab,
        }
    }
    #[doc = "OTG clock is not available."]
    #[inline(always)]
    pub fn is_otg_clock_is_not_ava(&self) -> bool {
        *self == UsbOtgclkstOtgClkOnEnum::OtgClockIsNotAva
    }
    #[doc = "OTG clock is available."]
    #[inline(always)]
    pub fn is_otg_clock_is_availab(&self) -> bool {
        *self == UsbOtgclkstOtgClkOnEnum::OtgClockIsAvailab
    }
}
#[doc = "AHB master clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbOtgclkstAhbClkOnEnum {
    #[doc = "0: AHB clock is not available."]
    AhbClockIsNotAva = 0,
    #[doc = "1: AHB clock is available."]
    AhbClockIsAvailab = 1,
}
impl From<UsbOtgclkstAhbClkOnEnum> for bool {
    #[inline(always)]
    fn from(variant: UsbOtgclkstAhbClkOnEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_CLK_ON` reader - AHB master clock status."]
pub type AhbClkOnR = crate::BitReader<UsbOtgclkstAhbClkOnEnum>;
impl AhbClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbOtgclkstAhbClkOnEnum {
        match self.bits {
            false => UsbOtgclkstAhbClkOnEnum::AhbClockIsNotAva,
            true => UsbOtgclkstAhbClkOnEnum::AhbClockIsAvailab,
        }
    }
    #[doc = "AHB clock is not available."]
    #[inline(always)]
    pub fn is_ahb_clock_is_not_ava(&self) -> bool {
        *self == UsbOtgclkstAhbClkOnEnum::AhbClockIsNotAva
    }
    #[doc = "AHB clock is available."]
    #[inline(always)]
    pub fn is_ahb_clock_is_availab(&self) -> bool {
        *self == UsbOtgclkstAhbClkOnEnum::AhbClockIsAvailab
    }
}
impl R {
    #[doc = "Bit 0 - Host clock status."]
    #[inline(always)]
    pub fn host_clk_on(&self) -> HostClkOnR {
        HostClkOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device clock status."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DevClkOnR {
        DevClkOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C clock status."]
    #[inline(always)]
    pub fn i2c_clk_on(&self) -> I2cClkOnR {
        I2cClkOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OTG clock status."]
    #[inline(always)]
    pub fn otg_clk_on(&self) -> OtgClkOnR {
        OtgClkOnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master clock status."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AhbClkOnR {
        AhbClkOnR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "OTG clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`otgclkst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgclkstSpec;
impl crate::RegisterSpec for OtgclkstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otgclkst::R`](R) reader structure"]
impl crate::Readable for OtgclkstSpec {}
#[doc = "`reset()` method sets OTGCLKST to value 0"]
impl crate::Resettable for OtgclkstSpec {}
