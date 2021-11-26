use rand::rngs::ThreadRng;
use crate::card;
use std::default;
use crate::card::CardType;


pub enum GeneratorOutput {
    STD,
    FILE(String)
}

pub struct Config {
    ctype: card::CardType,
    length: i64
}

impl default::Default for Config{
    fn default() -> Self {
        Config{ ctype: CardType::Visa, length: 15}
    }
}

impl Config {
    pub fn set_card_type(mut self, ctype: card::CardType) -> Self {
        self.ctype = ctype;
        self
    }

    pub fn set_length(mut self, length: i64) -> Self {
        self.length = length;
        self
    }
}

pub struct CardGenerator {
    cards: Vec<card::Card>,
    rng: ThreadRng,
    config: Config
}

impl Iterator for CardGenerator
{
    type Item = card::Card;

    fn next(&mut self) -> Option<Self::Item> {
        let mut card = card::Card::from(self.config.length.clone(), self.config.ctype.clone());
        card.generate_number(&mut self.rng);
        Some(card)
    }
}

impl CardGenerator {
    pub fn new(config: Config) -> CardGenerator {
        let cards: Vec<card::Card> = Vec::new();
        CardGenerator{cards, rng: rand::thread_rng(), config}
    }

    pub fn generate_type(&mut self, ctype: card::CardType, amount: i64, length: i64) {
        for _ in 0..amount {
            let mut card = card::Card::from(length, ctype.clone());
            card.generate_number(&mut self.rng);
            self.cards.push(card);
        }
    }

    pub fn output(&self, output: GeneratorOutput) {
        match output {
            GeneratorOutput::STD => {
                for (i, card) in self.cards.iter().enumerate() {
                    println!("card {}: {}", i, card);
                }
            },
            GeneratorOutput::FILE(location) => {
                //TODO IMPLEMENT
            }
        }
    }
}
