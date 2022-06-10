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
    
    pub fn add_experience(&mut self, wallet: AccountId, exp_title: String, v_link: String, amount: u128, desc: String, t: u16, date: i64, interest: u8){
        //assert wallet
        self.n_exp += 1;
        self.experience.insert(&self.n_exp.clone(),
        &Experience{title: exp_title.clone(),
            owner: wallet.clone(),
            description: desc,
            url: v_link,
            reward: amount,
            moment: "".to_string(),
            time : t,
            pov: UnorderedMap::new(b"m"),
            areas: interest.clone(),
            exp_date: date
        });
        if self.exp_by_area.contains_key(&interest.clone()){
            let mut vec = self.exp_by_area.get(&interest.clone()).unwrap();
            vec.push(self.n_exp.clone());
            self.exp_by_area.insert(&interest.clone(), &vec);
        }
        else{
            self.exp_by_area.insert(&interest.clone(), &Vec::new());
            let mut vec = self.exp_by_area.get(&interest.clone()).unwrap();
            vec.push(self.n_exp.clone());
            self.exp_by_area.insert(&interest.clone(), &vec);
        }
        let mut usr = self.users.get(&wallet.clone()).unwrap();
        usr.my_exp.push(self.n_exp.clone());
        self.users.insert(&wallet.clone(), &usr);
    }

    pub fn get_title(&self, video_n: u128) -> String{
        self.experience.get(&video_n.clone()).unwrap().title
    }
    
    pub fn get_description(&self, video_n: u128) -> String{
        self.experience.get(&video_n.clone()).unwrap().description
    }

    pub fn get_url(&self, video_n: u128) -> String{
        let exp = self.experience.get(&video_n.clone()).unwrap();
        exp.url
    }

    pub fn get_area(&self, video_n: u128) -> u8 {
        self.experience.get(&video_n.clone()).unwrap().areas
    }

    pub fn get_reward(&self, video_n: u128) -> u128{
        let exp = (self.experience.get(&video_n.clone())).unwrap();
        exp.reward
    }

    pub fn get_expiration_date(&self, video_n: u128) ->i64{
        self.experience.get(&video_n).unwrap().exp_date
    }

    pub fn get_moment_coment(&self, video_n: u128) ->String{
        self.experience.get(&video_n).unwrap().moment
    }

    pub fn get_moment_time(&self, video_n: u128) ->u16{
        self.experience.get(&video_n).unwrap().time
    }

    pub fn get_pov_of_vid(&self, video_n: u128) ->UnorderedMap<AccountId,String>{
        self.experience.get(&video_n).unwrap().pov
    }

    pub fn get_exp_by_area(&self, area: u8) -> Vec<u128>{
        self.exp_by_area.get(&area).unwrap()
    }

    pub fn get_usr_exp(&self, wallet: AccountId) -> Vec<u128>{
        let usr = self.users.get(&wallet.clone()).unwrap();
        usr.my_exp
    }
}

fn main() {
    let mut contract = Contract::new();
    let id: AccountId = "pepe.near".parse().unwrap();
    let id2: AccountId = "bob.near".parse().unwrap();
    contract.new_user(id.clone(), "pepe".to_string(), "pepediscord".to_string(), "pepemail".to_string(), 8);
    for n in 1..20{
        contract.add_experience(id.clone(), "experience 1".to_string(), "https://video.de/pepe".to_string(), 12, "descripcion video pepe".to_string(), 1200, 3000, 2);
    }
    contract.new_user(id2.clone(), "bob".to_string(), "bobdiscord".to_string(), "bobmail".to_string(), 7);
    contract.add_experience(id2.clone(), "experience 2".to_string(), "https://video.de/bob".to_string(), 20, "descripcion video bob".to_string(), 100, 4000, 2);
    let rew = contract.get_reward(1);
    println!("reward for experience 1 = {:?}", rew);
    println!("url = {}", contract.get_url(1));
    println!("pepe's experiences = {:?}", contract.get_usr_exp(id.clone()));
    println!("experiences on area 2 = {:?}", contract.get_exp_by_area(2));
}