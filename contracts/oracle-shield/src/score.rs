use {
    crate::{error::Error, status::Status},
    soroban_sdk::{Env, IntoVal, TryFromVal, Val},
};

/// u32 with range check
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(u32);

impl Score {
    const MAX_SCORE: u32 = 100;

    pub fn new(value: u32) -> Result<Self, Error> {
        if value <= Self::MAX_SCORE {
            Ok(Self(value))
        } else {
            Err(Error::ScoreBounds)
        }
    }

    pub fn get(self) -> u32 {
        self.0
    }

    pub fn status(self) -> Status {
        self.into()
    }
}

impl TryFromVal<Env, Val> for Score {
    type Error = Error;

    fn try_from_val(env: &Env, v: &Val) -> Result<Self, Self::Error> {
        let i = u32::try_from_val(env, v).map_err(|_| Error::ConversionError)?;
        Self::new(i)
    }
}

impl IntoVal<Env, Val> for Score {
    fn into_val(&self, e: &Env) -> Val {
        self.get().into_val(e)
    }
}
