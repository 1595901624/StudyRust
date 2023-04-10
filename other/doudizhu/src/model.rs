/// 玩家
#[non_exhaustive]
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

/// 卡牌
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