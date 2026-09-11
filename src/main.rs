use bevy::picking::Pickable;
use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use std::collections::{BTreeMap, HashMap};

const FACES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

// ---------------------------------- Colors ----------------------------------

fn felt_color() -> Color {
    Color::srgb(0x1B as f32 / 255., 0x2A as f32 / 255., 0x20 as f32 / 255.)
}

fn frame_color() -> Color {
    Color::srgb(0x3A as f32 / 255., 0x24 as f32 / 255., 0x18 as f32 / 255.)
}

fn pill_color() -> Color {
    Color::srgb(0x10 as f32 / 255., 0x14 as f32 / 255., 0x18 as f32 / 255.)
}

fn cream_color() -> Color {
    Color::srgb(0xE6 as f32 / 255., 0xD2 as f32 / 255., 0xA0 as f32 / 255.)
}

fn dim_cream_color() -> Color {
    Color::srgb(
        0xE6 as f32 / 255. * 0.65,
        0xD2 as f32 / 255. * 0.65,
        0xA0 as f32 / 255. * 0.65,
    )
}

fn panel_color() -> Color {
    Color::srgb(0., 0., 0.).with_alpha(0.35)
}

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

fn player_name(num: u8) -> String {
    if num == 1 {
        String::from("You")
    } else {
        format!("CPU {}", num - 1)
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
struct StatusText;

#[derive(Component, Default, Clone)]
struct NameLabel;

#[derive(Component, Default, Clone)]
struct PlayerCardNum;

/// The felt playfield node; fans, the pile and flying cards are attached here.
#[derive(Component, Default, Clone)]
struct PlayfieldRoot;

/// The face-up card resting on the center pile stack.
#[derive(Component, Default, Clone)]
struct PileCard;

#[derive(Component, Default, Clone)]
struct PassButton;

/// A clickable card in the human hand.
#[derive(Component, Clone)]
struct HandCard {
    card: Card,
}

/// Marks a spawned fan/hand card entity so it can be rebuilt per player.
#[derive(Component, Default, Clone)]
struct FanOwner(u8);

/// A card animating from its player's seat into the center pile.
#[derive(Component)]
struct FlyingCard {
    start: Vec2,
    end: Vec2,
    elapsed: f32,
}

const CARD_W: f32 = 120.;
const CARD_H: f32 = 168.;
const CPU_CARD_W: f32 = 90.;
const CPU_CARD_H: f32 = 126.;
const FLY_TIME: f32 = 0.5;

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
    trick_path: Option<String>,
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
            trick_path: None,
        }
    }
}

const CPU_DELAY: u32 = 90;

/// Countdown until the active CPU acts (roughly 1.5 seconds at 60 fps).
#[derive(Resource)]
struct CpuTurn {
    delay: u32,
}

impl Default for CpuTurn {
    fn default() -> Self {
        CpuTurn { delay: 45 }
    }
}

#[derive(Clone)]
enum HumanAction {
    Play(Card),
    Pass,
}

/// The pending action chosen by the human through clicking.
#[derive(Resource, Default)]
struct HumanIntent {
    action: Option<HumanAction>,
}

/// Tracks the rendered fan contents so card entities can be rebuilt on change.
#[derive(Resource, Default)]
struct FanRender {
    counts: BTreeMap<u8, usize>,
    win: Option<Vec2>,
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

fn seat(num: u8) -> impl Scene {
    let label = format!("{}", player_name(num));
    // Absolute anchor insets for each seat on the playfield.
    let (top, bottom, left, right, margin_left) = match num {
        // "You" pill anchored bottom-center.
        1 => (
            Val::Auto,
            Val::Px(10.),
            Val::Percent(50.),
            Val::Auto,
            Some(Val::Px(-75.)),
        ),
        // CPU 1 anchored top-center.
        2 => (
            Val::Px(10.),
            Val::Auto,
            Val::Percent(50.),
            Val::Auto,
            Some(Val::Px(-75.)),
        ),
        // CPU 2 anchored left-center.
        3 => (
            Val::Percent(30.),
            Val::Auto,
            Val::Px(16.),
            Val::Auto,
            None,
        ),
        // CPU 3 anchored right-center.
        _ => (
            Val::Percent(30.),
            Val::Auto,
            Val::Auto,
            Val::Px(16.),
            None,
        ),
    };
    let margin = match margin_left {
        Some(value) => UiRect {
            left: value,
            ..Default::default()
        },
        None => UiRect::DEFAULT,
    };
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(6.),
            top: {top},
            bottom: {bottom},
            left: {left},
            right: {right},
            margin: {margin},
        }
        Children [
            (
                Node {
                    width: Val::Px(150.),
                    height: Val::Px(36.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                BackgroundColor({pill_color()})
                Children [
                    (
                        Text({label})
                        NameLabel
                        PlayerNum(num)
                        TextFont {
                            font_size: FontSize::Px(20.),
                        }
                        TextColor(Color::WHITE)
                    )
                ]
            ),
            (
                Text(String::from(""))
                PlayerCardNum
                PlayerNum(num)
                TextFont {
                    font_size: FontSize::Px(16.),
                }
                TextColor({cream_color()})
            ),
        ]
    }
}

fn legend_row(label: &'static str, note: &'static str) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.),
            align_items: AlignItems::Center,
        }
        Children [
            (
                Text(String::from(label))
                TextFont {
                    font_size: FontSize::Px(20.),
                }
                TextColor({cream_color()})
            ),
            (
                Text(String::from(note))
                TextFont {
                    font_size: FontSize::Px(14.),
                }
                TextColor({dim_cream_color()})
            ),
        ]
    }
}

