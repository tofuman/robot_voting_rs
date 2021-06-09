use std::collections::HashMap;
pub mod actions;


pub fn add_vote(vote: String, vote_counts: &mut HashMap<String, u32>) {
    if actions::ACTION_STRINGS.iter().any(|v| v.contains(vote.trim())) {
        let count = vote_counts.entry(vote.clone()).or_insert(0);
        *count +=1;
        println!("{} Votes for {}", *count, vote);
    } else {
        println!{"Invalid Vote: {}",vote};
    }
}

pub fn evaluate(vote_counts: &HashMap<String, u32>) -> String {
    let mut highscore:u32 = 0;
    let mut action:String = String::from(actions::ACTION_HALT_STRING);
    for (key, value) in vote_counts {
        if  *value >= highscore {
            highscore = *value;
            action = key.clone();
        }
    }
    action
}
