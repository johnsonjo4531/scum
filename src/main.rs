use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use std::collections::{BTreeMap, HashMap};

const FACES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

// ---------------------------------- Cards -----------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card(String, String, u8);

fn card_value_map() -> HashMap<String, u8> {
    let faces = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    let values = 2..=14;

    faces
        .into_iter()
        .zip(values)
        .map(|(face, value)| (face.to_string(), value))
        .collect()
}

fn cards() -> impl Iterator<Item = Card> {
    let face_value = card_value_map();
    vec![
        vec!["Clubs", "Diamonds", "Hearts", "Spades"],
        vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ],
    ]
    .into_iter()
    .map(|x| {
        x.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    })
    .multi_cartesian_product()
    .map(move |pair| {
        let [suit, value]: [String; 2] = pair.try_into().expect("Expected a pair");
        Card(suit, value.clone(), *face_value.get(&value).unwrap())
    })
}

fn initial_cards() -> Vec<Card> {
    let mut rng = rand::rng();

    let mut cards = cards().collect::<Vec<Card>>();
    cards.shuffle(&mut rng);
    cards
}

fn display_card(card: &Card) -> String {
    let suit = card.0.as_str();
    match card.1.as_str() {
        "J" => format!("Jack of {suit}"),
        "Q" => format!("Queen of {suit}"),
        "K" => format!("King of {suit}"),
        "A" => format!("Ace of {suit}"),
        face => format!("{face} of {suit}"),
    }
}

fn image_name(card: &Card) -> String {
    format!(
        "kenney_boardgame-pack/PNG/Cards/card{}{}.png",
        card.0, card.1
    )
}

fn suit_order(suit: &str) -> u8 {
    match suit {
        "Clubs" => 0,
        "Diamonds" => 1,
        "Hearts" => 2,
        _ => 3,
    }
}

fn rank_word(value: u8) -> &'static str {
    match value {
        2 => "Twos",
        3 => "Threes",
        4 => "Fours",
        5 => "Fives",
        6 => "Sixes",
        7 => "Sevens",
        8 => "Eights",
        9 => "Nines",
        10 => "Tens",
        11 => "Jacks",
        12 => "Queens",
        13 => "Kings",
        _ => "Aces",
    }
}

fn play_description(size: u8, value: u8) -> String {
    match size {
        1 => format!("a lone {}", FACES[(value - 2) as usize % FACES.len()]),
        size => format!(
            "{} of {}",
            match size {
                2 => "a pair",
                3 => "a triple",
                _ => "a quad",
            },
            rank_word(value)
        ),
    }
}

// ---------------------------------- Ranks -----------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    King,
    Queen,
    Peasant(u8),
    ViceScum,
    Scum,
}

impl core::fmt::Display for Rank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Rank::King => write!(f, "King"),
            Rank::Queen => write!(f, "Queen"),
            Rank::Peasant(number) => write!(f, "Peasant-{number}"),
            Rank::ViceScum => write!(f, "Vice Scum"),
            Rank::Scum => write!(f, "Scum"),
        }
    }
}

fn rank_at(position: usize, player_count: usize) -> Rank {
    if position == 0 {
        Rank::King
    } else if position == 1 {
        Rank::Queen
    } else if position + 2 == player_count {
        Rank::ViceScum
    } else if position + 1 == player_count {
        Rank::Scum
    } else {
        Rank::Peasant((position - 1) as u8)
    }
}

// ------------------------------- Components ---------------------------------

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PlayerNum(u8);

#[derive(Component, Default, Clone)]
struct Player {
    cards: Vec<Card>,
}

#[derive(Component, Default, Clone)]
#[require(CurrentCardArea)]
struct CardsInPlay {
    cards: Vec<Card>,
}

#[derive(Component, Default, Clone)]
#[require(ImageNode)]
struct CurrentCardArea;

#[derive(Component, Default, Clone)]
struct DeckArea;

