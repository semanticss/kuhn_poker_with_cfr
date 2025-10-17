# Kuhn Poker with CFR
Kuhn Poker is a 2-player, 3-card simplified poker game used to study imperfect information games. The deck consists of a __Jack__, a __Queen__, and a __King__.
Each player antes 1 unit, each player recieves a random card. The game tree is small:

```mermaid
graph TD
A[Antes & Deal] --> B[Player 1: Check or Bet]
B -- Bet --> C[Player 2: Call or Fold]
B -- Check --> D[Player 2: Check or Bet]
C -- Call --> E[Showdown]
C -- Fold --> F[Player 1 Wins]
D -- Check --> G[Showdown]
D -- Bet --> H[Player 1: Call or Fold]
H -- Call --> I[Showdown]
H -- Fold --> J[Player 2 Wins]

```

This program uses basic __Counterfactual Regret Minimization__ to solve this game.