fn legend_panel() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.),
            top: Val::Px(20.),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            padding: UiRect::new(
                Val::Px(16.),
                Val::Px(16.),
                Val::Px(12.),
                Val::Px(12.),
            ),
        }
        BackgroundColor({panel_color()})
        Children [
            (
                Text(String::from("Scum"))
                TextFont {
                    font_size: FontSize::Px(34.),
                }
                TextColor({cream_color()})
            ),
            legend_row("King", "(Highest)"),
            legend_row("Queen", "(Second)"),
            legend_row("Vice Scum", "(Low, but beats Scum)"),
            legend_row("Scum", "(Lowest)"),
        ]
    }
}

fn pile_stack() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.),
            top: Val::Percent(50.),
            width: Val::Px(CARD_W),
            height: Val::Px(CARD_H),
            margin: UiRect::new(
                Val::Px(-CARD_W / 2. + 3.),
                Val::Auto,
                Val::Px(-CARD_H / 2. + 3.),
                Val::Auto,
            ),
        }
        ImageNode { image: "kenney_boardgame-pack/PNG/Cards/cardBack_blue1.png" }
    }
}

fn pile_card() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.),
            top: Val::Percent(50.),
            width: Val::Px(CARD_W),
            height: Val::Px(CARD_H),
            margin: UiRect::new(
                Val::Px(-CARD_W / 2.),
                Val::Auto,
                Val::Px(-CARD_H / 2.),
                Val::Auto,
            ),
        }
        ImageNode { image: Handle::default() }
        PileCard
    }
}

fn status_bar() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(24.),
            bottom: Val::Px(24.),
            max_width: Val::Px(800.),
        }
        Text(String::from(""))
        Children [
            (
                StatusText
                TextSpan(String::from(""))
                TextFont {
                    font_size: FontSize::Px(22.),
                }
                TextColor({cream_color()})
            )
        ]
    }
}

fn pass_button() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(24.),
            bottom: Val::Px(24.),
            width: Val::Px(130.),
            height: Val::Px(46.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor({pill_color()})
        Button
        PassButton
        Children [
            (
                Text(String::from("PASS"))
                TextFont {
                    font_size: FontSize::Px(22.),
                }
                TextColor({cream_color()})
                Pickable::IGNORE
            )
        ]
    }
}

fn player_entity(num: u8) -> impl Scene {
    bsn! {
        Player { cards: Vec::new() }
        PlayerNum(num)
    }
}

fn ui() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            padding: UiRect::all(Val::Px(24.)),
        }
        BackgroundColor({frame_color()})
        Children [
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                }
                BackgroundColor({felt_color()})
                PlayfieldRoot
                Children [
                    player_entity(1),
                    player_entity(2),
                    player_entity(3),
                    player_entity(4),
                    legend_panel(),
                    pile_stack(),
                    pile_card(),
                    status_bar(),
                    pass_button(),
                    seat(1),
                    seat(2),
                    seat(3),
                    seat(4),
                ]
            )
        ]
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_scene_list(bsn_list![Camera2d, ui()]);
}

// ---------------------------------- Systems ---------------------------------

fn fan_layout(num: u8, count: usize, win: Vec2) -> Vec<(f32, f32, f32)> {
    if count == 0 {
        return Vec::new();
    }
    let (cw, ch) = if num == 1 {
        (CARD_W, CARD_H)
    } else {
        (CPU_CARD_W, CPU_CARD_H)
    };
    let last = count - 1;
    let mut positions = Vec::with_capacity(count);
    match num {
        // Bottom hand: wide horizontal fan above the "You" pill.
        1 => {
            let step = ((win.x * 0.94 - cw) / last.max(1) as f32).clamp(16., 46.);
            let span = step * last as f32;
            for i in 0..count {
                let t = if last == 0 {
                    0.
                } else {
                    i as f32 / last as f32 - 0.5
                };
                let x = win.x * 0.5 - span / 2. + step * i as f32;
                let y = win.y * 0.75 + t.abs() * 34.;
                positions.push((x, y, t * 16.));
            }
        }
        // CPU 1: horizontal fan under the top pill.
        2 => {
            let step = ((win.x * 0.8 - cw) / last.max(1) as f32).clamp(14., 40.);
            let span = step * last as f32;
            for i in 0..count {
                let t = if last == 0 {
                    0.
                } else {
                    i as f32 / last as f32 - 0.5
                };
                let x = win.x * 0.5 - span / 2. + step * i as f32;
                let y = win.y * 0.175 + t.abs() * 18.;
                positions.push((x, y, t * -16.));
            }
        }
        // CPU 2: vertical fan down the left edge, starting below the pill.
        3 => {
            let step = ((win.y * 0.5 - ch) / last.max(1) as f32).clamp(8., 22.);
            for i in 0..count {
                let t = if last == 0 {
                    0.
                } else {
                    i as f32 / last as f32 - 0.5
                };
                let x = win.x * 0.095;
                let y = win.y * 0.42 + step * i as f32;
                positions.push((x, y, t * 8.));
            }
        }
        // CPU 3: vertical fan down the right edge, starting below the pill.
        _ => {
            let step = ((win.y * 0.5 - ch) / last.max(1) as f32).clamp(8., 22.);
            for i in 0..count {
                let t = if last == 0 {
                    0.
                } else {
                    i as f32 / last as f32 - 0.5
                };
                let x = win.x * 0.905;
                let y = win.y * 0.42 + step * i as f32;
                positions.push((x, y, t * -8.));
            }
        }
    }
    positions
}

