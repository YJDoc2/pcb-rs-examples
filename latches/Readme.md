# Latches

This contains implementations of various latches using basic gates.
This is designed using several sources, but mainly:

- [Wikipedia Flip-Flop page](<https://en.wikipedia.org/wiki/Flip-flop_(electronics)>)
- Fundamentals of Digital Logic with VHDL Design by Stephen D. Brown and Zvonko G Vranesic

This contains following latches:

- SR Latch
  - needs 2 Nor gates
  - exposes r (reset), s (set), q and notq pins
  - takes 1 tick to propagate input to output
- Gated SR Latch
  - needs 2 Nor gates and 2 And gates
  - exposes r (reset), s (set), e (enable), q and notq pins
  - needs 2 ticks for propagating input to output
- Gated D Latch
  - needs 1 gated SR Latch and 1 Not gate
  - exposes d (data), e (enable), q and notq pins
  - needs 3 ticks to propagate input to output
- Gated T Latch
  - needs 2 And3 gates and 2 Not gates
  - exposes t, e (enable), q and notq pins
  - needs 2 ticks to propagate input to output
  - default state is Q = False, NotQ = True
- Gated JK Latch
  - needs 2 And3 gates and 2 Not gates
  - exposes j, k, e (enable), q and notq pins
  - needs 2 ticks to propagate input to output
- SR (set-reset) D Latch
  - needs 1 Not gate, 2 And gates, 2 Nor3 gates
  - exposes d (data), e (enable), s (set), r (reset), q and notq pins
  - needs 3 ticks to propagate input to output
