# TODOs

1. [x] implement the GUI as described in design-docs/gui.md
2. [x] Make the player be able to choose their card from their face up cards on their turn by clicking on a card to play it the played card should go to the center.
3. [x] Make every other players turn be a timed animation of the card they play into the center.

## Phases

### Phase 1 — Interactive felt-table GUI (T1, T2, T3) - [x]

- T1: Rebuild the UI per design-docs/gui.md: dark-green felt root with brown frame, top-left legend panel, CPU pills with face-down card fans, face-up center pile, bottom "You" pill with a face-up hand fan.
- T2: Human turns resolve by clicking a hand card (played card goes to the center pile); Pass button for passing; illegal clicks are rejected with feedback.
- T3: CPU turns act automatically after a timed delay, with each played card animating from that player's seat into the center pile.
