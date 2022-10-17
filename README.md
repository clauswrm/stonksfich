# Stonksfich

A fast chess bot playable on Lichess written in Rust.

[![Rust CI](https://github.com/clauswrm/stonksfich/actions/workflows/rust.yml/badge.svg)](https://github.com/clauswrm/stonksfich/actions/workflows/rust.yml)

![Image of fich with stonks](https://i.imgflip.com/3zkg5p.jpg)

## How to run

`RUSTFLAGS="-C target-cpu=native" cargo run`

## Dependencies

The bot uses the [Chess crate](https://github.com/jordanbray/chess) for keeping track of the game state as well as fast move generation during search and evaluation. For communication with Lichess APIs, the [Licheszter crate](https://github.com/tontsa28/licheszter) is used.

## How it works

The bot listens for events from Lichess and responds accordingly.

```mermaid
sequenceDiagram
    participant Stonksfich
    participant Lichess
    actor Opponent
    Stonksfich->>Lichess: Listen for events
    loop Running
        Opponent->>Lichess: Create challenge
        Lichess-->>Stonksfich: New challenge
        Stonksfich->>Lichess: Accept challenge
        Lichess-->>Stonksfich: Game started
        rect rgb(130, 75, 20)
            loop Playing game
                Opponent->>Lichess: Make move
                Lichess-->>Stonksfich: Updated game state
                Stonksfich->>Lichess: Make move
                Lichess-->>Stonksfich: Updated game state
                Note right of Lichess: Assuming opponent is playing <br> White. If not, Stonksfich <br> makes the first move.
            end
        end
        Lichess-->>Stonksfich: Game over
    end
```
