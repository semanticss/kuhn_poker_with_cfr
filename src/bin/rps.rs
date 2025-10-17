// use rand::Rng;
// 0 = rock, 1 = paper, 2 = scissors
use rand::prelude::*;
use rand::rng;
use rand::distr::weighted::WeightedIndex;

const PAYOFF_MATRIX: [[(f32, f32); 3]; 3] = [[(0.0,0.0), (-1.0,1.0), (1.0,-1.0)], [(1.0, -1.0), (0.0,0.0), (-1.0,1.0)], [(-1.0,1.0), (1.0, -1.0), (0.0,0.0)]];

fn action_utilities(opponent_action: u8, matrix: &[[(f32, f32);3]; 3])-> Vec<f32> {
    let mut vals = Vec::new();
    for hero_action in 0..3 {
        vals.push(matrix[hero_action as usize][opponent_action as usize].0);
    }
    vals
}

fn get_regrets(hero_action: usize, opponent_action: usize, matrix: &[[(f32, f32);3]; 3]) -> Vec<f32> {
    let utilities = action_utilities(opponent_action as u8, matrix);
    let real_util = utilities[hero_action];

    let regrets = vec![utilities[0] - real_util, utilities[1] - real_util, utilities[2] - real_util];

    return regrets;
}

fn get_strategy(regrets: &[f32]) -> Vec<f32> {
    let mut strategy = Vec::new();
    let mut sum: f32 = 0.0;
    for x in regrets.iter() {
        if *x > 0.0 {
            sum += x;
        }
    }

    if sum <= 0.0 {
        return vec![1.0/3.0, 1.0/3.0, 1.0/3.0]; // 1/3 truncates to 0 because it doesnt do automatic floating point division -- no implicit type conversions are ever done
    }

    for x in regrets.iter() {
        if (*x as f32) < 0.0 {
            strategy.push(0.0)
        }
        else {
            strategy.push(*x / sum);
        }
    }
    return strategy;
}

fn get_action(strategy: &[f32]) -> usize { // returns index: 0 = rock, 1 = paper, 2 = scissors and this works now bang
    let mut rng = rng();
    let dist = WeightedIndex::new(strategy).unwrap();
    return dist.sample(&mut rng);
 }

fn normalize_vector(v: Vec<f32>) -> Vec<f32> { // makes negative numbers 0
    let len = v.len();
    let mut normalized = vec![0.0; len];
    if len == 0 {
        return vec![0.0; len]; // there is a better way to do this with option or smth i think
    }

    let mut sum = 0.0;
    for i in &v {
        sum += i;
    }

    if sum <= 0.0 {
        return vec![1.0 / (len as f32); len];
    } else {
        for i in 0..len {
            if v[i] <= 0.0 {
                normalized[i] = 0.0;
            } else {
            normalized[i] = v[i] / sum;
        }
    }
    }
    return normalized;
}


fn train_p1(iters: usize, p2_strategy: [f32; 3]) -> Vec<f32> {
    let mut p1_regret_sum = vec![0.0,0.0,0.0];
    let mut strategy_sum = vec![0.0,0.0,0.0];

    for _i in 0..iters {
        let p1_strat_curr = get_strategy(&p1_regret_sum);
        let p1_action = get_action(&p1_strat_curr);
        let p2_action = get_action(&p2_strategy);
        let p1_regrets = get_regrets(p1_action, p2_action, &PAYOFF_MATRIX); // theoretically borrow here but i dont use it again so it is calm
        for i in 0..3 {
            p1_regret_sum[i] += p1_regrets[i];
            strategy_sum[i] += p1_strat_curr[i];
        }
    }
    return normalize_vector(strategy_sum);
}

fn train(iters: usize) -> Vec<Vec<f32>> {
    let mut p1_regret_sum = vec![0.0, 0.0, 0.0];
    let mut p1_strategy_sum = vec![0.0,0.0,0.0];
    let mut p2_regret_sum = vec![0.0,0.0,0.0];
    let mut p2_strategy_sum = vec![0.0,0.0,0.0];

    for _i in 0..iters {
        let p1_current_strategy = get_strategy(&p1_regret_sum);
        let p2_current_strategy = get_strategy(&p2_regret_sum);
        let p1_action = get_action(&p1_current_strategy);
        let p2_action = get_action(&p2_current_strategy);
        let p1_regrets = get_regrets(p1_action, p2_action, &PAYOFF_MATRIX);
        let p2_regrets = get_regrets(p2_action, p1_action, &PAYOFF_MATRIX);
        for i in 0..3 {
            p1_regret_sum[i] += p1_regrets[i];
            p2_regret_sum[i] += p2_regrets[i];
            p1_strategy_sum[i] += p1_current_strategy[i];
            p2_strategy_sum[i] += p2_current_strategy[i]
        }
    }

    return vec![normalize_vector(p1_strategy_sum), normalize_vector(p2_strategy_sum)];
}

mod tests {
    
    use super::*;


    #[test]

    fn test() {
        for i in train(100000).iter() { //  issue occurs if the opponent is playing uniformly
            println!("{:?}", i);
        }
    }
}
