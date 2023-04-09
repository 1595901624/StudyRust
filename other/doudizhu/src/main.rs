use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::model::{Card, Player};

mod model;

fn main() {
    let cards = create_cards();

    // 创建玩家
    let mut cpu_player1 = Player::default();
    let mut cpu_player2 = Player::default();
    let mut player = Player::default();

    // 发牌
    let mut under_cards = deal_cards(cards, &mut cpu_player1, &mut cpu_player2, &mut player);
    print_cards(&mut cpu_player1, &mut cpu_player2, &mut player);

    // 展示底牌
    show_under_cards(&under_cards);

    // 征询地主
    ask_landlord(&mut cpu_player1, &mut cpu_player2, &mut player);
    println!("cpu_player1: {}, cpu_player2: {}, player: {}", cpu_player1.is_landlord(), cpu_player2.is_landlord(), player.is_landlord());

    // 给地主发底牌
    landlord_get_under_cards(&mut under_cards, &mut cpu_player1, &mut cpu_player2, &mut player);

    print_cards(&mut cpu_player1, &mut cpu_player2, &mut player);
}

/// 1. 创建扑克牌
fn create_cards() -> Vec<Card> {
    // let suits = vec!["︎♣️", "♠️", "❤️", "♦️"];
    let suits = vec!["♣", "♠", "♥", "♦"];
    let numbers = vec!["A", "2", "3", "4", "5", "6", "7", "8", "9", "X", "J", "Q", "K"];

    let mut vec: Vec<Card> = Vec::with_capacity(54);
    for suit in suits.iter() {
        for number in numbers.iter() {
            vec.push(Card::new(suit, number));
        }
    }
    // Black Joker
    vec.push(Card::new("", "B"));
    // Red Joker
    vec.push(Card::new("", "R"));

    // shuffle
    shuffle_cards(&mut vec);
    vec
}

/// 2. 发牌
fn deal_cards(cards: Vec<Card>,
              player1: &mut Player,
              player2: &mut Player,
              player3: &mut Player) -> Vec<Card> {
    let mut under_cards: Vec<Card> = Vec::with_capacity(3);
    for index in 0..cards.len() {
        if index >= 51 {
            under_cards.push(cards[index].to_owned());
        } else if index % 3 == 0 {
            player1.add_card(cards[index].to_owned());
        } else if index % 3 == 1 {
            player2.add_card(cards[index].to_owned());
        } else if index % 3 == 2 {
            player3.add_card(cards[index].to_owned());
        }
    }
    player1.sort();
    player2.sort();
    player3.sort();
    // player1.get_cards().sort_by_key(|item| { -item.get_priority() });
    // player2.get_cards().sort_by_key(|item| { -item.get_priority() });
    // player3.get_cards().sort_by_key(|item| { -item.get_priority() });
    // player1.set_cards();
    under_cards
}

/// 3.询问玩家是否当地主
fn ask_landlord(player1: &mut Player,
                player2: &mut Player,
                main_player: &mut Player) {
    // 优先询问玩家
    loop {
        println!("你是否要当地主？Y:是,N:否");
        let input = get_input();
        if input.trim() == "Y" || input.trim() == "y" {
            main_player.set_landlord(true);
            break;
        } else if input.trim() == "N" || input.trim() == "n" {
            main_player.set_landlord(false);
            break;
        } else {
            println!("输入内容无效，请重新输入！！");
        }
    };

    if !main_player.is_landlord() {
        // 玩家不是地主
        player1.set_landlord(rand::random::<bool>());

        // 这里偷懒，强制设置最后一个玩家是地主
        if !player1.is_landlord() {
            player2.set_landlord(true);
        }
    }
}

/// 4.向地主发底牌
fn landlord_get_under_cards(cards: &mut Vec<Card>,
                            player1: &mut Player,
                            player2: &mut Player,
                            player3: &mut Player) {
    if player1.is_landlord() {
        player1.add_cards_and_sort(cards, true);
        return;
    }

    if player2.is_landlord() {
        player2.add_cards_and_sort(cards, true);
        return;
    }

    if player3.is_landlord() {
        player3.add_cards_and_sort(cards, true);
        return;
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    return input;
}

// shuffle
fn shuffle_cards(cards: &mut Vec<Card>) {
    cards.shuffle(&mut thread_rng());
}

fn print_cards(player1: &mut Player,
               player2: &mut Player,
               main_player: &mut Player) {
    println!("玩家1的牌面：");
    for card in player1.get_cards().iter() {
        print!("{} ", card.get_card_string());
    }
    println!("\n玩家2的牌面：");
    for card in player2.get_cards().iter() {
        print!("{} ", card.get_card_string());
    }
    println!("\n我的牌面：");
    for card in main_player.get_cards().iter() {
        print!("{} ", card.get_card_string());
    }
    println!();
}

/// 展示底牌
fn show_under_cards(cards: &Vec<Card>) {
    println!("\n底牌：");
    for card in cards {
        print!("{} ", card.get_card_string());
    }
    println!();
}
