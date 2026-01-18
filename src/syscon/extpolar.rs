#[doc = "Register `EXTPOLAR` reader"]
pub type R = crate::R<ExtpolarSpec>;
#[doc = "Register `EXTPOLAR` writer"]
pub type W = crate::W<ExtpolarSpec>;
#[doc = "External interrupt 0 EINT0 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysconExtpolarExtpolar0Enum {
    #[doc = "0: Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    RisingEdge = 1,
}
impl From<SysconExtpolarExtpolar0Enum> for bool {
    #[inline(always)]
    fn from(variant: SysconExtpolarExtpolar0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR0` reader - External interrupt 0 EINT0 polarity."]
pub type Extpolar0R = crate::BitReader<SysconExtpolarExtpolar0Enum>;
impl Extpolar0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconExtpolarExtpolar0Enum {
        match self.bits {
            false => SysconExtpolarExtpolar0Enum::FallingEdge,
            true => SysconExtpolarExtpolar0Enum::RisingEdge,
        }
    }
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SysconExtpolarExtpolar0Enum::FallingEdge
    }
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SysconExtpolarExtpolar0Enum::RisingEdge
    }
}
#[doc = "Field `EXTPOLAR0` writer - External interrupt 0 EINT0 polarity."]
pub type Extpolar0W<'a, REG> = crate::BitWriter<'a, REG, SysconExtpolarExtpolar0Enum>;
impl<'a, REG> Extpolar0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SysconExtpolarExtpolar0Enum::FallingEdge)
    }
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SysconExtpolarExtpolar0Enum::RisingEdge)
    }
}
#[doc = "External interrupt 1 EINT1 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysconExtpolarExtpolar1Enum {
    #[doc = "0: Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    RisingEdge = 1,
}
impl From<SysconExtpolarExtpolar1Enum> for bool {
    #[inline(always)]
    fn from(variant: SysconExtpolarExtpolar1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR1` reader - External interrupt 1 EINT1 polarity."]
pub type Extpolar1R = crate::BitReader<SysconExtpolarExtpolar1Enum>;
impl Extpolar1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconExtpolarExtpolar1Enum {
        match self.bits {
            false => SysconExtpolarExtpolar1Enum::FallingEdge,
            true => SysconExtpolarExtpolar1Enum::RisingEdge,
        }
    }
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SysconExtpolarExtpolar1Enum::FallingEdge
    }
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SysconExtpolarExtpolar1Enum::RisingEdge
    }
}
#[doc = "Field `EXTPOLAR1` writer - External interrupt 1 EINT1 polarity."]
pub type Extpolar1W<'a, REG> = crate::BitWriter<'a, REG, SysconExtpolarExtpolar1Enum>;
impl<'a, REG> Extpolar1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SysconExtpolarExtpolar1Enum::FallingEdge)
    }
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SysconExtpolarExtpolar1Enum::RisingEdge)
    }
}
#[doc = "External interrupt 2 EINT2 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysconExtpolarExtpolar2Enum {
    #[doc = "0: Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    RisingEdge = 1,
}
impl From<SysconExtpolarExtpolar2Enum> for bool {
    #[inline(always)]
    fn from(variant: SysconExtpolarExtpolar2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR2` reader - External interrupt 2 EINT2 polarity."]
pub type Extpolar2R = crate::BitReader<SysconExtpolarExtpolar2Enum>;
impl Extpolar2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconExtpolarExtpolar2Enum {
        match self.bits {
            false => SysconExtpolarExtpolar2Enum::FallingEdge,
            true => SysconExtpolarExtpolar2Enum::RisingEdge,
        }
    }
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SysconExtpolarExtpolar2Enum::FallingEdge
    }
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SysconExtpolarExtpolar2Enum::RisingEdge
    }
}
#[doc = "Field `EXTPOLAR2` writer - External interrupt 2 EINT2 polarity."]
pub type Extpolar2W<'a, REG> = crate::BitWriter<'a, REG, SysconExtpolarExtpolar2Enum>;
impl<'a, REG> Extpolar2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SysconExtpolarExtpolar2Enum::FallingEdge)
    }
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SysconExtpolarExtpolar2Enum::RisingEdge)
    }
}
#[doc = "External interrupt 3 EINT3 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysconExtpolarExtpolar3Enum {
    #[doc = "0: Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    RisingEdge = 1,
}
impl From<SysconExtpolarExtpolar3Enum> for bool {
    #[inline(always)]
    fn from(variant: SysconExtpolarExtpolar3Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR3` reader - External interrupt 3 EINT3 polarity."]
pub type Extpolar3R = crate::BitReader<SysconExtpolarExtpolar3Enum>;
impl Extpolar3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconExtpolarExtpolar3Enum {
        match self.bits {
            false => SysconExtpolarExtpolar3Enum::FallingEdge,
            true => SysconExtpolarExtpolar3Enum::RisingEdge,
        }
    }
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SysconExtpolarExtpolar3Enum::FallingEdge
    }
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SysconExtpolarExtpolar3Enum::RisingEdge
    }
}
#[doc = "Field `EXTPOLAR3` writer - External interrupt 3 EINT3 polarity."]
pub type Extpolar3W<'a, REG> = crate::BitWriter<'a, REG, SysconExtpolarExtpolar3Enum>;
impl<'a, REG> Extpolar3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SysconExtpolarExtpolar3Enum::FallingEdge)
    }
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SysconExtpolarExtpolar3Enum::RisingEdge)
    }
}
impl R {
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    pub fn extpolar0(&self) -> Extpolar0R {
        Extpolar0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    pub fn extpolar1(&self) -> Extpolar1R {
        Extpolar1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    pub fn extpolar2(&self) -> Extpolar2R {
        Extpolar2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    pub fn extpolar3(&self) -> Extpolar3R {
        Extpolar3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    pub fn extpolar0(&mut self) -> Extpolar0W<'_, ExtpolarSpec> {
        Extpolar0W::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    pub fn extpolar1(&mut self) -> Extpolar1W<'_, ExtpolarSpec> {
        Extpolar1W::new(self, 1)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    pub fn extpolar2(&mut self) -> Extpolar2W<'_, ExtpolarSpec> {
        Extpolar2W::new(self, 2)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    pub fn extpolar3(&mut self) -> Extpolar3W<'_, ExtpolarSpec> {
        Extpolar3W::new(self, 3)
    }
}
#[doc = "External Interrupt Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extpolar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extpolar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtpolarSpec;
impl crate::RegisterSpec for ExtpolarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extpolar::R`](R) reader structure"]
impl crate::Readable for ExtpolarSpec {}
#[doc = "`write(|w| ..)` method takes [`extpolar::W`](W) writer structure"]
impl crate::Writable for ExtpolarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTPOLAR to value 0"]
impl crate::Resettable for ExtpolarSpec {}
