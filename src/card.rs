use rand::Rng;
use rand::rngs::ThreadRng;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum CardType {
    Visa(i64),
    Master,
    AmericanExpress(AmericanExpressCard),
    Discover,
    Custom(i64, i64, i64),
}

#[derive(Clone)]
pub enum AmericanExpressCard {
    T4,
    T7,
}

impl CardType {
    pub fn from_string(v: &String) -> Result<CardType, String> {
        match v.to_lowercase().as_ref() {
            "visa" | "v" => {
                Ok(CardType::Visa(16))
            }
            "master" | "m" => {
                Ok(CardType::Master)
            }
            "american_express_4" | "americanexpress_4" | "american_4" | "am_4" => {
                Ok(CardType::AmericanExpress(AmericanExpressCard::T4))
            }
            "american_express_7" | "americanexpress_7" | "american_7" | "am_7" => {
                Ok(CardType::AmericanExpress(AmericanExpressCard::T7))
            }
            "discover" | "d" => {
                Ok(CardType::Discover)
            }
            _ => {
                let mut v2 = v.split(':');
                let first = v2.next().unwrap();
                let second = v2.next().unwrap();
                if first.parse::<i64>().is_ok() && second.parse::<i64>().is_ok() {
                    Ok(CardType::Custom(first.parse::<i64>().unwrap(), second.parse::<i64>().unwrap(), 16))
                } else {
                    Err("Invalid Card Type".to_string())
                }
            }
        }
    }

    fn int_val(&self) -> i64 {
        match self {
            CardType::Visa(_) => 4,
            CardType::Master => 51,
            CardType::AmericanExpress(am) => {
                match am {
                    AmericanExpressCard::T4 => 34,
                    AmericanExpressCard::T7 => 37
                }
            }
            CardType::Discover => 6,
            CardType::Custom(v, _, _) => v.clone()
        }
    }

    fn get_number_length(&self) -> i64 {
        match self {
            CardType::Master | CardType::Discover => 16,
            CardType::AmericanExpress(_) => 15,
            CardType::Visa(length) => length.clone(),
            CardType::Custom(_, _, length) => length.clone()
        }
    }
}

pub struct Card {
    ccv: Option<i64>,
    number: Option<i64>,
    ctype: CardType,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut type_name = String::new();
        match self.ctype.clone() {
            CardType::Visa(_) => type_name = String::from("Visa"),
            CardType::Master => type_name = String::from("Master"),
            CardType::AmericanExpress(t) => {
                match t {
                    AmericanExpressCard::T4 => type_name = String::from("American Express v4"),
                    AmericanExpressCard::T7 => type_name = String::from("American Express v7")
                }
            }
            CardType::Discover => type_name = String::from("discover"),
            CardType::Custom(i, i2, _) => type_name = format!("custom({}, ccv_length({}))", i, i2)
        }
        if let Some(i) = self.number {
            if let Some(c) = self.ccv {
                write!(f, "type: {}, number: {}, ccv: {}", type_name, i, c)
            }else{
                write!(f, "type: {}, number: {}, ccv: None", type_name, i)
            }
        } else {
            if let Some(c) = self.ccv {
                write!(f, "type: {}, number: {}, ccv: {}", type_name, "None Generated", c)
            }else{
                write!(f, "type: {}, number: {}, ccv: {}", type_name, "None_Generated", "None_Generated")
            }
        }
    }
}

impl Card {
    pub fn from(card: CardType) -> Card {
        Card {ccv: None, ctype: card, number: None }
    }

    pub fn is_valid(&self) -> Result<bool, String> {
        let mut num: i64 = 0;
        if let Some(i) = self.number {
            num = i;
        } else {
            return Err("num is not set".to_string());
        }
        let mut num: i64 = num.clone();
        let mut new_num: i64 = 0;
        let mut add_num: i64 = 0;
        let mut first = true;
        let mut temp_num: i64 = 0;
        while num > 0 {
            temp_num *= 10;
            temp_num += num % 10;
            num = num / 10;
        }
        if temp_num % 10 == 3 {
            if (temp_num / 10) % 10 == 4 || (temp_num / 10) % 10 == 7 {
                first = false;
            }
        }
        num = temp_num;
        while num > 0 {
            add_num = num % 10;
            num /= 10;
            if first {
                add_num *= 2;
                if add_num > 9 {
                    add_num = add_num / 10 + (add_num % 10);
                }
            }
            first = !first;
            new_num += add_num;
        }
        return Ok(new_num % 10 == 0);
    }


    pub fn generate_ccv(&mut self, rng: &mut ThreadRng){
        match &self.ctype {
            CardType::Visa(_) | CardType::Master | CardType::Discover => self.ccv = Some(rng.gen_range(100..1000)),
            CardType::AmericanExpress(i) => self.ccv = Some(rng.gen_range(1000..10000)),
            CardType::Custom(_, i, _) => {
                let mut max: i64 = 10;
                let t = i.clone();
                for _ in 0..t-1{
                    max *= 10;
                }
                self.ccv = Some(rng.gen_range(max/10..max));
            }
        }
    }

    pub fn generate_number(&mut self, rng: &mut ThreadRng) {
        let mut t = self.ctype.clone().int_val();
        let glength = self.ctype.get_number_length() - numlength(self.ctype.clone().int_val());
        let mut beg = 1;
        for _ in 0..glength - 1 {
            beg *= 10;
        }
        t *= beg * 10;
        let mut seed = rng.gen_range(beg..beg * 10);
        let mut v = t + seed;
        self.number = Some(v);
        while !self.is_valid().unwrap() {
            seed = rng.gen_range(beg..beg * 10);
            v = t + seed;
            self.number = Some(v);
        }
    }
}


fn numlength(n: i64) -> i64 {
    let mut n = n;
    if n == 0 {
        return 1;
    }
    let mut l: i64 = 0;

    while n > 0 {
        n = n / 10;
        l += 1;
    }
    return l;
}
