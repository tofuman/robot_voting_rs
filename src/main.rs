use std::collections::HashMap;
mod votes;
mod roboter;
mod mqtt;


fn main() {
    let mut votes: HashMap<String, u32> = HashMap::new();
    votes::add_vote(String::from("w"), &mut votes);
    votes::add_vote(String::from("a"), &mut votes);
    votes::add_vote(String::from("a"), &mut votes);
    votes::add_vote(String::from("x"), &mut votes);

    println!("{:?}", votes::evaluate(&votes));

    mqtt::mqtttest();

}
