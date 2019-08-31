Playing random vs. random (cf random_vs_random.png) on 219373 games give 128711 white victories, 63058 black victories and 27604 ties .
This represents ~59% white wins, ~29% black wins and ~12% ties. The advantage of starting is clear!

Total games: 219373
White wins: 128711 (58.67221581507296%)
Black wins: 63058 (28.744649523870304%)
Ties: 27604 (12.58313466105674%)

A perfect white player vs. a random black player (cf perfect_vs_random.png) wins roughly 99.5% of the time and ties the remaining 0.5%:

Total games: 101988
White wins: 101437 (99.45974036161117%)
Black wins: 0 (0%)
Ties: 551 (0.5402596383888301%)

A random white player vs. a perfect black player (cf random_vs_perfect.png) looses roughly 80% of the time and ties the remaining 20%:

Total games: 108375
White wins: 0 (0%)
Black wins: 87326 (80.57762399077278%)
Ties: 21049 (19.42237600922722%)

This shows again the big advantage, for an imperfect player, of starting, even against a perfect player.

Of course, perfect player vs. perfect player results in 100% ties:

Total games: 10116
White wins: 0 (0%)
Black wins: 0 (0%)
Ties: 10116 (100%)


From memory, algorithms I wanted to implement (need to find back the source doc):
 - Value Iteration
 - Q Learning
 - Advantage learning
 - TD lambda

Advantage learning actually doesn't make sense here because it's main purpose is to avoid having a low difference in value between actions when using an estimator (e.g neural network) for the value function.
For example, if values are 999 vs. 1000 for 2 actions in the same state, the estimator needs to converge very well (0.1% precision) to make the correct decision. In this case, learning an advantage instead of a value avoids the problem (would be -1 vs. 0 for example).
[But is it not feasible? Because the whole exercise has no practical interest anyway but if it's feasible, it's still interesting to do it. Typically Q-learning has no interest over value iteration for TicTacToe ...]
