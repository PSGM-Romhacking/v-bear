<!--
  Kit template (see docs/kit-docs.md). Replace the {{PLACEHOLDERS}} with tools/new-docs.sh and
  resolve every TODO(...). Delete this comment. Everything here depends on the system: this
  file is a skeleton of what Mil Capivaras documented in docs/accuracy.md and in section 9 of
  its README. Write it as you validate the core, not at the end.
-->
# Accuracy: what is left to improve

<!-- TODO(translate): language switcher line (see docs/kit-docs.md) -->

Survey of {{DATE}}, checked against the code and the sources (cite them as [X1], [X2]… listed
in the README's References).

## Status

| # | Item | Status |
|---|---|---|
| 1 | <!-- TODO(system): a known difference from the reference or the real hardware --> | pending / **done: how** / no change needed: why |

Keep one row per item, with a status that says what was done and which test proves it.
Rows are never deleted; a solved row says how it was solved.

## How the comparison is done

<!-- TODO(system):
  - What the reference is (the original core, another emulator, a simulation of the circuit, measurements on the real console).
  - How both are run with the same input (an automatic player: which buttons, at which frames).
  - What is compared and when: picture (pixel by pixel), RAM (byte by byte), at which frames.
  - How a difference is reported (numbers, not impressions).
-->

## Results

<!-- TODO(system): a table per game: picture identical? RAM identical? at which frames. Say which games were tested and which were not. -->

## Validation against the hardware

<!-- TODO(system): measurements taken on a real console, test ROMs, and what matches. Skip if none exist. -->

## What is still missing

<!-- TODO(system): the honest list: what is approximated, what could not be verified, why. -->

## History of the accuracy improvements

| Date | Change |
|---|---|
| {{DATE}} | <!-- TODO(system) --> |
