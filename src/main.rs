use rand::prelude::*;
extern crate rand;

// remember rand_distr and statrs
#[derive(Clone, Copy, Debug)]
enum Action {Pass, Bet, Fold}

#[derive(Default)]
struct Player {
    id: u8,
    card: u8,
    active: bool,
}

#[derive(Default)]
struct GameState {
    player: usize,
    hole_cards: [i32; 2],
    pot: u8,
    history: Vec<u32>,
    is_terminal: bool,
    players: [Player; 2],
}

#[derive(Default)]
struct InfoSet {
    regrets: Vec<i32>,
    strategy_sum: Vec<i32>,
}

fn history_maker(action: Action, player: &Player) -> u8 {

    let action_integer = match action {
        Action::Pass => 0,
        Action::Bet => 1,
        Action::Fold => 2,
    }
    
    let bet: u8 = if matches!(action, Action::Bet) {1} else {0}; // if action == Action::Bet, then make bet 1, otherwise make it 0

    (action_integer << 1) | (player.id & 0b1)
}

fn history_reader(history: &u8) -> (Action, u8) {
    let action_integer = (history >> 1) & 0b11

    let action = match action_integer {
        0 => Action::Pass,
        1 => Action:: Bet,
        2 => Action::Fold,
    }
    
    let player_id = history & 0b1;

    (action, player_id)
}


fn evaluate_terminal_payout(state: &GameState) -> { // a tuple with player who won and how much they won?
    if !state.is_terminal {
        println("Evaluating payout at non-terminal game state")
        break
    }

    // only interested in last two histories, so call the second to last history in the hand the "first history" and the last action the "last history"
    let pot = state.pot;
    let mut last_history = state.history[state.history.len() - 1];
    let mut first_history = state.history[state.history.len() - 2];

    let last_actor = history_reader(last_action)[1]; // id of the player, not the position state.players, but i dont see how they could ever be different but idk
    let first_actor = history_reader(first_history)[1];

    let last_action = history_reader(last_history)[0];
    let first_action = history_reader(first_history)[0];

    match (first_action, last_action) {
        // Bet Fold
        first_action == Action::Bet && last_action == Action:Fold => (first_actor, pot),

        // Bet Bet

        first_action == Action::Bet && last_action == Action::Bet => {
            // card eval logic
        }

        // Check Check

        first_action == Action::Pass && last_action == Action::Pass {
            // card eval logic
        }

    }

}

fn take_action(state: &GameState, action: u8, player: &Player) { // 0 = pass, 1 = bet, 2 = fold
    let mut new_state = state.clone();

    match action {
        0 => {
            new_state.history.push(history_maker(0), player)
        }, 
        1 => {
            new_state.pot += 1;
            new_state.history.push(history_maker(1))
        }, 
        2 => {
            new_state.active_players[player.id] = False;
            new_state.is_terminal = True
            new_state.history.push(history_maker(2))
        }, 
        _ => (),
    }
    
}

fn deal(player1: &mut Player, player2: &mut Player) {
    let mut deck = vec![0,1,2];
    let mut rng = rand::rng();

    deck.shuffle(&mut rng);

    player1.card = deck[0];
    player2.card = deck[1];
}




fn play_game(optional_state: Optional<&GameState>) {
    let state = optional_state.unwrap_or(GameState {
        active_players: [true, true],
        .. Default::default() });

    
    
}


fn main() {

}
