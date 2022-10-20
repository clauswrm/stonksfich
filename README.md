# Stonksfish

A fast and simple chess bot written in Rust.

Play me on [Lichess](https://lichess.org/@/stonks_fish).

[![Rust CI](https://github.com/clauswrm/stonksfish/actions/workflows/rust.yml/badge.svg)](https://github.com/clauswrm/stonksfish/actions/workflows/rust.yml)

## How to run

`RUSTFLAGS="-C target-cpu=native" cargo run --release`

## Dependencies

The bot uses the [Chess crate](https://github.com/jordanbray/chess) for keeping track of the game state as well as fast move generation during search and evaluation. For communication with Lichess APIs, the [Licheszter crate](https://github.com/tontsa28/licheszter) is used.

## How it works

The bot listens for events from Lichess and responds accordingly.

```mermaid
sequenceDiagram
    participant Stonksfish
    participant Lichess
    actor Opponent
    Stonksfish->>Lichess: Listen for events
    loop Running
        Opponent->>Lichess: Create challenge
        Lichess-->>Stonksfish: New challenge
        Stonksfish->>Lichess: Accept challenge
        Lichess-->>Stonksfish: Game started
        rect rgb(130, 75, 20)
            loop Playing game
                Opponent->>Lichess: Make move
                Lichess-->>Stonksfish: Updated game state
                Stonksfish->>Lichess: Make move
                Lichess-->>Stonksfish: Updated game state
                Note right of Lichess: Assuming opponent is playing <br> White. If not, Stonksfish <br> makes the first move.
            end
        end
        Lichess-->>Stonksfish: Game over
    end
```
