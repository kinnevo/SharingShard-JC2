use near_sdk::{env, near_bindgen, AccountId, Balance, Gas, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::collections::{LookupMap, UnorderedMap};

/*
** Structures
*/

#[derive(BorshSerialize, BorshDeserialize)]
struct Experience{
    title: String,
    owner: AccountId,
    description: String,
    url: String,
    areas: u8,
    reward: u128,
    exp_date: i64,
    moment: String,
    time: u8,
    pov: UnorderedMap<AccountId, String>,
}
#[derive(BorshSerialize, BorshDeserialize)]
struct User{
    name: String,
    email: String,
    interests: u8,
    my_exp: Vec<u128>,
    pov_exp: Vec<u128>,
    date: i64,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] 
pub struct Contract{
    users: LookupMap<AccountId, User>,
    experience: LookupMap<u128, Experience>,
    exp_by_area: LookupMap< u8, Vec<u128> >,
    fee: u128,
}

/*
** Functions
*/

/*
** Initialization
*/
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self{
            users: LookupMap::new(b"m"),
            fee: 10,
        }
    }
    pub fn new_user(&mut self, wallet: AccountId){
        //assert wallet
        self.users.insert(&wallet.clone(), &User{wallet: wallet, videos: LookupMap::new(b"m"), date: 0});
    }
    pub fn add_video(&mut self, wallet: AccountId, video_title: String, v_link: String, amount: u128){
        //assert wallet
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.videos.insert(&video_title.clone(), &Videos{link: v_link.clone(), reward: amount});
    }
    pub fn get_reward(&mut self, wallet: AccountId, video_title: String) -> u128{
        let user = self.users.get(&wallet.clone()).unwrap();
        let reward = user.videos.get(&video_title.clone()).unwrap().reward;
        reward
    }
}
/*
fn main() {
    let mut contract = Contract::new();
    let id: AccountId = "bob.near".parse().unwrap();
    contract.new_user(id.clone());
    let title: String = "First vid".to_string();
    let link: String = "https://video.com".to_string();
    let m: u128 = 10;
    contract.add_video(id.clone(), title.clone(), link, m);
    println!("reward = {:?}", contract.get_reward(id, title));
}*/
/*
#[cfg(all(test, not(target_arch = "wasm32")))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(1))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new();
        testing_env!(context.is_view(true).build());
        let wal = accounts(1);
        contract.new_user(wal.clone());
        //let title = "vid_test1".to_string();
        //let url = "https://test1".to_string();
        //let budg: u128 = 125;
        //contract.add_video(wal, title, url, budg);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}*/