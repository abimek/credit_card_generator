#[cfg(test)]
pub mod tests{

    use assert_matches::assert_matches;

    use std::ops::Deref;
    use crate::*;
    use assert_impl::assert_impl;

    #[test]
    fn gen_infinite_iteration(){
        assert_impl!(std::iter::Iterator: generator::CardGenerator);
    }

    #[test]
    fn gen_generate_amount(){
        let mut gen = crate::generator::CardGenerator::new(crate::generator::Config::default());
        gen.generate_type(crate::card::CardType::Master, 64);
        assert_eq!(gen.get_cards().len(), 64);
    }

    #[test]
    fn card_generate_number() {
        let mut card = card::Card::from(card::CardType::Visa(17));
        card.generate_number(&mut rand::thread_rng());
        assert_matches!(card.get_number(), Some(_));
    }

    #[test]
    fn card_generate_ccv() {
        let mut card = card::Card::from(card::CardType::Visa(17));
        card.generate_ccv(&mut rand::thread_rng());
        assert_matches!(card.get_ccv(), Some(_))
    }

    #[test]
    fn card_valid(){
        let mut gen = crate::generator::CardGenerator::new(crate::generator::Config::default());
        gen.generate_type(crate::card::CardType::Master, 64);
        for card in gen.get_cards() {
            assert_matches!(card.is_valid(), Ok(true))
        }
    }
}