fn seat_point(num: u8, win: Vec2) -> Vec2 {
    match num {
        1 => Vec2::new(win.x * 0.5, win.y * 0.75),
        2 => Vec2::new(win.x * 0.5, win.y * 0.175),
        3 => Vec2::new(win.x * 0.095, win.y * 0.33),
        _ => Vec2::new(win.x * 0.905, win.y * 0.33),
    }
}

fn window_size(windows: &Query<&Window>) -> Vec2 {
    // Fans are laid out inside the playfield, which sits within the 24px frame.
    windows
        .iter()
        .next()
        .map(|window| {
            let res = window.resolution.size();
            Vec2::new((res.x - 48.).max(320.), (res.y - 48.).max(320.))
        })
        .unwrap_or(Vec2::new(1232., 672.))
}

/// Rebuilds the face-down CPU fans and the human hand fan when contents change.
fn sync_fans(
    mut commands: Commands,
    players: Query<(&Player, &PlayerNum)>,
    playfield: Query<Entity, With<PlayfieldRoot>>,
    mut owned: Query<(Entity, &FanOwner)>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
    mut render: ResMut<FanRender>,
) {
    let Some(table) = playfield.iter().next() else {
        return;
    };
    let win = window_size(&windows);

    let mut hands: BTreeMap<u8, Vec<Card>> = BTreeMap::new();
    for (player, player_num) in players.iter() {
        if player_num.0 == 1 {
            let mut hand = player.cards.clone();
            hand.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| suit_order(&a.0).cmp(&suit_order(&b.0))));
            hands.insert(player_num.0, hand);
        }
    }

    for (player, player_num) in players.iter() {
        let num = player_num.0;
        let count = player.cards.len();
        let stale = render.counts.get(&num) != Some(&count) || render.win != Some(win);
        if !stale {
            continue;
        }

        for (entity, owner) in owned.iter_mut() {
            if owner.0 == num {
                commands.entity(entity).despawn();
            }
        }

        let positions = fan_layout(num, count, win);
        let hand = hands.get(&num);
        for (i, pos) in positions.iter().enumerate() {
            let (x, y, rot) = *pos;
            let (cw, ch) = if num == 1 {
                (CARD_W, CARD_H)
            } else {
                (CPU_CARD_W, CPU_CARD_H)
            };
            let node = Node {
                position_type: PositionType::Absolute,
                width: Val::Px(cw),
                height: Val::Px(ch),
                left: Val::Px(x),
                top: Val::Px(y),
                margin: UiRect::new(
                    Val::Px(-cw / 2.),
                    Val::Auto,
                    Val::Px(-ch / 2.),
                    Val::Auto,
                ),
                ..Default::default()
            };
            let transform = UiTransform {
                rotation: Rot2::radians(rot.to_radians()),
                ..Default::default()
            };
            if num == 1 {
                let card = hand
                    .map(|hand| hand[i].clone())
                    .expect("Expected the dealt human card");
                commands.spawn((
                    node,
                    ImageNode {
                        image: asset_server.load(image_name(&card)),
                        ..Default::default()
                    },
                    transform,
                    ZIndex(i as i32 + 1),
                    Button,
                    HandCard { card },
                    FanOwner(num),
                    ChildOf(table),
                ));
            } else {
                commands.spawn((
                    node,
                    ImageNode {
                        image: asset_server.load(card_back_path(num)),
                        ..Default::default()
                    },
                    transform,
                    ZIndex(i as i32 + 1),
                    FanOwner(num),
                    ChildOf(table),
                ));
            }
        }

        render.counts.insert(num, count);
    }

    if render.win != Some(win) {
        render.win = Some(win);
    }
}

/// Animates cards flying into the center pile and despawns them on arrival.
fn animate_flying(
    mut commands: Commands,
    mut flying: Query<(Entity, &mut Node, &mut FlyingCard)>,
    time: Res<Time>,
) {
    for (entity, mut node, mut card) in flying.iter_mut() {
        card.elapsed += time.delta_secs();
        let t = (card.elapsed / FLY_TIME).clamp(0., 1.);
        let eased = t * t * (3. - 2. * t);
        let at = card.start.lerp(card.end, eased);
        node.left = Val::Px(at.x);
        node.top = Val::Px(at.y);
        if t >= 1. {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_flying(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    table: Option<Entity>,
    start: Vec2,
    end: Vec2,
    card: &Card,
) {
    let Some(table) = table else {
        return;
    };
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(CARD_W),
            height: Val::Px(CARD_H),
            left: Val::Px(start.x),
            top: Val::Px(start.y),
            margin: UiRect::new(
                Val::Px(-CARD_W / 2.),
                Val::Auto,
                Val::Px(-CARD_H / 2.),
                Val::Auto,
            ),
            ..Default::default()
        },
        ImageNode {
            image: asset_server.load(image_name(card)),
            ..Default::default()
        },
        ZIndex(50),
        FlyingCard {
            start,
            end,
            elapsed: 0.,
        },
        ChildOf(table),
    ));
}

