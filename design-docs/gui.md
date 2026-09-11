Below is a layout spec written so an LLM that cannot see the image can rebuild it in Bevy using `bsn!`‑style node macros. All Icons in UI can be skipped

## Global style

- Root fills the screen. Background = dark forest‑green felt (`#1B2A20`). Add a brown wooden border frame around the whole screen (`#3A2418`, ~24 px thick) — purely decorative, optional.
- Optional faint centered watermark (a large translucent crown ring in the middle of the green). Can be omitted.
- Default font: serif. UI text color = warm cream/gold (`#E6D2A0`). Pill labels use white/cream text.

## Top‑left legend panel

Positioned anchored top‑left, sitting on a slightly darker translucent rounded rectangle.

- Header row (horizontal): gold crown icon + the word **“Scum”** in large bold serif, gold color.
- Below header, a vertical list of 4 rows, each = [icon] [label] [parenthetical note]:
  1. gold‑crown icon — **King** — (Highest)
  2. red‑crown icon — **Queen** — (Second)
  3. blue‑diamond icon — **Vice Scum** — (Low, but beats Scum)
  4. skull icon — **Scum** — (Lowest)
- Label text cream; parenthetical notes slightly dimmer gray‑cream.

## Opponent areas (three pills + card fans)

Each opponent = a dark rounded pill containing [robot icon] + label, with a fan/stack of cards beneath it. Pill background near‑black (`#101418`), white text.

- **CPU 1** — anchored top‑center. Below the pill: a horizontal fan of **cards, all face down**.
- **CPU 2** — anchored left‑center (vertically middle). To/below the pill: an overlapping vertical fan of **cards, all face down**.
- **CPU 3** — anchored right‑center. Below the pill: an overlapping vertical fan of **cards, all face down**.

## Center deck

- Anchored dead center: a single face‑up card resting on a small stack. **Face up.**

## Player hand (bottom)

- **“You”** pill anchored bottom‑center, containing [person icon] + “You”, same dark pill style.
- Above the pill: a horizontal fan of **cards, all face up**.

## Card state summary (orientation only)

| Group           | Count     | Orientation |
| --------------- | --------- | ----------- |
| CPU 1,2,3 fan's | face down |
| Center deck     | face up   |
| You hand        | face up   |

## Suggested `bsn!` skeleton for the implementer (this is pseudo code as the instructing llm did not know bsn!)

```rust
bsn!(root {   // full‑screen column, dark green bg, brown border
    bsn!(row {   // top band
        bsn!(column {   // top‑left legend panel (translucent rounded rect)
            bsn!(row { Icon(Crown), Text("Scum") })          // header
            bsn!(column {                                     // legend list
                bsn!(row { Icon(GoldCrown),  Text("King"),       Text("(Highest)") })
                bsn!(row { Icon(RedCrown),   Text("Queen"),      Text("(Second)") })
                bsn!(row { Icon(BlueDiamond),Text("Vice Scum"),  Text("(Low, but beats Scum)") })
                bsn!(row { Icon(Skull),      Text("Scum"),       Text("(Lowest)") })
            })
        })
        bsn!(column {   // CPU1 top‑center
            bsn!(pill(Icon(Robot),"CPU 1"))
            bsn!(fan_face_down(CPU1Cards))
        })
    })

    bsn!(row {   // middle band: left / center / right
        bsn!(column { pill(Icon(Robot),"CPU 2"); fan_face_down(CPU2Cards) })   // left‑center
        bsn!(center_deck_face_down())                            // dead center
        bsn!(column { pill(Icon(Robot),"CPU 3"); fan_face_down(CPU3Cards) })   // right‑center
    })

    bsn!(column {   // bottom band, anchored bottom‑center
        bsn!(fan_face_up(PlayerCards))                                     // player hand
        bsn!(pill(Icon(Person),"You"))                                 // "You" pill
    })
})
```

Notes for the implementer: every `fan…` helper just lays out N card nodes with slight rotation/overlap. Keep all text cream/gold on dark pills, background dark green, border brown. All Icons in UI can be skipped
