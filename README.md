# SharingShard

Use near-cli to deploy the smart contract to NEAR test network:
near deploy --wasmFile target/wasm32-unknown-unknown/release/SharingShard.wasm --accountId <YOUR_ACCOUNT_HERE>

Initializing contract:
near call <YOUR_ACCOUNT_HERE> new --accountId <YOUR_ACCOUNT_HERE>

Add new user:
near call <CONTRACT OWNER WALLET> new_user --args '{"wallet": "<USER WALLET>", "n": "<USER NAME>", "disc": "<USER DIRCORD>", "mail": "<USER EMAL>", "interst": <CODE NUMBER FOR USER ITERESTS>}' --accountId <CALLER WALLET>

Add new experience:
near call <CONTRACT OWNER WALLET> add_experience --args '{"wallet": "<USER WALLET>", "experience_name": "<NAME>", "description": "<EXPERIENCE DESCRIPTION>", "url": "<VIDEO URL>", "reward": <MOMENT REWARD>, "expire_date": <EXPIRATION DATE>, "topic": <CODE NUMBER FOR TOPIC OF VIDEO>}' --accountId <CALLER WALLET>
  
Add moment to experience:
near call <CONTRACT OWNER WALLET> add_moment --args '{"wallet": "<USER WALLET>", "experience_number": "<CODE NUMBER FOR THE EXPERIENCE>", "time": <TIME ON THE VIDEO>, "comment": "<MOMENT COMMENT>"}' --accountId <CALLER WALLET>

Get experience title:
near call <CONTRACT OWNER WALLET> get_title '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get experience description:
near call <CONTRACT OWNER WALLET> get_description '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get video url:
near call <CONTRACT OWNER WALLET> get_url '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get experience topic:
near call <CONTRACT OWNER WALLET> get_topic '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get experience reward:
near call <CONTRACT OWNER WALLET> get_reward '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get experience expiration date:
near call <CONTRACT OWNER WALLET> get_expiration_date '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get moment coment:
near call <CONTRACT OWNER WALLET> get_moment_coment '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get moment time:
near call <CONTRACT OWNER WALLET> get_moment_time '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get points of view for a moment:
near call <CONTRACT OWNER WALLET> get_pov_of_vid '{"video_n": <EXPERIENCE NUMBER>}' --accountId <CALLER WALLET>
  
Get experiences by topic :
near call <CONTRACT OWNER WALLET> get_exp_by_topic '{"topic": <CODE NUMBER OF TOPIC>}' --accountId <CALLER WALLET>
  
Get user's name:
near call <CONTRACT OWNER WALLET> get_user_name '{"wallet": <USER WALLET>}' --accountId <CALLER WALLET>
