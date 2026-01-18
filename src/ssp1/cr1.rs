#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Loop Back Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssp1Cr1LbmEnum {
    #[doc = "0: During normal operation."]
    Normal = 0,
    #[doc = "1: Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    Ouptu = 1,
}
impl From<Ssp1Cr1LbmEnum> for bool {
    #[inline(always)]
    fn from(variant: Ssp1Cr1LbmEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - Loop Back Mode."]
pub type LbmR = crate::BitReader<Ssp1Cr1LbmEnum>;
impl LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssp1Cr1LbmEnum {
        match self.bits {
            false => Ssp1Cr1LbmEnum::Normal,
            true => Ssp1Cr1LbmEnum::Ouptu,
        }
    }
    #[doc = "During normal operation."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Ssp1Cr1LbmEnum::Normal
    }
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    #[inline(always)]
    pub fn is_ouptu(&self) -> bool {
        *self == Ssp1Cr1LbmEnum::Ouptu
    }
}
#[doc = "Field `LBM` writer - Loop Back Mode."]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG, Ssp1Cr1LbmEnum>;
impl<'a, REG> LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "During normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1Cr1LbmEnum::Normal)
    }
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    #[inline(always)]
    pub fn ouptu(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1Cr1LbmEnum::Ouptu)
    }
}
#[doc = "SSP Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssp1Cr1SseEnum {
    #[doc = "0: The SSP controller is disabled."]
    Disabled = 0,
    #[doc = "1: The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit."]
    Enabled = 1,
}
impl From<Ssp1Cr1SseEnum> for bool {
    #[inline(always)]
    fn from(variant: Ssp1Cr1SseEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSE` reader - SSP Enable."]
pub type SseR = crate::BitReader<Ssp1Cr1SseEnum>;
impl SseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssp1Cr1SseEnum {
        match self.bits {
            false => Ssp1Cr1SseEnum::Disabled,
            true => Ssp1Cr1SseEnum::Enabled,
        }
    }
    #[doc = "The SSP controller is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ssp1Cr1SseEnum::Disabled
    }
    #[doc = "The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ssp1Cr1SseEnum::Enabled
    }
}
#[doc = "Field `SSE` writer - SSP Enable."]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG, Ssp1Cr1SseEnum>;
impl<'a, REG> SseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SSP controller is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1Cr1SseEnum::Disabled)
    }
    #[doc = "The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1Cr1SseEnum::Enabled)
    }
}
#[doc = "Master/Slave Mode.This bit can only be written when the SSE bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssp1Cr1MsEnum {
    #[doc = "0: The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    Master = 0,
    #[doc = "1: The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    Slave = 1,
}
impl From<Ssp1Cr1MsEnum> for bool {
    #[inline(always)]
    fn from(variant: Ssp1Cr1MsEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
pub type MsR = crate::BitReader<Ssp1Cr1MsEnum>;
impl MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssp1Cr1MsEnum {
        match self.bits {
            false => Ssp1Cr1MsEnum::Master,
            true => Ssp1Cr1MsEnum::Slave,
        }
    }
    #[doc = "The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Ssp1Cr1MsEnum::Master
    }
    #[doc = "The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Ssp1Cr1MsEnum::Slave
    }
}
#[doc = "Field `MS` writer - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG, Ssp1Cr1MsEnum>;
impl<'a, REG> MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1Cr1MsEnum::Master)
    }
    #[doc = "The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Ssp1Cr1MsEnum::Slave)
    }
}
#[doc = "Field `SOD` reader - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
pub type SodR = crate::BitReader;
#[doc = "Field `SOD` writer - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSP Enable."]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<'_, Cr1Spec> {
        LbmW::new(self, 0)
    }
    #[doc = "Bit 1 - SSP Enable."]
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<'_, Cr1Spec> {
        SseW::new(self, 1)
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, Cr1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    pub fn sod(&mut self) -> SodW<'_, Cr1Spec> {
        SodW::new(self, 3)
    }
}
#[doc = "Control Register 1. Selects master/slave and other modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
