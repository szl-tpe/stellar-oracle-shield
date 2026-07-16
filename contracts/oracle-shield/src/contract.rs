use {
    crate::{error::Error, score::Score, status::Status},
    soroban_sdk::{
        Address, Env, contract, contractevent, contractimpl, contractmeta, contracttype,
    },
};

contractmeta!(key = "Description", val = "sunzu lab oracle shield");

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
enum DataKey {
    Admin,
    TTL,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Pair(Address, Address);

/// oracle shield main contract
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// initialze contract
    /// set administator address
    pub fn __constructor(env: Env, admin: Address, ttl: Option<u64>) {
        const DEFAULT_TTL: u64 = 3600;
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::TTL, &ttl.unwrap_or(DEFAULT_TTL));
    }

    /// set ttl for a pairs score
    /// `ttl` - u64 seconds
    ///
    /// restricted to admin
    pub fn set_ttl(env: Env, ttl: u64) -> Result<(), Error> {
        let admin = Self::get_admin(&env).ok_or(Error::MissingAdmin)?;
        admin.require_auth();
        env.storage().instance().set(&DataKey::TTL, &ttl);
        Ok(())
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

        let pair = Pair(base, quote);
        let old_score = Self::get_inner_score(&env, &pair);
        let score = Score::new(score, env.ledger().timestamp())?;
        env.storage().temporary().set(&pair, &score);

        if let Ok(old_score) = old_score {
            if old_score.status() != score.status() {
                StatusChange {
                    base: pair.0,
                    quote: pair.1,
                    status: score.into(),
                }
                .publish(&env)
            }
        }
        Ok(())
    }

    fn get_inner_score(env: &Env, pair: &Pair) -> Result<Score, Error> {
        env.storage()
            .temporary()
            .get(&pair)
            .ok_or(Error::PairNotCovered)
            .and_then(|score: Score| {
                let ttl: Option<u64> = env.storage().instance().get(&DataKey::TTL);
                if let Some(ttl) = ttl {
                    if env.ledger().timestamp() - score.ts > ttl {
                        return Err(Error::StaleInput);
                    }
                }
                Ok(score)
            })
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
        let score = Self::get_inner_score(&env, &Pair(base, quote))?;
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
        let score = Self::get_inner_score(&env, &Pair(base, quote))?;
        Ok(score.into())
    }
}

#[contractevent(data_format = "single-value")]
pub struct StatusChange {
    #[topic]
    pub base: Address,
    #[topic]
    pub quote: Address,
    pub status: Status,
}

mod tests;
