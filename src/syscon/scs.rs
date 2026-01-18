#[doc = "Register `SCS` reader"]
pub type R = crate::R<ScsSpec>;
#[doc = "Register `SCS` writer"]
pub type W = crate::W<ScsSpec>;
#[doc = "Main oscillator range select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysconScsOscrangeEnum {
    #[doc = "0: Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    Low = 0,
    #[doc = "1: High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    High = 1,
}
impl From<SysconScsOscrangeEnum> for bool {
    #[inline(always)]
    fn from(variant: SysconScsOscrangeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCRANGE` reader - Main oscillator range select."]
pub type OscrangeR = crate::BitReader<SysconScsOscrangeEnum>;
impl OscrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconScsOscrangeEnum {
        match self.bits {
            false => SysconScsOscrangeEnum::Low,
            true => SysconScsOscrangeEnum::High,
        }
    }
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SysconScsOscrangeEnum::Low
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SysconScsOscrangeEnum::High
    }
}
#[doc = "Field `OSCRANGE` writer - Main oscillator range select."]
pub type OscrangeW<'a, REG> = crate::BitWriter<'a, REG, SysconScsOscrangeEnum>;
impl<'a, REG> OscrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SysconScsOscrangeEnum::Low)
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SysconScsOscrangeEnum::High)
    }
}
#[doc = "Main oscillator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysconScsOscenEnum {
    #[doc = "0: Disabled. The main oscillator is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    Enabled = 1,
}
impl From<SysconScsOscenEnum> for bool {
    #[inline(always)]
    fn from(variant: SysconScsOscenEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCEN` reader - Main oscillator enable."]
pub type OscenR = crate::BitReader<SysconScsOscenEnum>;
impl OscenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconScsOscenEnum {
        match self.bits {
            false => SysconScsOscenEnum::Disabled,
            true => SysconScsOscenEnum::Enabled,
        }
    }
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SysconScsOscenEnum::Disabled
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SysconScsOscenEnum::Enabled
    }
}
#[doc = "Field `OSCEN` writer - Main oscillator enable."]
pub type OscenW<'a, REG> = crate::BitWriter<'a, REG, SysconScsOscenEnum>;
impl<'a, REG> OscenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SysconScsOscenEnum::Disabled)
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SysconScsOscenEnum::Enabled)
    }
}
#[doc = "Main oscillator status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysconScsOscstatEnum {
    #[doc = "0: Not ready. The main oscillator is not ready to be used as a clock source."]
    NotReady = 0,
    #[doc = "1: Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    Ready = 1,
}
impl From<SysconScsOscstatEnum> for bool {
    #[inline(always)]
    fn from(variant: SysconScsOscstatEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSTAT` reader - Main oscillator status."]
pub type OscstatR = crate::BitReader<SysconScsOscstatEnum>;
impl OscstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconScsOscstatEnum {
        match self.bits {
            false => SysconScsOscstatEnum::NotReady,
            true => SysconScsOscstatEnum::Ready,
        }
    }
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == SysconScsOscstatEnum::NotReady
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SysconScsOscstatEnum::Ready
    }
}
#[doc = "Field `OSCSTAT` writer - Main oscillator status."]
pub type OscstatW<'a, REG> = crate::BitWriter<'a, REG, SysconScsOscstatEnum>;
impl<'a, REG> OscstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(SysconScsOscstatEnum::NotReady)
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(SysconScsOscstatEnum::Ready)
    }
}
impl R {
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrange(&self) -> OscrangeR {
        OscrangeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&self) -> OscenR {
        OscenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&self) -> OscstatR {
        OscstatR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrange(&mut self) -> OscrangeW<'_, ScsSpec> {
        OscrangeW::new(self, 4)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&mut self) -> OscenW<'_, ScsSpec> {
        OscenW::new(self, 5)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&mut self) -> OscstatW<'_, ScsSpec> {
        OscstatW::new(self, 6)
    }
}
#[doc = "System control and status\n\nYou can [`read`](crate::Reg::read) this register and get [`scs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsSpec;
impl crate::RegisterSpec for ScsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scs::R`](R) reader structure"]
impl crate::Readable for ScsSpec {}
#[doc = "`write(|w| ..)` method takes [`scs::W`](W) writer structure"]
impl crate::Writable for ScsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCS to value 0"]
impl crate::Resettable for ScsSpec {}
