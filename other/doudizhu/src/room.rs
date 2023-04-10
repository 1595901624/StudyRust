use crate::model::{Card, Player};

/// 房间
#[derive(Debug)]
pub struct Room<'a,'b> {
    // 参与的玩家
    players: Vec<&'a Player>,

    // 当前出牌玩家的id
    current_player_id: Option<i32>,

    // 当前出牌的玩家
    current_player: Option<&'a Player>,

    // 已经经过的轮次
    turn: i32,

    // 已经打出的牌堆
    used_cards: Vec<&'b Card>,
}

impl<'a,'b> Room<'a,'b> {
    pub fn new(vec: Vec<&'a Player>) -> Self {
        Self {
            players: vec,
            current_player_id: None,
            current_player: None,
            turn: 0,
            used_cards: vec![],
        }
    }
}

impl<'a,'b> Default for Room<'a,'b> {
    fn default() -> Self {
        Self {
            players: vec![],
            current_player_id: None,
            current_player: None,
            turn: 0,
            used_cards: vec![],
        }
    }
}