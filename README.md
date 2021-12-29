# credit_card_validator
**A brute-force implementation of luhn's algorithm implemented in rust made for generating credit card numbers and ccv's**

This is my first rust library which I'm using to learn the language, my design choices and implementation of the code could be significantly improved upon.

At the current state of this project most of the major features such as ccv, expiration date, and random name generation(not sure if I'm going to include this) are not implemented

### Usage

this snipit of code shows the infinite card generation implementation witch uses for loops to do so, might implement it as 0.. syntax in rust once I research and learn about that portion of the language.
```rust
fn main() {
    let mut gen = generator::CardGenerator::new(generator::Config::default().set_length(13));
    for (i, el) in gen.enumerate() {
        println!("card {}: {}", i, el);
    }
}
```

### Todo
- [ ] Optimization of Algorithm
- [ ] Expiration Date Generation
- [ ] File Output


### Done
- [x] Number Generation
- [x] Simple infinite loop for generation
- [x] Different types of Card
  - [x] Visa
  - [x] Mastercard
  - [x] Discovery
  - [x] American Express
- [x] CCV Generation
- [x] Default Values for length of each type of card