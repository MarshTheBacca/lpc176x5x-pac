#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Port 0 GPIO interrupt pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpiointStatusP0intEnum {
    #[doc = "0: No pending interrupts on Port 0."]
    NoPendingInterrupt = 0,
    #[doc = "1: At least one pending interrupt on Port 0."]
    AtLeastOnePending = 1,
}
impl From<GpiointStatusP0intEnum> for bool {
    #[inline(always)]
    fn from(variant: GpiointStatusP0intEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0INT` reader - Port 0 GPIO interrupt pending."]
pub type P0intR = crate::BitReader<GpiointStatusP0intEnum>;
impl P0intR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpiointStatusP0intEnum {
        match self.bits {
            false => GpiointStatusP0intEnum::NoPendingInterrupt,
            true => GpiointStatusP0intEnum::AtLeastOnePending,
        }
    }
    #[doc = "No pending interrupts on Port 0."]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == GpiointStatusP0intEnum::NoPendingInterrupt
    }
    #[doc = "At least one pending interrupt on Port 0."]
    #[inline(always)]
    pub fn is_at_least_one_pending(&self) -> bool {
        *self == GpiointStatusP0intEnum::AtLeastOnePending
    }
}
#[doc = "Port 2 GPIO interrupt pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpiointStatusP2intEnum {
    #[doc = "0: No pending interrupts on Port 2."]
    NoPendingInterrupt = 0,
    #[doc = "1: At least one pending interrupt on Port 2."]
    AtLeastOnePending = 1,
}
impl From<GpiointStatusP2intEnum> for bool {
    #[inline(always)]
    fn from(variant: GpiointStatusP2intEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2INT` reader - Port 2 GPIO interrupt pending."]
pub type P2intR = crate::BitReader<GpiointStatusP2intEnum>;
impl P2intR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpiointStatusP2intEnum {
        match self.bits {
            false => GpiointStatusP2intEnum::NoPendingInterrupt,
            true => GpiointStatusP2intEnum::AtLeastOnePending,
        }
    }
    #[doc = "No pending interrupts on Port 2."]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == GpiointStatusP2intEnum::NoPendingInterrupt
    }
    #[doc = "At least one pending interrupt on Port 2."]
    #[inline(always)]
    pub fn is_at_least_one_pending(&self) -> bool {
        *self == GpiointStatusP2intEnum::AtLeastOnePending
    }
}
impl R {
    #[doc = "Bit 0 - Port 0 GPIO interrupt pending."]
    #[inline(always)]
    pub fn p0int(&self) -> P0intR {
        P0intR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 GPIO interrupt pending."]
    #[inline(always)]
    pub fn p2int(&self) -> P2intR {
        P2intR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "GPIO overall Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
