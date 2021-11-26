use rand::rngs::ThreadRng;
use crate::card;

pub enum GeneratorOutput {
    STD,
    FILE(String)
}

pub trait Argumentative {}

pub struct None;
impl Argumentative for None{}

pub struct DefaultOptions {
    pub(crate) ctype: card::CardType,
    pub(crate) length: i64
}
impl Argumentative for DefaultOptions{}

pub struct CardGenerator<Argument> {
    cards: Vec<card::Card>,
    rng: ThreadRng,
    args: Argument
}

impl Iterator for CardGenerator<DefaultOptions>
{
    type Item = card::Card;

    fn next(&mut self) -> Option<Self::Item> {
        let mut card = card::Card::from(self.args.length.clone(), self.args.ctype.clone());
        card.generate_number(&mut self.rng);
        Some(card)
    }
}

impl<Argument: Argumentative> CardGenerator<Argument> {
    pub fn new(t: Argument) -> CardGenerator<Argument> {
        let cards: Vec<card::Card> = Vec::new();
        CardGenerator{cards, rng: rand::thread_rng(), args: t}
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
