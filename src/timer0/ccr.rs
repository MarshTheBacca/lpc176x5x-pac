#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Capture on CAPn.0 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0CcrCap0reEnum {
    #[doc = "1: A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Timer0CcrCap0reEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0CcrCap0reEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0RE` reader - Capture on CAPn.0 rising edge"]
pub type Cap0reR = crate::BitReader<Timer0CcrCap0reEnum>;
impl Cap0reR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0CcrCap0reEnum {
        match self.bits {
            true => Timer0CcrCap0reEnum::Enable,
            false => Timer0CcrCap0reEnum::Disable,
        }
    }
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer0CcrCap0reEnum::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer0CcrCap0reEnum::Disable
    }
}
#[doc = "Field `CAP0RE` writer - Capture on CAPn.0 rising edge"]
pub type Cap0reW<'a, REG> = crate::BitWriter<'a, REG, Timer0CcrCap0reEnum>;
impl<'a, REG> Cap0reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap0reEnum::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap0reEnum::Disable)
    }
}
#[doc = "Capture on CAPn.0 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0CcrCap0feEnum {
    #[doc = "1: A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Timer0CcrCap0feEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0CcrCap0feEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0FE` reader - Capture on CAPn.0 falling edge"]
pub type Cap0feR = crate::BitReader<Timer0CcrCap0feEnum>;
impl Cap0feR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0CcrCap0feEnum {
        match self.bits {
            true => Timer0CcrCap0feEnum::Enable,
            false => Timer0CcrCap0feEnum::Disable,
        }
    }
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer0CcrCap0feEnum::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer0CcrCap0feEnum::Disable
    }
}
#[doc = "Field `CAP0FE` writer - Capture on CAPn.0 falling edge"]
pub type Cap0feW<'a, REG> = crate::BitWriter<'a, REG, Timer0CcrCap0feEnum>;
impl<'a, REG> Cap0feW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap0feEnum::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap0feEnum::Disable)
    }
}
#[doc = "Interrupt on CAPn.0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0CcrCap0iEnum {
    #[doc = "1: A CR0 load due to a CAPn.0 event will generate an interrupt."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Timer0CcrCap0iEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0CcrCap0iEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0I` reader - Interrupt on CAPn.0 event"]
pub type Cap0iR = crate::BitReader<Timer0CcrCap0iEnum>;
impl Cap0iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0CcrCap0iEnum {
        match self.bits {
            true => Timer0CcrCap0iEnum::Enable,
            false => Timer0CcrCap0iEnum::Disable,
        }
    }
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer0CcrCap0iEnum::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer0CcrCap0iEnum::Disable
    }
}
#[doc = "Field `CAP0I` writer - Interrupt on CAPn.0 event"]
pub type Cap0iW<'a, REG> = crate::BitWriter<'a, REG, Timer0CcrCap0iEnum>;
impl<'a, REG> Cap0iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap0iEnum::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap0iEnum::Disable)
    }
}
#[doc = "Capture on CAPn.1 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0CcrCap1reEnum {
    #[doc = "1: A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Timer0CcrCap1reEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0CcrCap1reEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1RE` reader - Capture on CAPn.1 rising edge"]
pub type Cap1reR = crate::BitReader<Timer0CcrCap1reEnum>;
impl Cap1reR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0CcrCap1reEnum {
        match self.bits {
            true => Timer0CcrCap1reEnum::Enable,
            false => Timer0CcrCap1reEnum::Disable,
        }
    }
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer0CcrCap1reEnum::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer0CcrCap1reEnum::Disable
    }
}
#[doc = "Field `CAP1RE` writer - Capture on CAPn.1 rising edge"]
pub type Cap1reW<'a, REG> = crate::BitWriter<'a, REG, Timer0CcrCap1reEnum>;
impl<'a, REG> Cap1reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap1reEnum::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap1reEnum::Disable)
    }
}
#[doc = "Capture on CAPn.1 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0CcrCap1feEnum {
    #[doc = "1: A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Timer0CcrCap1feEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0CcrCap1feEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1FE` reader - Capture on CAPn.1 falling edge"]
pub type Cap1feR = crate::BitReader<Timer0CcrCap1feEnum>;
impl Cap1feR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0CcrCap1feEnum {
        match self.bits {
            true => Timer0CcrCap1feEnum::Enable,
            false => Timer0CcrCap1feEnum::Disable,
        }
    }
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer0CcrCap1feEnum::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer0CcrCap1feEnum::Disable
    }
}
#[doc = "Field `CAP1FE` writer - Capture on CAPn.1 falling edge"]
pub type Cap1feW<'a, REG> = crate::BitWriter<'a, REG, Timer0CcrCap1feEnum>;
impl<'a, REG> Cap1feW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap1feEnum::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap1feEnum::Disable)
    }
}
#[doc = "Interrupt on CAPn.1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0CcrCap1iEnum {
    #[doc = "1: A CR1 load due to a CAPn.1 event will generate an interrupt."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Timer0CcrCap1iEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0CcrCap1iEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1I` reader - Interrupt on CAPn.1 event"]
pub type Cap1iR = crate::BitReader<Timer0CcrCap1iEnum>;
impl Cap1iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0CcrCap1iEnum {
        match self.bits {
            true => Timer0CcrCap1iEnum::Enable,
            false => Timer0CcrCap1iEnum::Disable,
        }
    }
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer0CcrCap1iEnum::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer0CcrCap1iEnum::Disable
    }
}
#[doc = "Field `CAP1I` writer - Interrupt on CAPn.1 event"]
pub type Cap1iW<'a, REG> = crate::BitWriter<'a, REG, Timer0CcrCap1iEnum>;
impl<'a, REG> Cap1iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap1iEnum::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0CcrCap1iEnum::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline(always)]
    pub fn cap0re(&self) -> Cap0reR {
        Cap0reR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline(always)]
    pub fn cap0fe(&self) -> Cap0feR {
        Cap0feR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline(always)]
    pub fn cap0i(&self) -> Cap0iR {
        Cap0iR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline(always)]
    pub fn cap1re(&self) -> Cap1reR {
        Cap1reR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline(always)]
    pub fn cap1fe(&self) -> Cap1feR {
        Cap1feR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline(always)]
    pub fn cap1i(&self) -> Cap1iR {
        Cap1iR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline(always)]
    pub fn cap0re(&mut self) -> Cap0reW<'_, CcrSpec> {
        Cap0reW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline(always)]
    pub fn cap0fe(&mut self) -> Cap0feW<'_, CcrSpec> {
        Cap0feW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline(always)]
    pub fn cap0i(&mut self) -> Cap0iW<'_, CcrSpec> {
        Cap0iW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline(always)]
    pub fn cap1re(&mut self) -> Cap1reW<'_, CcrSpec> {
        Cap1reW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline(always)]
    pub fn cap1fe(&mut self) -> Cap1feW<'_, CcrSpec> {
        Cap1feW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline(always)]
    pub fn cap1i(&mut self) -> Cap1iW<'_, CcrSpec> {
        Cap1iW::new(self, 5)
    }
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {}
