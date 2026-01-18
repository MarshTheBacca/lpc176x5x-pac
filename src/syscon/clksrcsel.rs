#[doc = "Register `CLKSRCSEL` reader"]
pub type R = crate::R<ClksrcselSpec>;
#[doc = "Register `CLKSRCSEL` writer"]
pub type W = crate::W<ClksrcselSpec>;
#[doc = "Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconClksrcselClksrcEnum {
    #[doc = "0: Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    SelectsTheInternal = 0,
    #[doc = "1: Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    SelectsTheMainOsc = 1,
    #[doc = "2: Selects the RTC oscillator as the PLL0 clock source."]
    SelectsTheRtcOsci = 2,
}
impl From<SysconClksrcselClksrcEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconClksrcselClksrcEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconClksrcselClksrcEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconClksrcselClksrcEnum {}
#[doc = "Field `CLKSRC` reader - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
pub type ClksrcR = crate::FieldReader<SysconClksrcselClksrcEnum>;
impl ClksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconClksrcselClksrcEnum {
        match self.bits {
            0 => SysconClksrcselClksrcEnum::SelectsTheInternal,
            1 => SysconClksrcselClksrcEnum::SelectsTheMainOsc,
            2 => SysconClksrcselClksrcEnum::SelectsTheRtcOsci,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == SysconClksrcselClksrcEnum::SelectsTheInternal
    }
    #[doc = "Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == SysconClksrcselClksrcEnum::SelectsTheMainOsc
    }
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    #[inline(always)]
    pub fn is_selects_the_rtc_osci(&self) -> bool {
        *self == SysconClksrcselClksrcEnum::SelectsTheRtcOsci
    }
}
#[doc = "Field `CLKSRC` writer - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
pub type ClksrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconClksrcselClksrcEnum>;
impl<'a, REG> ClksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut crate::W<REG> {
        self.variant(SysconClksrcselClksrcEnum::SelectsTheInternal)
    }
    #[doc = "Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut crate::W<REG> {
        self.variant(SysconClksrcselClksrcEnum::SelectsTheMainOsc)
    }
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    #[inline(always)]
    pub fn selects_the_rtc_osci(self) -> &'a mut crate::W<REG> {
        self.variant(SysconClksrcselClksrcEnum::SelectsTheRtcOsci)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline(always)]
    pub fn clksrc(&self) -> ClksrcR {
        ClksrcR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline(always)]
    pub fn clksrc(&mut self) -> ClksrcW<'_, ClksrcselSpec> {
        ClksrcW::new(self, 0)
    }
}
#[doc = "Clock Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clksrcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksrcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClksrcselSpec;
impl crate::RegisterSpec for ClksrcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksrcsel::R`](R) reader structure"]
impl crate::Readable for ClksrcselSpec {}
#[doc = "`write(|w| ..)` method takes [`clksrcsel::W`](W) writer structure"]
impl crate::Writable for ClksrcselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKSRCSEL to value 0"]
impl crate::Resettable for ClksrcselSpec {}