#[derive(Component, Default, Clone)]
struct PlayerCardNum;

#[derive(Component, Default, Clone)]
struct StatusText;

#[derive(Component, Default, Clone)]
struct NameLabel;

// ------------------------------ Game state ----------------------------------

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Begin,
    Setup,
    Turn,
    RoundEnd,
}

#[derive(Resource)]
struct Game {
    round_number: u32,
    trick_rank: Option<u8>,
    trick_size: Option<u8>,
    last_player: Option<u8>,
    active_player: u8,
    passes: u8,
    finish_order: Vec<u8>,
    ranks: Vec<(u8, Rank)>,
    message: String,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            round_number: 0,
            trick_rank: None,
            trick_size: None,
            last_player: None,
            active_player: 1,
            passes: 0,
            finish_order: Vec::new(),
            ranks: Vec::new(),
            message: String::from("SCUM - press SPACE to start"),
        }
    }
}

#[derive(Resource)]
struct AutoPlay {
    timer: Timer,
}

impl Default for AutoPlay {
    fn default() -> Self {
        AutoPlay {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

// ---------------------------------- Scenes -----------------------------------

fn card_back_path(player_num: u8) -> &'static str {
    match player_num {
        1 => "kenney_boardgame-pack/PNG/Cards/cardBack_blue5.png",
        2 => "kenney_boardgame-pack/PNG/Cards/cardBack_green5.png",
        3 => "kenney_boardgame-pack/PNG/Cards/cardBack_red5.png",
        _ => "kenney_boardgame-pack/PNG/Cards/cardBack_blue4.png",
    }
}

fn player_row(num: u8) -> impl Scene {
    bsn! {
        Node {
            margin: UiRect::all(Val::Px(10.)),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        PlayerNum(num)
        Children [
            (
                Node {
                    width: Val::Px(140.),
                    height: Val::Px(190.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                ImageNode { image: {card_back_path(num)} }
                DeckArea
                PlayerNum(num)
            ),
            (
                Node {
                    width: Val::Px(140.),
                    height: Val::Px(190.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                Player
                PlayerNum(num)
                CardsInPlay { cards: Vec::new() }
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                Children [
                    (
                        Text({format!("Player {}", num)})
                        NameLabel
                        PlayerNum(num)
                        TextFont {
                            font_size: FontSize::Px(20.),
                        }
                        TextColor(Color::WHITE)
                        Node {
                            margin: UiRect::all(Val::Px(10.)),
                        }
                    ),
                    (
                        Text({format!("Cards: {}", num)})
                        TextFont {
                            font_size: FontSize::Px(20.),
                        }
                        TextColor(Color::WHITE)
                        Node {
                            margin: UiRect::all(Val::Px(10.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                        }
                        Children [
                            (
                                TextSpan(String::from("?"))
                                PlayerCardNum
                                PlayerNum(num)
                                TextFont {
                                    font_size: FontSize::Px(20.),
                                }
                                TextColor(Color::WHITE)
                            )
                        ]
                    ),
                ]
            ),
        ]
    }
}

fn status_line() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Text(String::from(""))
        TextFont {
            font_size: FontSize::Px(48.),
        }
        TextColor(Color::WHITE)
        Children [
            (
                StatusText
                TextSpan(String::from(""))
                TextFont {
                    font_size: FontSize::Px(48.),
                }
                TextColor(Color::WHITE)
            )
        ]
    }
}

fn ui() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Stretch,
        }
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3))
        Children [
            player_row(1),
            player_row(2),
            player_row(3),
            player_row(4),
            status_line(),
        ]
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_scene_list(bsn_list![Camera2d, ui()]);
}

// ---------------------------------- Systems ---------------------------------

fn game_flow(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut auto: ResMut<AutoPlay>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut players: Query<(&mut Player, &mut CardsInPlay, &PlayerNum)>,
) {
    let current_state = *state.get();
    auto.timer.tick(time.delta());
    let space_pressed = keys.just_pressed(KeyCode::Space);

    match current_state {
        GameState::Begin | GameState::RoundEnd => {
            if space_pressed {
                next_state.set(GameState::Setup);
            }
        }
        GameState::Setup => {}
        GameState::Turn => {
            let acting = space_pressed
                || ((keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight))
                    && auto.timer.just_finished());
            if !acting {
                return;
            }

            let mut holders: Vec<(u8, usize)> = players
                .iter()
                .filter(|(player, _, _)| !player.cards.is_empty())
                .map(|(player, _, player_num)| (player_num.0, player.cards.len()))
                .collect();
            holders.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            if holders.len() <= 1 {
                finalize_round(&mut game, &players, &mut next_state);
                return;
            }

            let mut counts: BTreeMap<u8, u8> = BTreeMap::new();
            for (player, _, player_num) in players.iter() {
                if player_num.0 == game.active_player {
                    for card in &player.cards {
                        *counts.entry(card.2).or_default() += 1;
                    }
                }
            }

            let mut options: Vec<(u8, u8)> = Vec::new();
            if game.trick_rank.is_none() {
                for (rank, count) in &counts {
                    let max_size = (*count).min(4);
                    for size in 1..=max_size {
                        options.push((*rank, size));
                    }
                }
            } else {
                let trick_rank = game.trick_rank.unwrap();
                let trick_size = game.trick_size.unwrap();
                for (rank, count) in &counts {
                    if *count >= trick_size && *rank > trick_rank {
                        options.push((*rank, trick_size));
                    }
                }
            }
            let chosen = options
                .iter()
                .min_by_key(|&(rank, size)| (rank, 5 - size))
                .copied();

            let active_player = game.active_player;

            if let Some((rank, size)) = chosen {
                let mut eliminated = false;
                for (mut player, mut cards_in_play, player_num) in players.iter_mut() {
                    if player_num.0 != active_player {
                        continue;
                    }
                    let mut played: Vec<Card> = Vec::new();
                    while played.len() < size as usize {
                        let index = player
                            .cards
                            .iter()
                            .position(|card| card.2 == rank)
                            .expect("Expected a matching card");
                        played.push(player.cards.remove(index));
                    }
                    cards_in_play.cards = played.clone();
                    game.message = format!(
                        "Player {} played {}",
                        active_player,
                        play_description(size, rank)
                    );
                    if player.cards.is_empty() {
                        game.finish_order.push(active_player);
                        game.message =
                            format!("{} - Player {} is out!", game.message, active_player);
                        eliminated = true;
                    }
                }
                game.trick_rank = Some(rank);
                game.trick_size = Some(size);
                game.last_player = Some(active_player);
                game.passes = 0;

                if eliminated {
                    let remaining = players
                        .iter()
                        .filter(|(player, _, _)| !player.cards.is_empty())
                        .count();
                    if remaining <= 1 {
                        finalize_round(&mut game, &players, &mut next_state);
                        return;
                    }
                }

                // Turn passes to the next holder clockwise after a play.
                let mut nums: Vec<u8> = players
                    .iter()
                    .filter(|(player, _, _)| !player.cards.is_empty())
                    .map(|(_, _, player_num)| player_num.0)
                    .collect();
                nums.sort();
                let next_player = *nums
                    .iter()
                    .find(|num| **num > active_player)
                    .unwrap_or_else(|| nums.first().expect("Expected a holder"));
                game.active_player = next_player;
            } else {
                game.passes += 1;
                game.message = format!("Player {} passed", active_player);

                let holders_count = players
                    .iter()
                    .filter(|(player, _, _)| !player.cards.is_empty())
                    .count();
                let last_player_holds_cards = game.last_player.is_some_and(|last_player| {
                    players.iter().any(|(player, _, player_num)| {
                        player_num.0 == last_player && !player.cards.is_empty()
                    })
                });
                let required_passes = holders_count - if last_player_holds_cards { 1 } else { 0 };

                if game.passes as usize >= required_passes {
                    for (_, mut cards_in_play, _) in players.iter_mut() {
                        cards_in_play.cards.clear();
                    }
                    let holder_nums: Vec<u8> = players
                        .iter()
                        .filter(|(player, _, _)| !player.cards.is_empty())
                        .map(|(_, _, player_num)| player_num.0)
                        .collect();
                    // The last successful player leads again unless they no longer hold
                    // cards; then the lowest-numbered holder leads.
                    let lead = match game.last_player {
                        Some(last_player) if holder_nums.contains(&last_player) => last_player,
                        _ => holder_nums.first().copied().unwrap_or(active_player),
                    };
                    game.trick_rank = None;
                    game.trick_size = None;
                    game.passes = 0;
                    game.active_player = lead;
                    game.message = format!("Trick over - Player {} leads", lead);
                } else {
                    let mut nums: Vec<u8> = players
                        .iter()
                        .filter(|(player, _, _)| !player.cards.is_empty())
                        .map(|(_, _, player_num)| player_num.0)
                        .collect();
                    nums.sort();
                    let next_player = *nums
                        .iter()
                        .find(|num| **num > active_player)
                        .unwrap_or_else(|| nums.first().expect("Expected a holder"));
                    game.active_player = next_player;
                }
            }
        }
    }

    // Consume the press so a held/replayed resource cannot trigger extra turns.
    keys.clear_just_pressed(KeyCode::Space);
}

fn finalize_round(
    game: &mut Game,
    players: &Query<(&mut Player, &mut CardsInPlay, &PlayerNum)>,
    next_state: &mut NextState<GameState>,
) {
    let mut order = game.finish_order.clone();
    let mut remaining: Vec<(u8, usize)> = players
        .iter()
        .filter(|(player, _, _)| !player.cards.is_empty())
        .map(|(player, _, player_num)| (player_num.0, player.cards.len()))
        .collect();
    remaining.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    order.extend(remaining.into_iter().map(|(num, _)| num));

    let player_count = order.len();
    game.ranks = order
        .iter()
        .enumerate()
        .map(|(position, num)| (*num, rank_at(position, player_count)))
        .collect();

    let ranks = game
        .ranks
        .iter()
        .map(|(num, rank)| format!("P{} {}", num, rank))
        .join(", ");
    game.message = format!("Round {} over: {}", game.round_number, ranks);
    next_state.set(GameState::RoundEnd);
}

fn setup_round(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    mut players: Query<(&mut Player, &PlayerNum)>,
) {
    let mut narration = String::new();
    if !game.ranks.is_empty() {
        let rank_of = |wanted: Rank| {
            game.ranks
                .iter()
                .find(|entry| entry.1 == wanted)
                .map(|(num, _)| *num)
        };

        if let Some(scum_num) = rank_of(Rank::Scum) {
            let scum_cards: Vec<&Card> = players
                .iter()
                .find(|(_, player_num)| player_num.0 == scum_num)
                .map(|(player, _)| {
                    let mut cards: Vec<&Card> = player.cards.iter().collect();
                    cards.sort_by_key(|card| std::cmp::Reverse((card.2, suit_order(&card.0))));
                    cards
                })
                .unwrap_or_default();
            if scum_cards.is_empty() {
                narration.push_str("No Scum transfer (Scum holds no cards). ");
            } else {
                let surrendered = scum_cards.len().min(2);
                let given: Vec<String> = scum_cards[..surrendered]
                    .iter()
                    .map(|card| display_card(card))
                    .collect();
                narration.push_str(&format!(
                    "Scum (P{}) gave {} to King; King returned the same cards. ",
                    scum_num,
                    given.join(", ")
                ));
            }
        }
        if let Some(vice_scum_num) = rank_of(Rank::ViceScum) {
            let holding_cards = players.iter().any(|(player, player_num)| {
                player_num.0 == vice_scum_num && !player.cards.is_empty()
            });
            if holding_cards {
                narration.push_str(&format!(
                    "Vice Scum (P{}) surrendered their best card to Queen; Queen returned it. ",
                    vice_scum_num
                ));
            } else {
                narration.push_str("No Vice Scum transfer. ");
            }
        }
    }

    game.round_number += 1;

    let mut players_vec: Vec<_> = players.iter_mut().collect();
    players_vec.sort_by_key(|(_, player_num)| player_num.0);
    for (player, _) in players_vec.iter_mut() {
        player.cards.clear();
    }

    let deck = initial_cards();
    for (i, card) in deck.into_iter().enumerate() {
        let index = i % players_vec.len();
        players_vec[index].0.cards.push(card);
    }

    // The previous round's Scum starts subsequent rounds; Round 1 is won by the
    // holder of the lowest card (ties broken by suit alphabetical order).
    let previous_scum = game
        .ranks
        .iter()
        .find(|(_, rank)| matches!(rank, Rank::Scum))
        .map(|(num, _)| *num);
    let lead = previous_scum.unwrap_or_else(|| {
        players_vec
            .iter()
            .flat_map(|(player, player_num)| {
                player
                    .cards
                    .iter()
                    .map(move |card| (card.2, suit_order(&card.0), player_num.0))
            })
            .min()
            .map(|(_, _, num)| num)
            .unwrap_or(1)
    });

    game.trick_rank = None;
    game.trick_size = None;
    game.last_player = None;
    game.passes = 0;
    game.finish_order.clear();
    game.active_player = lead;
    if narration.is_empty() {
        game.message = format!("Round {} - Player {} leads", game.round_number, lead);
    } else {
        game.message = format!(
            "{} | Round {} - Player {} leads",
            narration.trim_end(),
            game.round_number,
            lead
        );
    }

    next_state.set(GameState::Turn);
}

fn update_card_count(
    players: Query<(&Player, &PlayerNum)>,
    mut card_texts: Query<(&mut TextSpan, &PlayerNum), With<PlayerCardNum>>,
) {
    let counts: BTreeMap<u8, String> = players
        .iter()
        .map(|(player, player_num)| (player_num.0, format!("{} cards", player.cards.len())))
        .collect();
    for (mut text, player_num) in card_texts.iter_mut() {
        if let Some(count) = counts.get(&player_num.0) {
            text.0.clone_from(count);
        }
    }
}

fn update_names(game: Res<Game>, mut names: Query<(&mut Text, &PlayerNum), With<NameLabel>>) {
    let labels: BTreeMap<u8, String> = game
        .ranks
        .iter()
        .map(|(num, rank)| (*num, format!("Player {} ({})", num, rank)))
        .collect();
    for (mut text, player_num) in names.iter_mut() {
        let label = labels
            .get(&player_num.0)
            .cloned()
            .unwrap_or_else(|| format!("Player {}", player_num.0));
        text.0 = label;
    }
}

fn display_current_card(
    mut cards_in_play: Query<(&CardsInPlay, &mut ImageNode), With<CurrentCardArea>>,
    asset_server: Res<AssetServer>,
) {
    for (cards, mut image) in cards_in_play.iter_mut() {
        match cards.cards.first() {
            Some(card) => {
                image.image = asset_server.load(image_name(card));
            }
            None => {
                image.image = Handle::default();
            }
        }
    }
}

fn update_status(
    state: Res<State<GameState>>,
    game: Res<Game>,
    mut statuses: Query<&mut TextSpan, With<StatusText>>,
) {
    let text = match state.get() {
        GameState::Begin => String::from("SCUM - press SPACE to start"),
        GameState::Setup => game.message.clone(),
        GameState::Turn => {
            let trick = match (game.trick_rank, game.trick_size) {
                (Some(rank), Some(size)) => format!("beat {}", play_description(size, rank)),
                _ => String::from("open lead"),
            };
            format!(
                "Round {} | P{} to play ({}) | {}",
                game.round_number, game.active_player, trick, game.message
            )
        }
        GameState::RoundEnd => format!("{} | Press SPACE for the next round", game.message),
    };
    for mut status in statuses.iter_mut() {
        status.0.clone_from(&text);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Scum!"),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Game>()
        .init_resource::<AutoPlay>()
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            (
                game_flow,
                update_card_count,
                update_names,
                display_current_card,
                update_status,
            ),
        )
        .add_systems(OnEnter(GameState::Setup), setup_round)
        .init_state::<GameState>()
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;
    use bevy::scene::ScenePlugin;
    use bevy::state::app::StatesPlugin;

    fn make_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            ScenePlugin::default(),
            StatesPlugin,
        ));
        app.init_resource::<ButtonInput<KeyCode>>()
            .init_asset::<Image>()
            .init_resource::<Game>()
            .init_resource::<AutoPlay>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    game_flow,
                    update_card_count,
                    update_names,
                    display_current_card,
                    update_status,
                ),
            )
            .add_systems(OnEnter(GameState::Setup), setup_round)
            .init_state::<GameState>();
        app.update();
        app
    }

    fn game_state(app: &App) -> GameState {
        *app.world().resource::<State<GameState>>().get()
    }

    fn press_space(app: &mut App) {
        let mut input = ButtonInput::default();
        input.press(KeyCode::Space);
        *app.world_mut().resource_mut::<ButtonInput<KeyCode>>() = input;
        app.update();
    }

    fn hand_sizes(app: &mut App) -> Vec<usize> {
        let mut sizes = Vec::new();
        let world = app.world_mut();
        let query = world.query::<&Player>();
        for player in query.iter_manual(world) {
            sizes.push(player.cards.len());
        }
        sizes
    }

    fn count_with(app: &mut App, marker: &str) -> usize {
        let world = app.world_mut();
        match marker {
            "status" => {
                let query = world.query_filtered::<(), With<StatusText>>();
                query.iter_manual(world).count()
            }
            "player" => {
                let query = world.query::<&Player>();
                query.iter_manual(world).count()
            }
            _ => 0,
        }
    }

    #[test]
    fn deals_and_plays_multiple_rounds() {
        let mut app = make_app();

        // The UI scene (and its player entities) must spawn before input.
        for _ in 0..120 {
            if count_with(&mut app, "player") == 4 && count_with(&mut app, "status") == 1 {
                break;
            }
            app.update();
        }
        assert_eq!(count_with(&mut app, "player"), 4);
        assert_eq!(count_with(&mut app, "status"), 1);

        // Start the game: the round is dealt and a lead is chosen.
        press_space(&mut app);
        let mut guard = 0;
        while game_state(&app) != GameState::Turn && guard < 10 {
            app.update();
            guard += 1;
        }
        assert_eq!(game_state(&app), GameState::Turn);
        let sizes = hand_sizes(&mut app);
        assert_eq!(sizes.len(), 4);
        assert_eq!(sizes.iter().sum::<usize>(), 52);
        assert!(sizes.iter().all(|size| *size == 13));

        // The holder of the lowest card leads in round 1.
        let mut lowest: Option<(u8, u8, u8)> = None;
        {
            let world = app.world_mut();
            let query = world.query::<(&PlayerNum, &Player)>();
            for (player_num, player) in query.iter_manual(world) {
                for card in &player.cards {
                    let key = (card.2, suit_order(&card.0), player_num.0);
                    if lowest.is_none_or(|current| key < current) {
                        lowest = Some(key);
                    }
                }
            }
        }
        let lowest = lowest.expect("Expected dealt cards");
        assert_eq!(app.world().resource::<Game>().active_player, lowest.2);

        // Play until the round ends, checking the trick-size invariant as we go.
        let mut turns = 0;
        while game_state(&app) != GameState::RoundEnd && turns < 3000 {
            if game_state(&app) == GameState::Turn {
                let trick_size = app.world().resource::<Game>().trick_size;
                if let Some(size) = trick_size {
                    let world = app.world_mut();
                    let query = world.query::<&CardsInPlay>();
                    for cards in query.iter_manual(world) {
                        assert!(
                            cards.cards.is_empty() || cards.cards.len() == size as usize,
                            "Cards in play must match the trick size"
                        );
                    }
                }
            }
            press_space(&mut app);
            turns += 1;
        }
        assert_eq!(game_state(&app), GameState::RoundEnd);

        // Exactly one player (the Scum) still holds cards.
        let mut holders = 0;
        for size in hand_sizes(&mut app) {
            if size > 0 {
                holders += 1;
            }
        }
        assert_eq!(holders, 1);

        // With four players the ranks are King, Queen, Vice Scum and Scum.
        let ranks = &app.world().resource::<Game>().ranks;
        assert_eq!(ranks.len(), 4);
        let rank_values: Vec<Rank> = ranks.iter().map(|(_, rank)| *rank).collect();
        assert_eq!(
            rank_values,
            vec![Rank::King, Rank::Queen, Rank::ViceScum, Rank::Scum]
        );
        let scum_num = ranks
            .iter()
            .find(|(_, rank)| matches!(rank, Rank::Scum))
            .map(|(num, _)| *num)
            .expect("Expected a Scum rank");

        // Start round 2: the previous Scum leads and hands are re-dealt.
        press_space(&mut app);
        let game = app.world().resource::<Game>();
        assert_eq!(game.round_number, 2);
        assert_eq!(game.active_player, scum_num);
        let sizes = hand_sizes(&mut app);
        assert_eq!(sizes.iter().sum::<usize>(), 52);
        assert!(sizes.iter().all(|size| *size == 13));

        // Play a handful of turns in round 2 and confirm progress.
        for _ in 0..10 {
            if game_state(&app) == GameState::Turn {
                press_space(&mut app);
            }
        }
        assert!(!app.world().resource::<Game>().message.is_empty());
    }

    fn card(suit: &str, face: &str, value: u8) -> Card {
        Card(String::from(suit), String::from(face), value)
    }

    fn advance_setup_to_turn(mut next_state: ResMut<NextState<GameState>>) {
        next_state.set(GameState::Turn);
    }

    fn make_logic_app(hands: [Vec<Card>; 4]) -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<Game>()
            .init_resource::<AutoPlay>()
            .add_systems(Update, game_flow)
            .add_systems(OnEnter(GameState::Setup), advance_setup_to_turn)
            .init_state::<GameState>();
        app.update();
        for (index, hand) in hands.into_iter().enumerate() {
            let num = index + 1;
            app.world_mut().spawn((
                Player { cards: hand },
                PlayerNum(num as u8),
                CardsInPlay::default(),
            ));
        }
        // Enter Turn through Setup; transitions may cascade one per update.
        press_space(&mut app);
        let mut guard = 0;
        while game_state(&app) != GameState::Turn && guard < 10 {
            app.update();
            guard += 1;
        }
        assert_eq!(game_state(&app), GameState::Turn);
        app
    }

    #[test]
    fn follow_pass_reset_and_ranking() {
        let mut app = make_logic_app([
            vec![
                card("Hearts", "6", 6),
                card("Spades", "6", 6),
                card("Spades", "A", 14),
            ],
            vec![
                card("Hearts", "7", 7),
                card("Spades", "7", 7),
                card("Clubs", "3", 3),
            ],
            vec![card("Hearts", "K", 13), card("Spades", "K", 13)],
            vec![card("Clubs", "2", 2)],
        ]);

        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.round_number = 1;
            game.trick_rank = Some(5);
            game.trick_size = Some(2);
            game.last_player = Some(3);
            game.active_player = 1;
            game.passes = 0;
        }

        // Player 1 follows with a pair of Sixes (lowest legal rank, same size).
        press_space(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(6));
            assert_eq!(game.trick_size, Some(2));
            assert_eq!(game.passes, 0);
            assert_eq!(game.active_player, 2);
            assert!(game.message.contains("Player 1 played a pair of Sixes"));
        }

        // Player 2 follows with a pair of Sevens; Player 3 with a pair of Kings.
        press_space(&mut app);
        assert_eq!(app.world().resource::<Game>().trick_rank, Some(7));
        press_space(&mut app);

        // Player 3 played their last cards and is out.
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(13));
            assert_eq!(game.finish_order, vec![3]);
            assert!(game.message.contains("Player 3 is out"));
        }

        // Player 4 cannot follow a pair with one card and passes.
        press_space(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.passes, 1);
            assert!(game.message.contains("Player 4 passed"));
            // Turn rotates to the lowest-numbered remaining holder.
            assert_eq!(game.active_player, 1);
        }

        // Players 1 and 2 pass; with three holders left of the last winner gone,
        // the trick resets and Player 1 leads instead of the eliminated Player 3.
        press_space(&mut app);
        press_space(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert!(game.trick_rank.is_none());
            assert!(game.trick_size.is_none());
            assert_eq!(game.passes, 0);
            assert_eq!(game.active_player, 1);
            assert!(game.message.contains("Trick over - Player 1 leads"));
        }

        // The trick contents were discarded.
        let played_cards: Vec<usize> = {
            let mut counts = Vec::new();
            let world = app.world_mut();
            let query = world.query::<&CardsInPlay>();
            for cards in query.iter_manual(world) {
                counts.push(cards.cards.len());
            }
            counts
        };
        assert!(played_cards.iter().all(|count| *count == 0));

        // Player 1 leads with their lone Ace and goes out.
        press_space(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(14));
            assert_eq!(game.trick_size, Some(1));
            assert_eq!(game.finish_order, vec![3, 1]);
            assert!(game.message.contains("Player 1 is out"));
        }

        // Nobody can beat the Ace: Players 2 and 4 pass, and since Player 1 no
        // longer holds cards the trick resets with the lowest-numbered holder
        // (Player 2) leading.
        press_space(&mut app);
        assert_eq!(app.world().resource::<Game>().passes, 1);
        press_space(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert!(game.trick_rank.is_none());
            assert_eq!(game.active_player, 2);
            assert!(game.message.contains("Trick over - Player 2 leads"));
        }

        // Player 2 leads with their lone Three and goes out; only Player 4 holds
        // cards, so the round ends and ranks are assigned by finish order.
        press_space(&mut app);
        let mut guard = 0;
        while game_state(&app) != GameState::RoundEnd && guard < 10 {
            app.update();
            guard += 1;
        }
        assert_eq!(game_state(&app), GameState::RoundEnd);
        {
            let game = app.world().resource::<Game>();
            assert_eq!(
                game.ranks,
                vec![
                    (3, Rank::King),
                    (1, Rank::Queen),
                    (2, Rank::ViceScum),
                    (4, Rank::Scum),
                ]
            );
        }
        let mut holders = 0;
        for size in hand_sizes(&mut app) {
            if size > 0 {
                holders += 1;
            }
        }
        assert_eq!(holders, 1);
    }

    #[test]
    fn lead_opens_with_lowest_rank_group() {
        let mut app = make_logic_app([
            vec![
                card("Clubs", "5", 5),
                card("Hearts", "6", 6),
                card("Spades", "6", 6),
            ],
            vec![card("Clubs", "9", 9)],
            vec![card("Clubs", "10", 10)],
            vec![card("Clubs", "J", 11)],
        ]);

        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.active_player = 1;
        }

        // The lead opens with their lowest card.
        press_space(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(5));
            assert_eq!(game.trick_size, Some(1));
        }
    }
}