fn update_pile_view(
    game: Res<Game>,
    mut piles: Query<&mut ImageNode, With<PileCard>>,
    asset_server: Res<AssetServer>,
) {
    for mut image in piles.iter_mut() {
        match &game.trick_path {
            Some(path) => image.image = asset_server.load(path.clone()),
            None => image.image = Handle::default(),
        }
    }
}

/// Dims hand cards that cannot legally beat the current pile.
fn tint_hand(
    state: Res<State<GameState>>,
    game: Res<Game>,
    players: Query<(&Player, &PlayerNum)>,
    mut cards: Query<(&mut ImageNode, &HandCard)>,
) {
    let human_turn = *state.get() == GameState::Turn && game.active_player == 1;
    let counts: BTreeMap<u8, u8> = players
        .iter()
        .filter(|(_, player_num)| player_num.0 == 1)
        .flat_map(|(player, _)| player.cards.iter().map(|card| card.2))
        .fold(BTreeMap::new(), |mut map: BTreeMap<u8, u8>, value| {
            *map.entry(value).or_default() += 1;
            map
        });

    for (mut image, hand_card) in cards.iter_mut() {
        let legal = human_turn
            && match game.trick_rank {
                None => true,
                Some(rank) => {
                    let value = hand_card.card.2;
                    let size = game.trick_size.unwrap_or(0);
                    value > rank && counts.get(&value).copied().unwrap_or(0) >= size
                }
            };
        let shade = if legal { 1. } else { 0.5 };
        image.color = Color::srgb(shade, shade, shade);
    }
}

fn hand_clicks(
    state: Res<State<GameState>>,
    game: Res<Game>,
    cards: Query<&HandCard>,
    mut clicks: MessageReader<Pointer<Click>>,
    mut intent: ResMut<HumanIntent>,
) {
    if *state.get() != GameState::Turn || game.active_player != 1 {
        return;
    }
    for click in clicks.read() {
        if let Ok(hand_card) = cards.get(click.entity) {
            intent.action = Some(HumanAction::Play(hand_card.card.clone()));
        }
    }
}

fn pass_clicks(
    state: Res<State<GameState>>,
    game: Res<Game>,
    buttons: Query<(), With<PassButton>>,
    mut clicks: MessageReader<Pointer<Click>>,
    mut intent: ResMut<HumanIntent>,
) {
    if *state.get() != GameState::Turn || game.active_player != 1 {
        return;
    }
    for click in clicks.read() {
        if buttons.get(click.entity).is_ok() {
            intent.action = Some(HumanAction::Pass);
        }
    }
}

fn holders_of(players: &Query<(&mut Player, &PlayerNum)>) -> Vec<u8> {
    let mut nums: Vec<u8> = players
        .iter()
        .filter(|(player, _)| !player.cards.is_empty())
        .map(|(_, player_num)| player_num.0)
        .collect();
    nums.sort();
    nums
}

fn rotate_turn(game: &mut Game, players: &Query<(&mut Player, &PlayerNum)>) {
    let nums = holders_of(players);
    let active = game.active_player;
    game.active_player = *nums
        .iter()
        .find(|num| **num > active)
        .unwrap_or_else(|| nums.first().expect("Expected a holder"));
}

fn apply_play(
    game: &mut Game,
    players: &mut Query<(&mut Player, &PlayerNum)>,
    played: Vec<Card>,
) -> bool {
    let active = game.active_player;
    let rank = played[0].2;
    let mut eliminated = false;
    for (mut player, player_num) in players.iter_mut() {
        if player_num.0 != active {
            continue;
        }
        for card in &played {
            let index = player
                .cards
                .iter()
                .position(|held| held.0 == card.0 && held.1 == card.1)
                .expect("Expected the played card in hand");
            player.cards.remove(index);
        }
        if player.cards.is_empty() {
            game.finish_order.push(active);
            eliminated = true;
        }
    }
    game.trick_rank = Some(rank);
    game.trick_size = Some(played.len() as u8);
    game.last_player = Some(active);
    game.passes = 0;
    game.trick_path = Some(image_name(&played[0]));
    game.message = format!(
        "{} played {}",
        player_name(active),
        play_description(played.len() as u8, rank)
    );
    if eliminated {
        game.message = format!("{} - {} are out!", game.message, player_name(active));
    }
    eliminated
}

fn apply_pass(game: &mut Game, players: &Query<(&mut Player, &PlayerNum)>) {
    let active = game.active_player;
    game.passes += 1;
    game.message = format!("{} passed", player_name(active));

    let nums = holders_of(players);
    let last_holds_cards = game.last_player.is_some_and(|last| nums.contains(&last));
    let required = nums.len() - if last_holds_cards { 1 } else { 0 };

    if game.passes as usize >= required {
        game.trick_rank = None;
        game.trick_size = None;
        game.passes = 0;
        game.trick_path = None;
        let lead = match game.last_player {
            Some(last) if nums.contains(&last) => last,
            _ => nums.first().copied().unwrap_or(active),
        };
        game.active_player = lead;
        game.message = format!("Trick over - {} leads", player_name(lead));
    } else {
        rotate_turn(game, players);
    }
}

