use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
extern crate rand;

// remember rand_distr and statrs

#[derive(Clone, Copy, Debug)]
enum Act {Pass, Bet, Call, Fold}


#[derive(Clone)]
struct Node {
    regret_sum: [f64; 2], // two actions at one node
    strategy_sum: [f64; 2],
    n_actions: usize,
}

impl Node {
    fn new(n_actions: usize) -> Self {
        Node {
            regret_sum: [0.0; 2],
            strategy_sum: [0.0; 2],
            n_actions,
        }
    }

    fn get_strategy(&self) -> [f64; 2] {
        // regret match
        let r0 = self.regret_sum[0].max(0.0);
        let r1 = self.regret_sum[1].max(0.0);
        let sum = r0 + r1;

        if sum > 0.0 {
            [r0 / sum, r1 / sum]
        } else { // just flip a coin otherwise
            [0.5, 0.5]
        }
    }

    fn accumulate_strategy(&mut self, strategy: [f64; 2], reach_probability: f64) {
        self.strategy_sum[0] += reach_probability * strategy[0]; // weighting with chance of reaching the state
        self.strategy_sum[1] += reach_probability * strategy[1];
    }

    fn average_strategy(&self) -> [f64; 2] {
        let sum = self.strategy_sum[0] + self.strategy_sum[1];
        if sum > 0.0 {
            [self.strategy_sum[0] / sum, self.strategy_sum[1] / sum]
        } else {
            [0.5, 0.5]
        }
    }
}

#[derive(Clone)]
struct GameState {
    player: usize, // represents the players perspective that it is from
    cards: [u8; 2],
    history: String,
}

impl GameState {
    fn new(cards: [u8; 2]) -> Self {
        Self {
            cards,
            history: String::new(),
            player: 0, // !!
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(
            self.history.as_str(),
            "pp" | "bc" | "bf" | "pbc" | "pbf"
        )
    }

    fn utility_p0(&self) -> f64 {
        let p0 = self.cards[0];
        let p1 = self.cards[1];
        let higher_p0 = (p0 > p1) as i32; // true -> 1, false -> 0, casting boolean as i32 i guess
        let higher_p1 = (p1 > p0) as i32;

        match self.history.as_str() {
            "pp" => {
                if higher_p0 == 1 { 1.0 } else { -1.0 }
            }
            "bc" => {
                if higher_p0 == 1 { 2.0 } else { -2.0 }
            }
            "pbc" => {
                if higher_p0 == 1 { 2.0 } else { -2.0 }
            }
            "bf" => {
                1.0
            }
            "pbf" => {
                -1.0
            }
            _ => 0.0,
            }
        }

        fn legal_actions(&self) -> [Act; 2] {
        match self.history.as_str() {
            "" => [Act::Pass, Act::Bet], // p0 can check or bet
            "p" => [Act::Pass, Act::Bet], // P1 can check or bet
            "b" => [Act::Call, Act::Fold], // p1 can call or fold
            "pb" => [Act::Call, Act::Fold], // p0 can call or fold
            _ => [Act::Pass, Act::Pass], // won't be used at terminal
            }
        }

    fn next(&self, a: Act) -> Self {
        let mut s = self.clone();
        match (s.history.as_str(), a) {
            ("", Act::Pass) => { s.history.push('p'); s.player = 1; }
            ("", Act::Bet)  => { s.history.push('b'); s.player = 1; }

            ("p", Act::Pass) => { s.history.push('p');}
            ("p", Act::Bet)  => { s.history.push('b'); s.player = 0; }

            ("b", Act::Call) => { s.history.push('c');}
            ("b", Act::Fold) => { s.history.push('f');}

            ("pb", Act::Call) => { s.history.push('c');}
            ("pb", Act::Fold) => { s.history.push('f');}

            _ => {}
            }
        s
        }
    }

struct Trainer {
    //             infokey  node
    //               ↓       ↓
    nodes: HashMap<String, Node>
}

impl Trainer {
    fn new() -> Self {
        Self {nodes: HashMap::new()}
    }

    fn info_key(state: &GameState) -> String {
        let card = state.cards[state.player]; // how to know which player we are talking about?
        format!("{} | {}", card, state.history)
    }

    fn get_or_make_node(&mut self, key: &str, n_actions: usize) -> &mut Node {
        self.nodes.entry(key.to_string()).or_insert_with(|| Node::new(n_actions)) // gets entry from key or makes a new node with that key
    }

    //                                   reach probabilities
    //                                         ↓      ↓
    fn cfr(&mut self, state: &GameState, p0: f64, p1: f64) -> f64 {
        // basically this runs until we reach a terminal state
        if state.is_terminal() {
            return state.utility_p0();
        }

        let player = state.player;
        let key = Self::info_key(state);
        let actions = state.legal_actions();

        let n_actions = match actions {
            [Act::Pass, Act::Bet] | [Act::Call, Act::Fold] => 2, // always 2 in kuhn poker
            _ => 2,
        };

        let strategy = self.get_or_make_node(&key, 2).get_strategy();
        let reach = if player == 0 {p0} else {p1};

        let mut child_util_p0 = [0.0_f64; 2];

        for (i, a) in [actions[0], actions[1]].iter().enumerate() {
            let next_state = state.next(*a);
            let (np0, np1) = if player == 0 {
                (p0 * strategy[i], p1)
            } else {
                (p0, p1 * strategy[i])
            };
            child_util_p0[i] = self.cfr(&next_state, np0, np1);
        }

        let mut action_val_cur = child_util_p0;

        if player == 1 {
        action_val_cur[0] = -action_val_cur[0];
        action_val_cur[1] = -action_val_cur[1];
    }

    let node_util_cur = strategy[0] * action_val_cur[0] + strategy[1] * action_val_cur[1];


        {
        let node = self.get_or_make_node(&key, 2);
        for i in 0..2 {
            let regret = action_val_cur[i] - node_util_cur;
            if player == 0 { node.regret_sum[i] += regret * p1; }
            else           { node.regret_sum[i] += regret * p0; }
        }

        node.accumulate_strategy(strategy, reach);

        }

        if player == 0 {node_util_cur} else {-node_util_cur}


    }

    fn train(&mut self, iters: usize) {
        let mut rng = rng();
        let mut deck = [0_u8, 1, 2];
        
        for _ in 0..iters {
            deck.shuffle(&mut rng);
            let state = GameState::new([deck[0], deck[1]]);
            self.cfr(&state, 1.0, 1.0);
        }
    }

}

fn main() {

    let mut trainer = Trainer::new();
    trainer.train(10_000_000);

    let mut keys: Vec<_> = trainer.nodes.keys().cloned().collect();
    keys.sort();
    for k in keys {
        let node = trainer.nodes.get(&k).unwrap();
        let avg = node.average_strategy();
        println!("{:<6} -> [{:.3}, {:.3}]", k, avg[0], avg[1]);
    }
}
