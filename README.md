# credit_card_validator
**A brute-force implementation luhn's algorithm implemented in rust made for generating credit card numbers that and ccv's**

At the current state of this project most of the major features such as ccv, expiration date, and random name generation(not sure if I'm going to include this) are not implemented

Also, the project is currently not a library and will be converted to one once most of the major features are finished

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
- [ ] CCV Generation
- [ ] Optimization of Algorithm
- [ ] Expiration Date Generation
- [ ] 0.. syntax for infinite looop (Possible)


### Done
- [x] Number Generation
- [x] Simple infinite loop for generation
- [x] Different types of Card
  - [x] Visa
  - [x] Mastercard
  - [x] Discovery
  - [x] American Express