fn ai_choice(game: &Game, players: &Query<(&mut Player, &PlayerNum)>) -> Option<(u8, u8)> {
    let mut counts: BTreeMap<u8, u8> = BTreeMap::new();
    for (player, player_num) in players.iter() {
        if player_num.0 == game.active_player {
            for card in &player.cards {
                *counts.entry(card.2).or_default() += 1;
            }
        }
    }

    let mut options: Vec<(u8, u8)> = Vec::new();
    match game.trick_rank {
        None => {
            for (rank, count) in &counts {
                let max_size = (*count).min(4);
                for size in 1..=max_size {
                    options.push((*rank, size));
                }
            }
        }
        Some(trick_rank) => {
            let trick_size = game.trick_size.unwrap();
            for (rank, count) in &counts {
                if *count >= trick_size && *rank > trick_rank {
                    options.push((*rank, trick_size));
                }
            }
        }
    }
    options
        .iter()
        .min_by_key(|&(rank, size)| (rank, 5 - size))
        .copied()
}

fn finalize_round(
    game: &mut Game,
    players: &Query<(&mut Player, &PlayerNum)>,
    next_state: &mut NextState<GameState>,
) {
    let mut order = game.finish_order.clone();
    let mut remaining: Vec<(u8, usize)> = players
        .iter()
        .filter(|(player, _)| !player.cards.is_empty())
        .map(|(player, player_num)| (player_num.0, player.cards.len()))
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
        .map(|(num, rank)| format!("{} {}", player_name(*num), rank))
        .join(", ");
    game.message = format!("Round {} over: {}", game.round_number, ranks);
    next_state.set(GameState::RoundEnd);
}

fn game_flow(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut cpu: ResMut<CpuTurn>,
    mut intent: ResMut<HumanIntent>,
    mut game: ResMut<Game>,
    mut players: Query<(&mut Player, &PlayerNum)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    playfield: Query<Entity, With<PlayfieldRoot>>,
    windows: Query<&Window>,
) {
    let current_state = *state.get();

    match current_state {
        GameState::Begin | GameState::RoundEnd => {
            if keys.just_pressed(KeyCode::Space) {
                next_state.set(GameState::Setup);
            }
        }
        GameState::Setup => {}
        GameState::Turn => {
            let nums = holders_of(&players);
            if nums.len() <= 1 {
                finalize_round(&mut game, &players, &mut next_state);
                return;
            }

            let active = game.active_player;
            if active == 1 {
                let Some(action) = intent.action.take() else {
                    return;
                };
                cpu.delay = CPU_DELAY;
                match action {
                    HumanAction::Pass => {
                        apply_pass(&mut game, &players);
                    }
                    HumanAction::Play(clicked) => {
                        let trick_open = game.trick_rank.is_none();
                        let trick_rank = game.trick_rank.unwrap_or(0);
                        let trick_size = game.trick_size.unwrap_or(1);
                        let held: Vec<Card> = players
                            .iter()
                            .find(|(_, player_num)| player_num.0 == 1)
                            .map(|(player, _)| player.cards.clone())
                            .unwrap_or_default();

                        if !held.iter().any(|card| *card == clicked) {
                            game.message = format!("You cannot play the {}", display_card(&clicked));
                            return;
                        }

                        let played: Vec<Card> = if trick_open {
                            vec![clicked.clone()]
                        } else {
                            held.iter()
                                .filter(|card| card.2 == clicked.2)
                                .take(trick_size as usize)
                                .cloned()
                                .collect()
                        };

                        let legal = trick_open
                            || (played.len() == trick_size as usize
                                && clicked.2 > trick_rank
                                && held
                                    .iter()
                                    .filter(|card| card.2 == clicked.2)
                                    .count() as u8
                                        >= trick_size);
                        if !legal {
                            game.message = format!(
                                "The {} cannot beat {}",
                                display_card(&clicked),
                                play_description(trick_size, trick_rank)
                            );
                            return;
                        }

                        let eliminated = apply_play(&mut game, &mut players, played.clone());
                        cpu.delay = CPU_DELAY;
                        spawn_flying(
                            &mut commands,
                            &asset_server,
                            playfield.iter().next(),
                            seat_point(1, window_size(&windows)),
                            Vec2::new(
                                window_size(&windows).x * 0.5,
                                window_size(&windows).y * 0.5,
                            ),
                            &played[0],
                        );

                        if eliminated {
                            let nums = holders_of(&players);
                            if nums.len() <= 1 {
                                finalize_round(&mut game, &players, &mut next_state);
                                return;
                            }
                        }
                        rotate_turn(&mut game, &players);
                    }
                }
            } else {
                if cpu.delay > 0 {
                    cpu.delay -= 1;
                    return;
                }
                cpu.delay = CPU_DELAY;

                match ai_choice(&game, &players) {
                    Some((rank, size)) => {
                        let held: Vec<Card> = players
                            .iter()
                            .find(|(_, player_num)| player_num.0 == active)
                            .map(|(player, _)| player.cards.clone())
                            .unwrap_or_default();
                        let played: Vec<Card> = held
                            .iter()
                            .filter(|card| card.2 == rank)
                            .take(size as usize)
                            .cloned()
                            .collect();

                        let eliminated = apply_play(&mut game, &mut players, played.clone());
                        spawn_flying(
                            &mut commands,
                            &asset_server,
                            playfield.iter().next(),
                            seat_point(active, window_size(&windows)),
                            Vec2::new(
                                window_size(&windows).x * 0.5,
                                window_size(&windows).y * 0.5,
                            ),
                            &played[0],
                        );

                        if eliminated {
                            let nums = holders_of(&players);
                            if nums.len() <= 1 {
                                finalize_round(&mut game, &players, &mut next_state);
                                return;
                            }
                        }
                        rotate_turn(&mut game, &players);
                    }
                    None => {
                        apply_pass(&mut game, &players);
                    }
                }
            }
        }
    }

    keys.clear_just_pressed(KeyCode::Space);
}

