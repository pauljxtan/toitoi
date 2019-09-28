# toitoi

[![Build Status](https://travis-ci.org/pauljxtan/toitoi.svg?branch=master)](https://travis-ci.org/pauljxtan/toitoi)

[API docs](https://paultan.ca/toitoi/) | [Scoring demo](https://paultan.ca/toitoi-scorer-demo/)

A [Riichi mahjong](https://en.wikipedia.org/wiki/Japanese_Mahjong) in Rust. Very WIP.

For now, it handles most [hand scoring](https://en.wikipedia.org/wiki/Japanese_Mahjong_scoring_rules) use cases under standard rules. In addition to the tests included in this repo, the logic has also been tested on about a million [Tenhou](http://tenhou.net/) game logs, and seems to be accurate outside of super-edge cases.

The plan is to eventually expand this library into a full game engine to be used for reinforcement learning and other computational experiments.

Here's a [demo](https://paultan.ca/toitoi-scorer-demo/) of a Wasm-based hand calculator that calls this library under the hood.

More details forthcoming... In the meantime, feel free to check out the tests for examples.

## API example

Here's an example that scores a hand with 1 _han_ and 110 _fu_:

- Closed tiles: _4-man_, _5-man_, _6-man_, _ton_, _ton_, _nan_, _nan_
- Calls:
  - Closed kan on _1-sou_
  - Closed kan on _chun_
- Agari: _ton_ (by _ron_)
- Round wind: _nan_
- Player wind: _nan_

```rust
use toitoi::score::score;
use toitoi::tile::{tile_from_string, tiles_from_string};
use toitoi::types::{Call, FuReason, HandContext, HanReason, Limit, Points, Yaku};

let tiles = tiles_from_string("456m1122z");
let calls = vec![
    Call::ankan(tile_from_string("1s")),
    Call::ankan(tile_from_string("7z")),
];
let context = HandContext {
    winning_tile: tile_from_string("1z"),
    is_tsumo: false,
    round_wind: tile_from_string("2z"),
    player_wind: tile_from_string("2z"),
    ..Default::default()
};

let results = score(&tiles, &calls, &context);

assert_eq!(results.len(), 1);

assert_eq!(results[0].fu(), 110);
assert_eq!(results[0].han(), 1);
assert_eq!(results[0].limit(), Limit::NoLimit);
assert_eq!(results[0].points(), Points::Ron(3600));

assert_eq!(results[0].fu_reasons(), vec![
    (FuReason::Base, 20),
    (FuReason::OpenTripletHonours, 4),
    (FuReason::YakuhaiPairRoundWind, 2),
    (FuReason::YakuhaiPairPlayerWind, 2),
    (FuReason::ClosedQuadTerminals, 32),
    (FuReason::ClosedQuadHonours, 32),
    (FuReason::ClosedRon, 10),
    (FuReason::RoundUp, 8),
]);
assert_eq!(results[0].han_reasons(), vec![(HanReason::Yaku(Yaku::Chun), 1)]);
```
