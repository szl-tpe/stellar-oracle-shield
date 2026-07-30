use {
    crate::{error::Error, score::Score, status::Status},
    soroban_sdk::{Address, Env, contract, contractimpl, contractmeta, contracttype},
};

contractmeta!(key = "Description", val = "sunzu lab oracle shield");

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
enum DataKey {
    Score(Address, Address),
    Admin,
}

/// oracle shield main contract
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// initialze contract
    /// set administator address
    pub fn __constructor(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    fn get_admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Admin)
    }

    /// set score of a pair
    /// `base` - SAC address of an asset
    /// `quote` - SAC address of an asset
    /// `score` - [0-100] scoring. 0 the more unsafe, 100 the healthier
    ///
    /// restricted to admin
    pub fn set_score(env: Env, base: Address, quote: Address, score: u32) -> Result<(), Error> {
        let admin = Self::get_admin(&env).ok_or(Error::MissingAdmin)?;
        admin.require_auth();
        let score = Score::new(score)?;
        env.storage()
            .temporary()
            .set(&DataKey::Score(base, quote), &score);
        // todo - generate event if score induce State changes
        Ok(())
    }

    fn get_inner_score(env: Env, base: Address, quote: Address) -> Result<Score, Error> {
        // todo - if stale - return StaleInput error
        env.storage()
            .temporary()
            .get(&DataKey::Score(base, quote))
            .ok_or(Error::PairNotCovered)
    }

    /// get score of a pair
    /// `base` - SAC address of an asset
    /// `quote` - SAC address of an asset
    ///
    /// return the score
    /// fails if
    /// - pair is not covered
    /// - input for pair is stale (unreliable score)
    pub fn get_score(env: Env, base: Address, quote: Address) -> Result<u32, Error> {
        let score = Self::get_inner_score(env, base, quote)?;
        Ok(score.get())
    }

    /// get health status of a pair
    /// `base` - SAC address of an asset
    /// `quote` - SAC address of an asset
    ///
    /// return the score
    /// fails if
    /// - pair is not covered
    /// - input for pair is stale (unreliable score)
    pub fn get_status(env: Env, base: Address, quote: Address) -> Result<Status, Error> {
        let score = Self::get_inner_score(env, base, quote)?;
        Ok(score.into())
    }
}

mod tests;
