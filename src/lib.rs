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
    time: u16,
    pov: UnorderedMap<AccountId, String>,
}
#[derive(BorshSerialize, BorshDeserialize)]
struct User{
    name: String,
    discord: String,
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
    n_exp: u128,
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
            experience: LookupMap::new(b"m"),
            exp_by_area: LookupMap::new(b"m"),
            n_exp: 0,
            fee: 10,
        }
    }
    pub fn new_user(&mut self, wallet: AccountId, n: String, disc: String, mail: String, interst: u8){
        //assert wallet
        self.users.insert(&wallet.clone(), &User{name: n, discord: disc, email: mail, interests: interst, my_exp: Vec::new(), pov_exp: Vec::new(), date: 0});
    }
    pub fn add_experience(&mut self, wallet: AccountId, exp_title: String, v_link: String, amount: u128, desc: String, t: u16, date: i64){
        //assert wallet
        self.experience.insert(&self.n_exp.clone(),
        &Experience{title: exp_title.clone(),
            owner: wallet.clone(),
            description: desc,
            url: v_link,
            reward: amount,
            moment: "".to_string(),
            time : t,
            pov: UnorderedMap::new(b"m"),
            areas: 0,
            exp_date: date
        });
        self.n_exp += 1;
    }
    pub fn get_reward(&mut self, wallet: AccountId, video_n: u128) -> u128{
        let exp = (self.experience.get(&video_n.clone())).unwrap();
        let reward = exp.reward;
        if exp.owner == wallet {
            reward
        }
        else{
            0
        }
    }
}
/*
fn main() {
    let mut contract = Contract::new();
    let id: AccountId = "pepe.near".parse().unwrap();
    contract.new_user(id.clone(), "pepe".to_string(), "pepediscord".to_string(), "pepemail".to_string(), 8);
    contract.add_experience(id.clone(), "experience 1".to_string(), "https://video.de/pepe".to_string(), 12, "descripcion video pepe".to_string(), 1200, 3000);
    let rew = contract.get_reward(id.clone(), 0);
    println!("{:?}", rew);
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