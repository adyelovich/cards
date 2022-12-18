use crate::cards;
use rand::Rng;

pub struct Player {
    id: i32,
    hand: Vec<cards::Card>,
    score: i32,
}

impl Player {
    pub fn new(id: i32) -> Player {
        Player {
            id: id,
            hand: Vec::new(),
            score: 0,
        }
    }
}

pub struct Table {
    table: Vec<Player>,
}

impl Table {
    pub fn new(n: i32) -> Table {
        let mut v: Vec<Player> = Vec::new();
        
        for i in 0..n {
            v.push(Player::new(i));
        }

        Table {
            table: v,
        }
    }

    pub fn deal_deck(&self, deck: &cards::Deck, num_to_deal: i32) {
        let mut cards_left = num_to_deal;
        let mut cur_player = self.table.iter().cycle();

        let deck_size = deck.deck_size();


        //This should be changed to writing some method that tells deck
        //to deal a card to some player, similar to how the C version
        //has a deal_card helper function, maybe at this point we can
        //make a more general `move_state` function, since deal_card
        //would be a special instance of that anyway
        while cards_left > 0 {
            let rnum = loop {
                let r = rand::rng_thread().gen_range(0..deck_size);

                if deck.card_at(r).has_state(cards::State::Deck) {
                    return r;
                    break;
                }
            }

            cur_player.next();
            cards_left -= 1;
            
        }
    }
}
