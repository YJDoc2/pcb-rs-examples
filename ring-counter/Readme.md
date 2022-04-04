# Ring Counters

This contains two ring counters implemented using Latches and `pcb!`.
This is designed using [this wikipedia page](https://en.wikipedia.org/wiki/Ring_counter).

- One Hot Ring Counter / Straight Ring Counter
  <img src="https://upload.wikimedia.org/wikipedia/commons/4/41/Overbeck_Counter_4bit.svg" alt="Block diagram of One-hot ring counter" width ="700px" />
  Image taken from Wikimedia commons.

  This sets a pattern of `1000` and rotates it.

  - This needs 4 D latches as seen from the diagram
  - exposes e (enable), r (reset), q1-q4 (output) pins
  - to reset back to `1000`, set `e` to `false`, set `r` to `true` for one tick
  - after that each 3 ticks with `e` set to `true` will shift `1` by 1 position

- Johnson's Ring Counter / Twisted Ring Counter
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/69/Johnson_Counter_4bit.svg" alt="Block diagram of Johnson's ring counter" width ="700px" />
  Image taken from Wikimedia commons.
  This changes pattern of `0000` to `1111` and back to `0000`
  - This needs 4 D latches as seen from the diagram
  - exposes e (enable), r (reset), q1-q4 (output) pins
  - to reset back to `0000`, set `e` to `false`, set `r` to `true` for one tick
  - after that each 3 ticks with `e` set to `true` will shift the pattern to next state
