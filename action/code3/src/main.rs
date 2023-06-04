use std::str::FromStr;
use std::time::SystemTime;

use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

/// 玩家
#[derive(Debug, Clone)]
pub struct Player {
    id: i32,
    // 卡牌
    cards: Vec<Card>,
    // 是否是地主
    landlord: bool,
}

impl Player {
    pub fn new(id: i32, cards: Vec<Card>, landlord: bool) -> Self {
        Self {
            id,
            cards,
            landlord,
        }
    }

    pub fn new_with_id(id: i32) -> Self {
        Self {
            id,
            cards: vec![],
            landlord: false,
        }
    }

    pub fn is_landlord(&self) -> bool {
        self.landlord
    }

    pub fn set_landlord(&mut self, landlord: bool) {
        self.landlord = landlord;
    }

    pub fn get_cards(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn set_cards(&mut self, cards: Vec<Card>) {
        self.cards = cards;
    }

    pub fn add_card(&mut self, card: Card) {
        self.add_card_and_sort(card, false);
    }

    pub fn add_card_and_sort(&mut self, card: Card, sort: bool) {
        self.cards.push(card);
        if sort {
            self.sort();
        }
    }

    pub fn add_cards(&mut self, cards: &mut Vec<Card>) {
        self.add_cards_and_sort(cards, false);
    }

    pub fn add_cards_and_sort(&mut self, cards: &mut Vec<Card>, sort: bool) {
        self.cards.append(cards);
        if sort {
            self.sort();
        }
    }

    /// 排序
    pub fn sort(&mut self) {
        self.cards.sort_by_key(|item| { -item.get_priority() });
    }
}

/// 扑克牌
#[derive(Debug, Clone)]
pub struct Card {
    // 花色
    suit: &'static str,
    // 数字
    number: &'static str,
    // priority
    priority: i32,
}

impl Card {
    pub fn new(suit: &'static str, number: &'static str) -> Self {
        let priority = Self::generate_priority(number);
        Card {
            suit,
            number,
            priority,
        }
    }

    pub fn get_card_string(&self) -> String {
        format!("{}{}", self.suit, self.number)
    }

    pub fn get_priority(&self) -> i32 {
        self.priority
    }

    /// 优先级的数字越大，牌面就越大
    fn generate_priority(number: &'static str) -> i32 {
        match number {
            "X" => {
                10
            }
            "J" => {
                11
            }
            "Q" => {
                12
            }
            "K" => {
                13
            }
            "A" => {
                14
            }
            "2" => {
                15
            }
            "B" => {
                16
            }
            "R" => {
                17
            }
            _ => number.parse::<i32>().unwrap_or_else(|_| 0)
        }
    }
}


fn main() {
    let mut cards = create_cards();

    // 创建玩家
    let mut cpu_player1 = Player::new_with_id(1);
    let mut cpu_player2 = Player::new_with_id(2);
    let mut player = Player::new_with_id(3);

    // 发牌
    let mut under_cards = deal_cards(&mut cards, &mut cpu_player1, &mut cpu_player2, &mut player);
    print_cards(&mut cpu_player1, &mut cpu_player2, &mut player);

    // 征询地主
    ask_landlord(&mut cpu_player1, &mut cpu_player2, &mut player);
    println!("cpu_player1: {}, cpu_player2: {}, player: {}", cpu_player1.is_landlord(), cpu_player2.is_landlord(), player.is_landlord());
    // 展示底牌
    show_under_cards(&under_cards);
    // 给地主发底牌
    landlord_get_under_cards(&mut under_cards, &mut cpu_player1, &mut cpu_player2, &mut player);

    print_cards(&mut cpu_player1, &mut cpu_player2, &mut player);
}

/// 1. 创建扑克牌
fn create_cards() -> Vec<Card> {
    let suits = vec!["♣", "♠", "♥", "♦"];
    let numbers = vec!["A", "2", "3", "4", "5", "6", "7", "8", "9", "X", "J", "Q", "K"];

    let mut vec: Vec<Card> = Vec::with_capacity(54);
    for suit in suits.iter() {
        for number in numbers.iter() {
            vec.push(Card::new(suit, number));
        }
    }
    // 黑色的 Joker 牌
    vec.push(Card::new("", "B"));
    // 彩色的 Joker 牌
    vec.push(Card::new("", "R"));
    vec
}

/// 2. 发牌
fn deal_cards(cards: &mut Vec<Card>,
              player1: &mut Player,
              player2: &mut Player,
              player3: &mut Player) -> Vec<Card> {

    // 洗牌
    shuffle_cards(cards);

    let mut under_cards: Vec<Card> = Vec::with_capacity(3);
    for index in 0..cards.len() {
        if index >= 51 {
            under_cards.push(cards[index].clone());
        } else if index % 3 == 0 {
            player1.add_card(cards[index].clone());
        } else if index % 3 == 1 {
            player2.add_card(cards[index].clone());
        } else if index % 3 == 2 {
            player3.add_card(cards[index].clone());
        }
    }
    // 排序
    player1.sort();
    player2.sort();
    player3.sort();
    under_cards
}

/// 3.叫地主
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

/// 获取输入的字符串
fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    return input;
}

