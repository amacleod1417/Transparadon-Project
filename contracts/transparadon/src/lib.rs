#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, BytesN, Env, IntoVal, Vec};

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    Contribution(Address),
    ContributorList,
    DepositToken,
    RewardToken,
    CampaignActive,
    ContractAdmin,
    ContractInitialized,
}

#[contract]
pub struct CharityContract;

#[contractimpl]
impl CharityContract {
    /// Initialize the contract with the admin address and the deposit token contract address.
    /// Deploys the reward token contract and initializes it.
    ///
    /// # Arguments
    /// - `env` - The execution environment of the contract.
    /// - `admin` - The address of the admin.
    /// - `token_wasm_hash` - The hash of the token contract wasm.
    /// - `token` - The address of the deposit token contract.
    pub fn initialize(env: Env, admin: Address, token_wasm_hash: BytesN<32>, deposit_token: Address) {
        env.storage().instance().set(&StorageKey::ContractAdmin, &admin);

        let reward_contract = token::create_contract(&env, token_wasm_hash, &deposit_token);

        token::Client::new(&env, &reward_contract).initialize(
            &env.current_contract_address(),
            &18u32,
            &"Reward Token".into_val(&env),
            &"REWARD".into_val(&env),
        );

        env.storage().instance().set(&StorageKey::DepositToken, &deposit_token);
        env.storage().instance().set(&StorageKey::RewardToken, &reward_contract);
        env.storage().instance().set(&StorageKey::ContractInitialized, &true);
    }

    /// Get the status of the campaign
    pub fn check_campaign_status(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&StorageKey::CampaignActive)
            .unwrap_or(false)
    }

    /// Records a deposit made by a contributor if the campaign is active.
    ///
    /// # Arguments
    /// - `env` - The execution environment of the contract.
    /// - `contributor` - The address of the contributor making the contribution.
    /// - `deposit_token` - The address of the token to deposit.
    /// - `amount` - The amount of contribution in tokens.
    pub fn contribute(env: Env, contributor: Address, deposit_token: Address, amount: i128) {
        contributor.require_auth();

        let campaign_active: bool = Self::check_campaign_status(env.clone());
        if !campaign_active {
            panic!("The campaign is inactive");
        }

        token::Client::new(&env, &deposit_token).transfer(
            &contributor,
            &env.current_contract_address(),
            &amount,
        );

        let reward_token = Self::get_reward_token(env.clone());
        token::Client::new(&env, &reward_token).mint(&contributor, &amount);

        Self::set_contribution(env.clone(), contributor.clone(), amount);
    }

    /// Withdraws the contribution made by a contributor if the campaign is active.
    ///
    /// # Arguments
    /// - `env` - The execution environment of the contract.
    /// - `contributor` - The address of the contributor making the contribution.
    /// - `recipient` - The address of the recipient of the contribution.
    /// - `deposit_token` - The address of the token to withdraw.
    pub fn withdraw(env: Env, contributor: Address, recipient: Address, deposit_token: Address) {
        contributor.require_auth();

        let campaign_active = Self::check_campaign_status(env.clone());
        if !campaign_active {
            panic!("The campaign is inactive");
        }

        let contribution_amount = Self::get_contribution(env.clone(), contributor.clone());
        token::Client::new(&env, &deposit_token).transfer(
            &env.current_contract_address(),
            &recipient,
            &contribution_amount,
        );

        let reward_token = Self::get_reward_token(env.clone());
        token::Client::new(&env, &reward_token).burn(&contributor, &contribution_amount);
    }

    /// Clear the contributor from the storage
    pub fn clear_contributor(env: Env, contributor: Address) {
        env.storage().instance().remove(&StorageKey::Contribution(contributor));
    }

    /// Get a user's total contribution
    pub fn get_contribution(env: Env, contributor: Address) -> i128 {
        env.storage()
            .instance()
            .get(&StorageKey::Contribution(contributor))
            .unwrap_or(0)
    }

    /// Set a user's contribution
    pub fn set_contribution(env: Env, contributor: Address, amount: i128) {
        env.storage()
            .instance()
            .set(&StorageKey::Contribution(contributor), &amount);
    }

    /// Get the list of contributors
    pub fn get_contributors(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&StorageKey::ContributorList)
            .unwrap_or(vec![&env, env.current_contract_address()])
    }

    /// Get the total contributions
    pub fn get_total_contributions(env: Env) -> i128 {
        let contributors = Self::get_contributors(env.clone());
        let mut total = 0;
        for contributor in contributors.iter() {
            total += Self::get_contribution(env.clone(), contributor.clone());
        }
        total
    }

    /// Get the RewardToken address
    pub fn get_reward_token(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&StorageKey::RewardToken)
            .unwrap_or(env.current_contract_address())
    }

    /// Get user's reward token balance
    pub fn get_reward_token_balance(env: Env, user: Address) -> i128 {
        let reward_token = Self::get_reward_token(env.clone());
        token::Client::new(&env, &reward_token).balance(&user)
    }

    /// Check if a user is a contributor
    fn is_contributor(env: Env, contributor: Address) -> bool {
        env.storage()
            .instance()
            .get(&StorageKey::Contribution(contributor))
            .unwrap_or(false)
    }

    /// Add a new admin
    pub fn add_new_admin(env: Env, new_admin: Address) {
        Self::update_admin(env, new_admin);
    }

    /// Sets the new admin address in the storage.
    fn update_admin(env: Env, new_admin: Address) {
        let current_admin = env
            .storage()
            .instance()
            .get(&StorageKey::ContractAdmin)
            .unwrap_or(env.current_contract_address());

        current_admin.require_auth();
        env.storage().instance().set(&StorageKey::ContractAdmin, &new_admin);
    }

    /// Get the admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&StorageKey::ContractAdmin)
            .unwrap_or(env.current_contract_address())
    }
}

mod token;
mod test;
