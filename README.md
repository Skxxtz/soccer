# Soccer
## TL;DR:
Soccer is a Rust-based terminal app that retrieves the scores, standings, and lineups for the current matchday of the German Bundesliga.

## Commands:
-   soccer                                     » Displays the current scores
-   soccer scores                              » Displays the current scores
-   soccer standings                           » Displays the current standings 
-   soccer match [team name (fuzzy search)]    » Displays the lineup for the selected match
-   soccer --help                              » Displays the available Commands 
-   soccer --version                           » Displays the current version 

```text

┌──────────────────────────┬───────┬───────────────────────┬──────┐
│ Home                     │       │ Away                  │ Time │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ VfL Wolfsburg            │ 2 - 3 │ RB Leipzig            │ OVER │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ Borussia Mönchengladbach │ 1 - 2 │ SC Freiburg           │ OVER │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ VfL Bochum               │ 1 - 2 │ FC Augsburg           │ OVER │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ 1899 Hoffenheim          │ 2 - 0 │ 1. FSV Mainz 05       │ OVER │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ Bayer Leverkusen         │ 0 - 0 │ 1. FC Union Berlin    │ OVER │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ Holstein Kiel            │ 1 - 2 │ FC St. Pauli          │ OVER │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ Bayern München           │ 2 - 2 │ Borussia Dortmund     │ OVER │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ VfB Stuttgart            │ 1 - 2 │ Werder Bremen         │ OVER │
├──────────────────────────┼───────┼───────────────────────┼──────┤
│ Eintracht Frankfurt      │ 3 - 0 │ 1. FC Heidenheim 1846 │ OVER │
└──────────────────────────┴───────┴───────────────────────┴──────┘

```
