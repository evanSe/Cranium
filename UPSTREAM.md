# Cranium provenance

- Upstream: https://github.com/scrdest/Cranium
- Commit: `ac2c36e0eb176742b4f6f3380d4fd4fa1e6a432b`
- Version: 0.3.0, Bevy 0.19
- License: MPL-2.0, retained in `LICENSE.md` and source headers.
- Published (as the game ships it): https://github.com/evanSe/Cranium/tree/habet

Habet uses the native plugin, registered context fetchers, considerations, curves,
action handlers and action trackers. The library is vendored to make these fixes
reviewable and keep all native crates at the same revision:

1. Reset the raw best-context score per action template. Cross-template comparison
   must account for priority and consideration adjustment. Reproduced before fixing:
   `0.8 * priority 1` incorrectly beat `0.7 * priority 3`.
2. Remove completed tracker components from an AI controller instead of despawning
   the controller. The native spawner attaches trackers to the controller itself.
   The unmodified runtime failed the controller-survival regression.
3. Register the `NoDecisionMessage` channel used by the native decision engine.
4. Enable the native trigger-dispatch observer so registered action handlers receive
   selected actions. Upstream left this observer commented out.
5. Leave a consideration on a yes/no curve (`AtLeast`, `LessThan`, `Equals`,
   `NotEquals`; `SupportedUtilityCurve::is_gate`) out of the count behind the
   consideration makeup. A gate passes a score at 1.0 or zeroes it; counting it
   raised every passing score, so adding Habet's `habet::eligible` gate would
   have reweighed every choice.

The vendored Rust source and examples are formatted with the workspace's rustfmt;
trailing whitespace in upstream API macros was removed to allow `cargo fmt --all`.

Regression tests run as part of Habet's `tests/cranium_contract.rs`. Changes to
these covered upstream files remain available here with their original license.
Unused API/FFI crates are upstream source, not enabled Habet dependencies.
