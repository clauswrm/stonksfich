use chess::{ChessMove, Color, Game};
use licheszter::{
    client::Licheszter,
    models::board::{BoardState, Challenger, Event},
};
use std::str::FromStr;
use stonksfich::engine::player::{Bot, Player};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    const DEPTH: u8 = 5;
    let bot_player = Bot { depth: DEPTH };

    let client = Licheszter::new(String::from("lip_PV83SUBeitbrybhXD57C"));
    let mut stream = client
        .stream_events()
        .await
        .expect("Error while streaming events.");
    let mut opponent_name = String::from("");
    let mut bot_color = Color::Black;

    println!("Starting...");
    while let Ok(Some(event)) = stream.try_next().await {
        match event {
            Event::Challenge {
                challenge,
                compat: _,
            } => {
                if let Some(user) = challenge.challenger {
                    opponent_name = user.username;
                    println!(
                        "[{}] Challenge recieved. Time control: {}.",
                        challenge.id,
                        challenge.time_control.show.unwrap_or(String::from("n/a"))
                    );
                    client
                        .challenge_accept(&challenge.id)
                        .await
                        .expect("Error when accepting challenge.");
                }
            }
            Event::GameStart { game: game_id } => {
                let mut game = Game::new();
                let mut stream = client
                    .stream_game_state(&game_id.id)
                    .await
                    .expect("Error while streaming game state.");
                while let Ok(Some(state)) = stream.try_next().await {
                    match state {
                        BoardState::GameFull(game_full) => {
                            bot_color = match game_full.white {
                                Challenger::LightUser(white_user) => {
                                    match white_user.username == opponent_name {
                                        true => Color::Black,
                                        false => Color::White,
                                    }
                                }
                                _ => Color::Black,
                            };
                            println!("[{}] Game started. Bot plays {:?}.", game_id.id, bot_color);
                            if bot_color == Color::White {
                                let board = game.current_position();
                                let chosen_move = bot_player.choose_move(&board);
                                let uci_move = format!("{}", chosen_move);
                                client
                                    .make_move(&game_id.id, &uci_move, false)
                                    .await
                                    .expect("Error when making move.");
                            }
                        }
                        BoardState::GameState(game_state) => {
                            if game_state.status == "started" {
                                let last_move = game_state
                                    .moves
                                    .rsplitn(2, " ")
                                    .next()
                                    .expect("Move string should contain a substring when splitting by space.");
                                println!("[{}] Move made: {}", game_id.id, last_move);
                                if let Ok(chess_move) = ChessMove::from_str(last_move) {
                                    let move_result = game.make_move(chess_move);
                                    if move_result {
                                        if game.side_to_move() == bot_color {
                                            let board = game.current_position();
                                            let chosen_move = bot_player.choose_move(&board);
                                            let uci_move = format!("{}", chosen_move);
                                            client
                                                .make_move(&game_id.id, &uci_move, false)
                                                .await
                                                .expect("Error when making move.");
                                        }
                                    } else {
                                        println!(
                                            "[{}] Move could not be made: '{}'.",
                                            game_id.id, last_move
                                        );
                                    }
                                } else {
                                    println!(
                                        "[{}] Illegal move recieved: '{}'.",
                                        game_id.id, last_move
                                    );
                                }
                            } else {
                                println!(
                                    "[{}] Game ended with status {}.",
                                    game_id.id, game_state.status
                                );
                            }
                        }
                        game_state => {
                            println!(
                                "[{}] Other game state recieved: {:?}",
                                game_id.id, game_state
                            );
                        }
                    }
                }
            }
            Event::GameFinish { game: game_id } => {
                println!("[{}] Finished.", game_id.id);
                break;
            }
            Event::ChallengeCanceled { challenge } => {
                println!("[{}] Cancelled.", challenge.id);
                break;
            }
            event => {
                println!("Other event recieved: {:?}", event);
            }
        }
    }
    println!("Shutting down...");
}