fn setup_round(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    mut cpu: ResMut<CpuTurn>,
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
                    "Scum ({}) gave {} to King; King returned the same cards. ",
                    player_name(scum_num),
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
                    "Vice Scum ({}) surrendered their best card to Queen; Queen returned it. ",
                    player_name(vice_scum_num)
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
    game.trick_path = None;
    game.active_player = lead;
    cpu.delay = CPU_DELAY / 2;
    if narration.is_empty() {
        game.message = format!(
            "Round {} - {} leads",
            game.round_number,
            player_name(lead)
        );
    } else {
        game.message = format!(
            "{} | Round {} - {} leads",
            narration.trim_end(),
            game.round_number,
            player_name(lead)
        );
    }

    next_state.set(GameState::Turn);
}

fn update_card_count(
    players: Query<(&Player, &PlayerNum)>,
    mut card_texts: Query<(&mut Text, &PlayerNum), With<PlayerCardNum>>,
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
        .map(|(num, rank)| {
            (
                *num,
                format!("{} ({})", player_name(*num), rank),
            )
        })
        .collect();
    for (mut text, player_num) in names.iter_mut() {
        let label = labels
            .get(&player_num.0)
            .cloned()
            .unwrap_or_else(|| player_name(player_num.0));
        text.0 = label;
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
            if game.active_player == 1 {
                format!(
                    "Round {} | Your turn ({}) - click a card or PASS | {}",
                    game.round_number, trick, game.message
                )
            } else {
                format!(
                    "Round {} | {} thinking ({}) | {}",
                    game.round_number,
                    player_name(game.active_player),
                    trick,
                    game.message
                )
            }
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
        .init_resource::<CpuTurn>()
        .init_resource::<HumanIntent>()
        .init_resource::<FanRender>()
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            (
                hand_clicks,
                pass_clicks,
                game_flow,
                sync_fans,
                animate_flying,
                update_pile_view,
                tint_hand,
                update_card_count,
                update_names,
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
    use bevy::camera::NormalizedRenderTarget;
    use bevy::picking::{
        backend::HitData,
        events::{Click, Pointer},
        pointer::{Location, PointerId},
    };
    use bevy::scene::ScenePlugin;
    use bevy::state::app::StatesPlugin;
    use std::time::Duration;

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
            .init_resource::<CpuTurn>()
            .init_resource::<HumanIntent>()
            .init_resource::<FanRender>()
            .add_message::<Pointer<Click>>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    hand_clicks,
                    pass_clicks,
                    game_flow,
                    sync_fans,
                    animate_flying,
                    update_pile_view,
                    tint_hand,
                    update_card_count,
                    update_names,
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

    fn wait_for_turn(app: &mut App) {
        let mut guard = 0;
        while game_state(app) != GameState::Turn && guard < 120 {
            app.update();
            guard += 1;
        }
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

    fn count_entities<M: Component>(app: &mut App) -> usize {
        let world = app.world_mut();
        let query = world.query_filtered::<(), With<M>>();
        query.iter_manual(world).count()
    }

    fn active_player(app: &App) -> u8 {
        app.world().resource::<Game>().active_player
    }

    /// Drives one CPU turn to completion.
    fn cpu_act(app: &mut App) {
        assert_ne!(active_player(app), 1, "expected a CPU turn");
        app.world_mut().resource_mut::<CpuTurn>().delay = 0;
        app.update();
    }

    /// Drives one human turn with the given action.
    fn human_act(app: &mut App, action: HumanAction) {
        assert_eq!(active_player(app), 1, "expected a human turn");
        app.world_mut().resource_mut::<HumanIntent>().action = Some(action);
        app.update();
    }

    fn lowest_in_human_hand(app: &mut App) -> Card {
        let world = app.world_mut();
        let query = world.query::<(&PlayerNum, &Player)>();
        let mut best: Option<Card> = None;
        for (player_num, player) in query.iter_manual(world) {
            if player_num.0 != 1 {
                continue;
            }
            for card in &player.cards {
                let key = (card.2, suit_order(&card.0));
                let better = match &best {
                    None => true,
                    Some(best) => key < (best.2, suit_order(&best.0)),
                };
                if better {
                    best = Some(card.clone());
                }
            }
        }
        best.expect("Expected human cards")
    }

    /// The human plays the lowest card when leading and passes otherwise.
    fn human_take_turn(app: &mut App) {
        let trick_open = app.world().resource::<Game>().trick_rank.is_none();
        let action = if trick_open {
            HumanAction::Play(lowest_in_human_hand(app))
        } else {
            HumanAction::Pass
        };
        human_act(app, action);
    }

    fn card(suit: &str, face: &str, value: u8) -> Card {
        Card(String::from(suit), String::from(face), value)
    }

    fn advance_setup_to_turn(mut next_state: ResMut<NextState<GameState>>) {
        next_state.set(GameState::Turn);
    }

    fn make_logic_app(hands: [Vec<Card>; 4]) -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            StatesPlugin,
        ));
        app.init_resource::<ButtonInput<KeyCode>>()
            .init_asset::<Image>()
            .init_resource::<Game>()
            .init_resource::<CpuTurn>()
            .init_resource::<HumanIntent>()
            .add_systems(Update, game_flow)
            .add_systems(OnEnter(GameState::Setup), advance_setup_to_turn)
            .init_state::<GameState>();
        app.update();
        for (index, hand) in hands.into_iter().enumerate() {
            let num = index + 1;
            app.world_mut().spawn((
                Player { cards: hand },
                PlayerNum(num as u8),
            ));
        }
        // Enter Turn through Setup; transitions may cascade one per update.
        press_space(&mut app);
        wait_for_turn(&mut app);
        assert_eq!(game_state(&app), GameState::Turn);
        app
    }

    #[test]
    fn deals_and_plays_multiple_rounds() {
        let mut app = make_app();

        // The UI scene (players, legend pile marker and status) must spawn first.
        for _ in 0..120 {
            if count_entities::<Player>(&mut app) == 4 && count_entities::<StatusText>(&mut app) == 1
            {
                break;
            }
            app.update();
        }
        assert_eq!(count_entities::<Player>(&mut app), 4);
        assert_eq!(count_entities::<StatusText>(&mut app), 1);

        // Start the game: the round is dealt and a lead is chosen.
        press_space(&mut app);
        wait_for_turn(&mut app);
        assert_eq!(game_state(&mut app), GameState::Turn);
        let sizes = hand_sizes(&mut app);
        assert_eq!(sizes.len(), 4);
        assert_eq!(sizes.iter().sum::<usize>(), 52);
        assert!(sizes.iter().all(|size| *size == 13));

        // The face-up hand fan renders exactly the human's cards.
        assert_eq!(count_entities::<HandCard>(&mut app), 13);

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

        // Play until the round ends: CPUs act on their own timer, the human
        // leads with the lowest card and passes otherwise.
        let mut turns = 0;
        while game_state(&app) != GameState::RoundEnd && turns < 4000 {
            if active_player(&app) == 1 {
                human_take_turn(&mut app);
            } else {
                cpu_act(&mut app);
            }
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
        wait_for_turn(&mut app);
        let game = app.world().resource::<Game>();
        assert_eq!(game.round_number, 2);
        assert_eq!(game.active_player, scum_num);
        let sizes = hand_sizes(&mut app);
        assert_eq!(sizes.iter().sum::<usize>(), 52);
        assert!(sizes.iter().all(|size| *size == 13));
        assert_eq!(count_entities::<HandCard>(&mut app), 13);

        // Play a handful of turns in round 2 and confirm progress.
        for _ in 0..10 {
            if game_state(&app) == GameState::Turn {
                if active_player(&app) == 1 {
                    human_take_turn(&mut app);
                } else {
                    cpu_act(&mut app);
                }
            }
        }
        assert!(!app.world().resource::<Game>().message.is_empty());
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

        // Clicking one of the sixes plays the pair of Sixes (lowest legal rank).
        human_act(&mut app, HumanAction::Play(card("Hearts", "6", 6)));
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(6));
            assert_eq!(game.trick_size, Some(2));
            assert_eq!(game.passes, 0);
            assert_eq!(game.active_player, 2);
            assert!(game.message.contains("You played a pair of Sixes"));
        }

        // CPU 1 follows with a pair of Sevens; CPU 2 with a pair of Kings.
        cpu_act(&mut app);
        assert_eq!(app.world().resource::<Game>().trick_rank, Some(7));
        cpu_act(&mut app);

        // CPU 2 played their last cards and is out.
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(13));
            assert_eq!(game.finish_order, vec![3]);
            assert!(game.message.contains("CPU 2 are out"));
        }

        // CPU 3 cannot follow a pair with one card and passes.
        cpu_act(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.passes, 1);
            assert!(game.message.contains("CPU 3 passed"));
            // Turn rotates to the lowest-numbered remaining holder.
            assert_eq!(game.active_player, 1);
        }

        // The human passes; CPU 1 also passes and with the last winner gone the
        // trick resets so the human leads instead of the eliminated CPU 2.
        human_act(&mut app, HumanAction::Pass);
        cpu_act(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert!(game.trick_rank.is_none());
            assert!(game.trick_size.is_none());
            assert_eq!(game.passes, 0);
            assert_eq!(game.active_player, 1);
            assert!(game.message.contains("Trick over - You lead"));
        }

        // The trick contents were discarded.
        assert!(app.world().resource::<Game>().trick_path.is_none());

        // Leading with the lone Ace empties the human hand.
        human_act(&mut app, HumanAction::Play(card("Spades", "A", 14)));
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(14));
            assert_eq!(game.trick_size, Some(1));
            assert_eq!(game.finish_order, vec![3, 1]);
            assert!(game.message.contains("You are out"));
        }

        // Nobody can beat the Ace: CPU 1 and CPU 3 pass, and since the human no
        // longer holds cards the trick resets with the lowest-numbered holder
        // (CPU 1) leading.
        cpu_act(&mut app);
        assert_eq!(app.world().resource::<Game>().passes, 1);
        cpu_act(&mut app);
        {
            let game = app.world().resource::<Game>();
            assert!(game.trick_rank.is_none());
            assert_eq!(game.active_player, 2);
            assert!(game.message.contains("Trick over - CPU 1 leads"));
        }

        // CPU 1 leads with their lone Three and goes out; only CPU 3 holds
        // cards, so the round ends and ranks are assigned by finish order.
        cpu_act(&mut app);
        wait_for_turn(&mut app);
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

        // Clicking the lowest card opens the trick with a single.
        human_act(&mut app, HumanAction::Play(card("Clubs", "5", 5)));
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(5));
            assert_eq!(game.trick_size, Some(1));
        }
    }

    #[test]
    fn illegal_click_is_rejected() {
        let mut app = make_logic_app([
            vec![card("Hearts", "6", 6), card("Spades", "7", 7)],
            vec![card("Clubs", "9", 9)],
            vec![card("Clubs", "10", 10)],
            vec![card("Clubs", "J", 11)],
        ]);

        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.trick_rank = Some(8);
            game.trick_size = Some(2);
            game.last_player = Some(2);
            game.active_player = 1;
        }

        // A lone seven cannot beat a pair, so the click is rejected and the
        // turn stays with the human.
        human_act(&mut app, HumanAction::Play(card("Spades", "7", 7)));
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.trick_rank, Some(8));
            assert_eq!(game.active_player, 1);
            assert!(game.message.contains("cannot beat"));
        }

        // Passing is always allowed.
        human_act(&mut app, HumanAction::Pass);
        {
            let game = app.world().resource::<Game>();
            assert_eq!(game.passes, 1);
            assert!(game.message.contains("You passed"));
        }
    }

    #[test]
    fn cpu_turns_run_without_input() {
        let mut app = make_logic_app([
            vec![card("Hearts", "6", 6), card("Spades", "6", 6)],
            vec![card("Clubs", "9", 9)],
            vec![card("Clubs", "10", 10)],
            vec![card("Clubs", "J", 11)],
        ]);

        {
            let mut game = app.world_mut().resource_mut::<Game>();
            game.active_player = 2;
            game.trick_rank = None;
            game.trick_size = None;
        }

        // Without any input the CPU counts down and eventually acts.
        let mut guard = 0;
        while app.world().resource::<Game>().trick_rank.is_none() && guard < 200 {
            app.update();
            guard += 1;
        }
        assert!(app.world().resource::<Game>().trick_rank.is_some());
        assert_eq!(app.world().resource::<Game>().active_player, 3);
    }

    fn find_entity<M: Component>(app: &mut App) -> Option<Entity> {
        let world = app.world_mut();
        let query = world.query_filtered::<Entity, With<M>>();
        query.iter_manual(world).next()
    }

    fn wait_for_entity<M: Component>(app: &mut App) -> Entity {
        let mut guard = 0;
        loop {
            if let Some(entity) = find_entity::<M>(app) {
                return entity;
            }
            app.update();
            guard += 1;
            assert!(guard < 120, "Expected the entity to spawn");
        }
    }

    fn child_of(app: &mut App, parent: Entity) -> Option<Entity> {
        let world = app.world_mut();
        let query = world.query::<(Entity, &ChildOf)>();
        query
            .iter_manual(world)
            .find(|(_, child)| child.parent() == parent)
            .map(|(entity, _)| entity)
    }

    fn click_entity(app: &mut App, entity: Entity) {
        let hit = HitData::new(Entity::PLACEHOLDER, 0., None, None);
        let click = Pointer::new(
            PointerId::Mouse,
            Location {
                target: NormalizedRenderTarget::None {
                    width: 1,
                    height: 1,
                },
                position: Vec2::ZERO,
            },
            Click {
                button: PointerButton::Primary,
                hit,
                duration: Duration::ZERO,
                count: 1,
            },
            entity,
        );
        app.world_mut()
            .resource_mut::<Messages<Pointer<Click>>>()
            .write(click);
    }

    #[test]
    fn pass_button_label_does_not_swallow_clicks() {
        let mut app = make_app();
        let button = wait_for_entity::<PassButton>(&mut app);

        // The button label sits above the button in the picking order, so it
        // must be ignored by picking or clicks on the text never reach the
        // button entity.
        let label = child_of(&mut app, button).expect("Expected the PASS label");
        let pickable = app.world().get::<Pickable>(label).expect("Expected the label to be explicitly non-pickable");
        assert!(!pickable.is_hoverable);
        assert!(!pickable.should_block_lower);

        press_space(&mut app);
        wait_for_turn(&mut app);
        app.world_mut().resource_mut::<Game>().active_player = 1;

        // A pointer click targeting the button entity registers as a pass.
        click_entity(&mut app, button);
        app.update();

        let game = app.world().resource::<Game>();
        assert_eq!(game.passes, 1);
        assert!(game.message.contains("You passed"));
    }
}
