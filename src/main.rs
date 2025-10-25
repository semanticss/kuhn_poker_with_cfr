use rand::prelude::*;
use rand::rng;

// player 1 requires an information set including their card, their history / information set, and more stuff to come
// goal is create 2 player self play


const RANKS: [&str; 3] = ["K", "Q", "J"];
const ACTIONS: [&str; 3] = ["b", "p"];
const TERMINALS: [&str; 5] = [
    "pp",
    "bb",
    "bp",
    "pbb",
    "pbp",
];


struct Player {
    card: usize, // can get 0 1 or 2

} 
// need a structure to hold the infoset, which just actually holds the regrets and strategy

struct ActionData {
    strategy: f64,
    util: Option<f64>,
    cummulative_gain: f64,
}

impl ActionData { // for dictionaries, use hashmap
    fn new(init_strat_val: f64) -> Self {
        Self {
        strategy: init_strat_val,
        util: None,
        cummulative_gain: init_strat_val,
        }
    }
}

struct InfoSet {
    actions: HashMap<String, ActionData>,
    beliefs: HashMap<String, f64>,
    expected_utility: Option<f64>,
    likelihood: Option<f64>,
}

impl InfoSet {
    fn new_uniform() -> Self {
        let mut actions = HashMap::new();
        let init = 0.5;
        actions.insert("b".to_string(), ActionData::new(init));
        actions.insert("p".to_string(), ActionData::new(init));

        Self {
            actions,
            beliefs: HashMap::new(),
            expected_utility: None,
            likelihood: None,
        }
    }
}

fn find_winner(p1_hole: &str, p2_hole: &str) -> bool { // True = p1 win False = p2 win
    if p1_hole == "K" {
        return True
    }
    if p1_hole == "J" {
        return False
    }
    if p2_hole == "K" {
        return False
    }
    if p2_hole == "J" {
        return True
    }
}

fn get_deciding_player(infoSetStr: str) {
    return (infoSetStr.len() - 1) % 2;
}

fn get_possible_opponent_cards(p1_card: str) {
    return RANKS.iter().filter(|&&r| r != p1_card).cloned().collect();
}

fn find_utility_at_terminal_node(p1_hole, p2_hole, terminal_action_str: &str) {
    if terminal_action_str == "pp" {
        if find_winner(p1_hole, p2_hole) {
            return (1,- 1);
        } else {
            return (-1, 1);
        }
    }

    else if terminal_action_str == "bb" || terminal_action_str == "pbb" {
        if find_winner(p1_hole, p2_hole) {
            return (2, -2);
        } else {
            return (-2, 2);
        }
    }

    else if terminal_action_str == "bp" {
        return (1, -1);
    }

    else if terminal_action_str == "pbp" {
        return (-1, 1);
    }
}

fn update_beliefs() {
    for infoSetStr in sortedInfoSets { // to come
        infoSet = infoSets[infoSetStr] // infoSets is a hashmap
        if infoSet.len() == 1 {
            let possible_villain_hole_cards = get_possible_opponent_cards(infoSetStr[0]); //
            for card in possible_villain_hole_cards {
                infoSet.beliefs[card] == 0.5;
            }
        }
        else 
    }
}