use clap::Parser;
use crate::card::CardType;
use crate::generator::Config;

pub mod generator;
pub mod card;

#[derive(Parser, Debug)]
#[clap(name = "hello")]
struct Arguments {
    ///required options(VISA(4), MASTER(5), AM(34, 37)=American Express, DISCOVER(6))
    #[clap(short, long)]
    ctype: String,
    ///amount to be generated
    #[clap(short, long, default_value = "1")]
    amount: i64,
    //card length<Check online for valid lengths for selected type>
    #[clap(short, long)]
    length: i64,
    //should it be outputed to a file<<not implemented yet>>
    #[clap(short, long)]
    file_output: bool
}

fn main() {
   // let args = Arguments::parse();
    let mut gen = generator::CardGenerator::new(generator::Config::default().set_length(13));
    for (i, el) in gen.enumerate() {
        println!("card {}: {}", i, el);
    }
  //  gen.generate_type(card::CardType::from_string(&args.ctype).unwrap(), args.amount, args.length);
   // gen.output(generator::GeneratorOutput::STD);
}