// 洗牌
fn shuffle_cards(cards: &mut Vec<Card>) {
    cards.shuffle(&mut thread_rng());
}

fn print_cards(player1: &mut Player,
               player2: &mut Player,
               main_player: &mut Player) {
    if player1.landlord {
        println!("玩家{}(地主)的牌面：", player1.id);
    } else {
        println!("玩家{}的牌面：", player1.id);
    }
    for card in player1.get_cards().iter() {
        print!("{} ", card.get_card_string());
    }

    if player2.landlord {
        println!("\n玩家{}(地主)的牌面：", player2.id);
    } else {
        println!("\n玩家{}的牌面：", player2.id);
    }
    for card in player2.get_cards().iter() {
        print!("{} ", card.get_card_string());
    }
    if main_player.landlord {
        println!("\n我(地主)的牌面：");
    } else {
        println!("\n我的牌面：");
    }
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

/// 自定义parse
#[test]
fn test_from_str() {
    #[derive(Debug)]
    struct Example {
        name: String,
    }

    impl FromStr for Example {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Example { name: s.to_string() })
        }
    }

    let name = "ZhangSan";
    let ex = name.parse::<Example>().unwrap();
    println!("{:?}", ex);
}

#[test]
fn test_rand() {
    // 随机生成布尔值
    let b = rand::random::<bool>();
    println!("rand bool: {}", b);

    // 随机生成unicode字符，生成的字符可能不可见
    let c = rand::random::<char>();
    println!("rand char: {}", c);

    // 随机生成 i32 值
    let i = rand::random::<i32>();
    println!("rand i32: {}", i);

    // 生成范围值
    let r = thread_rng().gen_range(1..10);
    println!("rand rang: {}", r);

    // 打乱顺序
    let mut arr: Vec<i32> = (1..10).collect();
    arr.shuffle(&mut rand::thread_rng());
    println!("rand vec: {:?}", arr);
}

#[test]
fn test_shuffle() {
    let mut arr: Vec<i32> = (1..10).collect();
    shuffle(&mut arr);
    println!("rand vec: {:?}", arr);

    // 乱序
    fn shuffle(arr: &mut Vec<i32>) {
        for i in (1..arr.len()).rev() {
            // 获取当前的时间
            let now = SystemTime::now();
            let time_ms = now.duration_since(std::time::UNIX_EPOCH).expect("");
            // 生成要交换的随机位置 j
            let j = time_ms.as_millis() % (i as u128 + 1);
            // 交换第i和第j的位置
            arr.swap(i, j as usize);
        }
    }
}


