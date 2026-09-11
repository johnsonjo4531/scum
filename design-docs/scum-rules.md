# SCUM — Complete Rule Specification (Agentic Recreation Format)

## 1. GAME OVERVIEW

Scum is a multi-round shedding card game where players race to empty their hands. Players are assigned **Ranks** each round based on finishing order. The lowest-ranked player (Scum) suffers consequences and must surrender cards upward. The game has no fixed end condition; it continues indefinitely or until players agree to stop.

---

## 2. HIERARCHY (RANK LADDER)

Ranks are assigned by finishing order in a round. From highest to lowest:

| Position           | Rank Name                                       | Abbrev |
| ------------------ | ----------------------------------------------- | ------ |
| 1st out            | **King**                                        | KNG    |
| 2nd out            | **Queen**                                       | QN     |
| 3rd–(n−2) out      | **Peasant** (numbered: Peasant-1, Peasant-2, …) | PST-x  |
| Second-to-last out | **Vice Scum**                                   | VSC    |
| Last out           | **Scum**                                        | SCM    |

> _Note:_ The number of "Peasant" slots depends on player count. In a 5-player game there is exactly one Peasant slot. In a 4-player game, Queen and Vice Scum are adjacent (no Peasants). Minimum players: 3. Recommended: 4–8.

---

## 3. COMPONENTS

- **Deck:** Standard 52-card deck (no jokers).
- **Card values (ascending):** 2 < 3 < 4 < 5 < 6 < 7 < 8 < 9 < 10 < J < Q < K < A.
- Suits are irrelevant to value; only rank matters for matching.

---

## 4. SETUP (ROUND 1)

1. Shuffle the deck.
2. Deal all cards face-down, one at a time, clockwise. Distribute remainder arbitrarily to first players dealt into. Players must have equal or near-equal hand sizes (max difference of 1 card).
3. All players look at their own hands privately. No information is shared.

---

## 5. SETUP (SUBSEQUENT ROUNDS — CARD TRANSFER)

Before dealing, the previous round's finishing order triggers mandatory transfers. The transfer is **asymmetric**: lower-ranked players must surrender their best cards; higher-ranked players may return any cards they choose.

### Scum → King (2-card exchange)

1. **Scum** identifies their **two highest-ranked cards** in hand (Aces are highest; ties broken arbitrarily since suits are irrelevant). These two cards are handed to the King, face-down.
2. **King** now holds their original hand plus the two received cards. From this combined set, the King selects **any two cards** (these may be the received cards themselves, originals, or a mix) and returns them to the Scum, face-down.
3. The exchange is simultaneous/secret: neither player sees what the other selected before committing.

### Vice Scum → Queen (1-card exchange)

1. **Vice Scum** identifies their **single highest-ranked card** in hand. This card is handed to the Queen, face-down.
2. **Queen** now holds her original hand plus the one received card. From this combined set, the Queen selects **any one card** and returns it to the Vice Scum, face-down.
3. Same simultaneous/secret procedure as above.

### Peasants

- No transfer. Peasants neither give nor receive cards during setup.

> _Edge case:_ If the Scum (or Vice Scum) holds fewer cards than required for the transfer (e.g., only 1 card), they surrender all cards they hold and receive back `min(transfer_count, received_hand_size)` cards from the King/Queen. If a player holds zero cards at transfer time, no transfer occurs for that pair.

After transfers are complete, re-deal all cards evenly as in §4.

---

## 6. TURN STRUCTURE (WITHIN A ROUND)

### 6.1 — Starting Player

- **Round 1:** Randomly determined (e.g., lowest card held plays first; ties broken by suit alphabetical: Clubs < Diamonds < Hearts < Spades).
- **Subsequent rounds:** The previous round's Scum starts.

### 6.2 — Play Phase Loop

The active player must do exactly one of the following:

**(a) PLAY** a legal combination (see §7). Cards are placed face-up on the central pile, visible to all.

**(b) PASS.** If the player cannot or chooses not to play. Passing is **optional** even if a valid play exists (strategic hiding). See §8 for pass consequences.

### 6.3 — Round Reset (when everyone passes consecutively after a play)

- The pile of played cards is collected and set aside (discarded from the round).
- The player who **last successfully played** becomes the new lead for the next trick. They must open with any legal combination.
- If no one can or chooses to play after a reset, the last player who passed before the most recent successful play leads instead (avoids infinite loops).

### 6.4 — Elimination

- A player who empties their hand is **out** and receives their Rank in finishing order. They take no further action for the remainder of the round.
- Play continues with remaining players until only one player holds cards → that player is Scum.

---

## 7. LEGAL PLAYS (COMBINATIONS)

A play consists of N cards where **all cards share the same rank**. Valid combinations:

| Combination | Size    | Example         |
| ----------- | ------- | --------------- |
| Single      | 1 card  | Any single card |
| Pair        | 2 cards | Two 7s          |
| Triple      | 3 cards | Three Jacks     |
| Quad        | 4 cards | Four Queens     |

> A quad is simply four cards of the same rank played together. It carries no special power beyond being a 4-card combination.

### 7.1 — Beating a Previous Play (the "Beat Rule")

To play on top of the current pile, the new combination must satisfy **both** conditions:

- **Same size** as the previous play (a single beats a single; a pair beats a pair; a triple beats a triple; a quad beats a quad), AND
- **Strictly higher rank.**

> A player **cannot** play a different-sized combination over another. You may not play a quad on top of a pair, nor a single on top of a triple. If you hold no card(s) of the same size as the current pile with a strictly higher rank, your only option is to pass.

### 7.2 — Opening a New Trick (after reset)

- Any single card, pair, triple, or quad may be played as an opener. The opener sets the size and rank that subsequent players must beat.

---

## 8. PASS RULES & FORCED CONSEQUENCES

- A player **may** pass even if they hold a valid play (bluffing/hiding).
- If **all remaining active players pass** consecutively after the last successful play, trigger §6.3 (Round Reset). The passing players are **not penalized**.
- There is no "forced play" rule. You may always pass.

---

## 9. ROUND END & RANKING RECAP

When only one player holds cards:

1. That player is **Scum** (last place).
2. The order in which players emptied their hands determines King → Queen → Peasants → Vice Scum → Scum.
3. Record ranks. Proceed to §5 for next-round transfers, then re-deal and begin a new round.

---

## 10. WINNING / ENDING THE GAME

- **There is no winning condition.** The game is endless by design.
- Players may agree to stop after any number of rounds.
- Optional house rule: play to a set number of rounds (e.g., 10) and declare King the "winner."

---

## 11. STATE VARIABLES THE AGENT MUST TRACK

For faithful simulation, maintain:

```
game_state = {
    round_number: int,
    players: [{ id, rank (from prev round), hand: [cards], eliminated: bool }],
    pile: [cards on table],
    current_trick_size: int,          // size of last successful play (1–4)
    current_trick_rank: card_rank,    // rank to beat
    active_player_index: int,         // whose turn
    consecutive_passes: int,          // for reset detection
    finish_order: [player_ids],       // who got out in what order this round
}
```

---

## 12. TURN EXECUTION ALGORITHM (PSEUDOCODE)

```
function take_turn(player):
    if player.hand is empty: mark eliminated; return next_player()

    # Determine legal moves
    legal_plays = compute_legal_plays(player.hand, pile_rank, trick_size)

    if legal_plays is not empty AND player chooses to play:
        chosen = select_play(legal_plays)   # agent strategy layer
        remove cards from hand → place on pile
        update pile_rank, trick_size
        if player.hand is empty: mark eliminated; record finish_order
    else:
        consecutive_passes += 1

    if all_remaining_players_passed():
        reset_trick()
        lead_player = last_successful_player
        return lead_player

    return next_active_player(player)


function compute_legal_plays(hand, pile_rank, trick_size):
    # Only plays of the SAME size as pile_rank, with strictly higher rank
    candidates = all groups of `trick_size` same-rank cards in hand
                   where group.rank > pile_rank
    return candidates   # empty list if none exist → player must pass or bluff-pass


function reset_trick():
    discard pile
    trick_size = null   // cleared; next lead sets it via their opener
    consecutive_passes = 0
```

---

## 13. STRATEGIC NOTES (FOR AGENT DECISION-MAKING)

These are not rules but guidance for an LLM choosing plays:

- Hide high cards early if you're a Peasant or lower; reveal them only when necessary.
- Quads of the same rank as a pair on the table are useless unless another quad is already in play — size must match exactly. Save quads for moments when someone opens with 4, or discard them via passing strategically.
- If you are Scum, your goal is to dump low cards fast and avoid being Scum again — but remember your best cards will be taken in the next round's transfer regardless.
- Passing with cards in hand is valid but risky: it may signal weakness or strength depending on context.
- The King/Queen who receive strong cards during transfer should integrate them into their strategy for the new round; they chose what to return, so they already know what the Scum/Vice Scum will hold.

---

## 14. EDGE CASES & CLARIFICATIONS

| Situation                                                                 | Resolution                                                                                                                                   |
| ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Scum holds fewer than 2 cards at transfer time                            | Transfer `min(2, hand_size)` cards only. If 0 cards (already out), no transfer occurs.                                                       |
| Vice Scum holds 0 cards at transfer time                                  | No transfer occurs for that pair.                                                                                                            |
| King and Queen are the same player (impossible in ≥3 players)             | N/A; ranks are unique per round.                                                                                                             |
| Two players empty hands simultaneously                                    | Not possible under this rule set: plays are sequential, one at a time.                                                                       |
| A quad is played as an opener after reset                                 | Legal. The next player must match with a higher quad or pass. No other size is valid against it.                                             |
| All players pass consecutively but pile is empty (start of round)         | First player must play; if they pass, the trick resets and they lead again. Prevent deadlock by requiring at least one play per trick cycle. |
| Scum's two best cards are tied in rank (e.g., two Aces)                   | Both are surrendered; no tiebreaker needed since both go to King anyway.                                                                     |
| King receives two identical-rank cards that beat everything in their hand | They may choose to return those same cards, effectively "giving back" what they received. This is legal.                                     |
| A player holds a quad but the pile is a pair                              | The quad cannot be played. Only higher pairs are valid responses. The quad must wait for a future trick opened with 4 cards.                 |

---

_End of specification._
