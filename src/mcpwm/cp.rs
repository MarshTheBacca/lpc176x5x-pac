#[doc = "Register `CP` reader"]
pub type R = crate::R<CpSpec>;
#[doc = "Register `CP` writer"]
pub type W = crate::W<CpSpec>;
#[doc = "Communication pattern output A, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmCpCcpa0Enum {
    #[doc = "0: MCOA0 passive."]
    Mcoa0Passive_ = 0,
    #[doc = "1: internal MCOA0."]
    InternalMcoa0_ = 1,
}
impl From<McpwmCpCcpa0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmCpCcpa0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPA0` reader - Communication pattern output A, channel 0."]
pub type Ccpa0R = crate::BitReader<McpwmCpCcpa0Enum>;
impl Ccpa0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmCpCcpa0Enum {
        match self.bits {
            false => McpwmCpCcpa0Enum::Mcoa0Passive_,
            true => McpwmCpCcpa0Enum::InternalMcoa0_,
        }
    }
    #[doc = "MCOA0 passive."]
    #[inline(always)]
    pub fn is_mcoa0_passive_(&self) -> bool {
        *self == McpwmCpCcpa0Enum::Mcoa0Passive_
    }
    #[doc = "internal MCOA0."]
    #[inline(always)]
    pub fn is_internal_mcoa0_(&self) -> bool {
        *self == McpwmCpCcpa0Enum::InternalMcoa0_
    }
}
#[doc = "Field `CCPA0` writer - Communication pattern output A, channel 0."]
pub type Ccpa0W<'a, REG> = crate::BitWriter<'a, REG, McpwmCpCcpa0Enum>;
impl<'a, REG> Ccpa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA0 passive."]
    #[inline(always)]
    pub fn mcoa0_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpa0Enum::Mcoa0Passive_)
    }
    #[doc = "internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0_(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpa0Enum::InternalMcoa0_)
    }
}
#[doc = "Communication pattern output B, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmCpCcpb0Enum {
    #[doc = "0: MCOB0 passive."]
    Mcob0Passive_ = 0,
    #[doc = "1: MCOB0 tracks internal MCOA0."]
    Mcob0TracksInterna = 1,
}
impl From<McpwmCpCcpb0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmCpCcpb0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPB0` reader - Communication pattern output B, channel 0."]
pub type Ccpb0R = crate::BitReader<McpwmCpCcpb0Enum>;
impl Ccpb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmCpCcpb0Enum {
        match self.bits {
            false => McpwmCpCcpb0Enum::Mcob0Passive_,
            true => McpwmCpCcpb0Enum::Mcob0TracksInterna,
        }
    }
    #[doc = "MCOB0 passive."]
    #[inline(always)]
    pub fn is_mcob0_passive_(&self) -> bool {
        *self == McpwmCpCcpb0Enum::Mcob0Passive_
    }
    #[doc = "MCOB0 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcob0_tracks_interna(&self) -> bool {
        *self == McpwmCpCcpb0Enum::Mcob0TracksInterna
    }
}
#[doc = "Field `CCPB0` writer - Communication pattern output B, channel 0."]
pub type Ccpb0W<'a, REG> = crate::BitWriter<'a, REG, McpwmCpCcpb0Enum>;
impl<'a, REG> Ccpb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB0 passive."]
    #[inline(always)]
    pub fn mcob0_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpb0Enum::Mcob0Passive_)
    }
    #[doc = "MCOB0 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob0_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpb0Enum::Mcob0TracksInterna)
    }
}
#[doc = "Communication pattern output A, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmCpCcpa1Enum {
    #[doc = "0: MCOA1 passive."]
    Mcoa1Passive_ = 0,
    #[doc = "1: MCOA1 tracks internal MCOA0."]
    Mcoa1TracksInterna = 1,
}
impl From<McpwmCpCcpa1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmCpCcpa1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPA1` reader - Communication pattern output A, channel 1."]
pub type Ccpa1R = crate::BitReader<McpwmCpCcpa1Enum>;
impl Ccpa1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmCpCcpa1Enum {
        match self.bits {
            false => McpwmCpCcpa1Enum::Mcoa1Passive_,
            true => McpwmCpCcpa1Enum::Mcoa1TracksInterna,
        }
    }
    #[doc = "MCOA1 passive."]
    #[inline(always)]
    pub fn is_mcoa1_passive_(&self) -> bool {
        *self == McpwmCpCcpa1Enum::Mcoa1Passive_
    }
    #[doc = "MCOA1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcoa1_tracks_interna(&self) -> bool {
        *self == McpwmCpCcpa1Enum::Mcoa1TracksInterna
    }
}
#[doc = "Field `CCPA1` writer - Communication pattern output A, channel 1."]
pub type Ccpa1W<'a, REG> = crate::BitWriter<'a, REG, McpwmCpCcpa1Enum>;
impl<'a, REG> Ccpa1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA1 passive."]
    #[inline(always)]
    pub fn mcoa1_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpa1Enum::Mcoa1Passive_)
    }
    #[doc = "MCOA1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcoa1_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpa1Enum::Mcoa1TracksInterna)
    }
}
#[doc = "Communication pattern output B, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmCpCcpb1Enum {
    #[doc = "0: MCOB1 passive."]
    Mcob1Passive_ = 0,
    #[doc = "1: MCOB1 tracks internal MCOA0."]
    Mcob1TracksInterna = 1,
}
impl From<McpwmCpCcpb1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmCpCcpb1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPB1` reader - Communication pattern output B, channel 1."]
pub type Ccpb1R = crate::BitReader<McpwmCpCcpb1Enum>;
impl Ccpb1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmCpCcpb1Enum {
        match self.bits {
            false => McpwmCpCcpb1Enum::Mcob1Passive_,
            true => McpwmCpCcpb1Enum::Mcob1TracksInterna,
        }
    }
    #[doc = "MCOB1 passive."]
    #[inline(always)]
    pub fn is_mcob1_passive_(&self) -> bool {
        *self == McpwmCpCcpb1Enum::Mcob1Passive_
    }
    #[doc = "MCOB1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcob1_tracks_interna(&self) -> bool {
        *self == McpwmCpCcpb1Enum::Mcob1TracksInterna
    }
}
#[doc = "Field `CCPB1` writer - Communication pattern output B, channel 1."]
pub type Ccpb1W<'a, REG> = crate::BitWriter<'a, REG, McpwmCpCcpb1Enum>;
impl<'a, REG> Ccpb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB1 passive."]
    #[inline(always)]
    pub fn mcob1_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpb1Enum::Mcob1Passive_)
    }
    #[doc = "MCOB1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob1_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpb1Enum::Mcob1TracksInterna)
    }
}
#[doc = "Communication pattern output A, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmCpCcpa2Enum {
    #[doc = "0: MCOA2 passive."]
    Mcoa2Passive_ = 0,
    #[doc = "1: MCOA2 tracks internal MCOA0."]
    Mcoa2TracksInterna = 1,
}
impl From<McpwmCpCcpa2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmCpCcpa2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPA2` reader - Communication pattern output A, channel 2."]
pub type Ccpa2R = crate::BitReader<McpwmCpCcpa2Enum>;
impl Ccpa2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmCpCcpa2Enum {
        match self.bits {
            false => McpwmCpCcpa2Enum::Mcoa2Passive_,
            true => McpwmCpCcpa2Enum::Mcoa2TracksInterna,
        }
    }
    #[doc = "MCOA2 passive."]
    #[inline(always)]
    pub fn is_mcoa2_passive_(&self) -> bool {
        *self == McpwmCpCcpa2Enum::Mcoa2Passive_
    }
    #[doc = "MCOA2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcoa2_tracks_interna(&self) -> bool {
        *self == McpwmCpCcpa2Enum::Mcoa2TracksInterna
    }
}
#[doc = "Field `CCPA2` writer - Communication pattern output A, channel 2."]
pub type Ccpa2W<'a, REG> = crate::BitWriter<'a, REG, McpwmCpCcpa2Enum>;
impl<'a, REG> Ccpa2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA2 passive."]
    #[inline(always)]
    pub fn mcoa2_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpa2Enum::Mcoa2Passive_)
    }
    #[doc = "MCOA2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcoa2_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpa2Enum::Mcoa2TracksInterna)
    }
}
#[doc = "Communication pattern output B, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmCpCcpb2Enum {
    #[doc = "0: MCOB2 passive."]
    Mcob2Passive_ = 0,
    #[doc = "1: MCOB2 tracks internal MCOA0."]
    Mcob2TracksInterna = 1,
}
impl From<McpwmCpCcpb2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmCpCcpb2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPB2` reader - Communication pattern output B, channel 2."]
pub type Ccpb2R = crate::BitReader<McpwmCpCcpb2Enum>;
impl Ccpb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmCpCcpb2Enum {
        match self.bits {
            false => McpwmCpCcpb2Enum::Mcob2Passive_,
            true => McpwmCpCcpb2Enum::Mcob2TracksInterna,
        }
    }
    #[doc = "MCOB2 passive."]
    #[inline(always)]
    pub fn is_mcob2_passive_(&self) -> bool {
        *self == McpwmCpCcpb2Enum::Mcob2Passive_
    }
    #[doc = "MCOB2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcob2_tracks_interna(&self) -> bool {
        *self == McpwmCpCcpb2Enum::Mcob2TracksInterna
    }
}
#[doc = "Field `CCPB2` writer - Communication pattern output B, channel 2."]
pub type Ccpb2W<'a, REG> = crate::BitWriter<'a, REG, McpwmCpCcpb2Enum>;
impl<'a, REG> Ccpb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB2 passive."]
    #[inline(always)]
    pub fn mcob2_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpb2Enum::Mcob2Passive_)
    }
    #[doc = "MCOB2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob2_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(McpwmCpCcpb2Enum::Mcob2TracksInterna)
    }
}
impl R {
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&self) -> Ccpa0R {
        Ccpa0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&self) -> Ccpb0R {
        Ccpb0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&self) -> Ccpa1R {
        Ccpa1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&self) -> Ccpb1R {
        Ccpb1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&self) -> Ccpa2R {
        Ccpa2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&self) -> Ccpb2R {
        Ccpb2R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&mut self) -> Ccpa0W<'_, CpSpec> {
        Ccpa0W::new(self, 0)
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&mut self) -> Ccpb0W<'_, CpSpec> {
        Ccpb0W::new(self, 1)
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&mut self) -> Ccpa1W<'_, CpSpec> {
        Ccpa1W::new(self, 2)
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&mut self) -> Ccpb1W<'_, CpSpec> {
        Ccpb1W::new(self, 3)
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&mut self) -> Ccpa2W<'_, CpSpec> {
        Ccpa2W::new(self, 4)
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&mut self) -> Ccpb2W<'_, CpSpec> {
        Ccpb2W::new(self, 5)
    }
}
#[doc = "Communication Pattern register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpSpec;
impl crate::RegisterSpec for CpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cp::R`](R) reader structure"]
impl crate::Readable for CpSpec {}
#[doc = "`write(|w| ..)` method takes [`cp::W`](W) writer structure"]
impl crate::Writable for CpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CP to value 0"]
impl crate::Resettable for CpSpec {}
