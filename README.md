# toitoi

A [Riichi mahjong library](https://en.wikipedia.org/wiki/Japanese_Mahjong) in Rust. Very WIP.

For now, it handles most [hand scoring](https://en.wikipedia.org/wiki/Japanese_Mahjong_scoring_rules) use cases under standard rules. In addition to the tests included in this repo, the logic has also been tested on about a million [Tenhou](http://tenhou.net/) game logs, and seems to be accurate outside of super-edge cases.

The plan is to eventually expand this library into a full game engine to be used for reinforcement learning and other computational experiments.

Here's a [demo](https://paultan.ca/toitoi-scorer-demo/) of a Wasm-based hand calculator that calls this library under the hood.

More details forthcoming... In the meantime, feel free to check out the tests for examples.
