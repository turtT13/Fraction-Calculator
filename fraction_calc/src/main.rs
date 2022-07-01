use std::io::stdin;
struct Fraction{
    numer: u32,
    denom: u32,
    whole: u32,
}
impl Fraction {
    fn new(whole:u32, numer: u32, denom: u32) -> Fraction{
        Fraction{
            whole,
            numer,
            denom,
        }
    }
    fn to_improper(&mut self){
        self.numer = self.whole * self.denom + self.numer;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input)
    	.ok()
        .expect("Failed to read line");
    
    let mut op = 'l';
    match input {
        "+" => op = '+',
        "-" => op = '-',
        "*" => op = '*',
        "/" => op = '/',
        _ => op = 'l',
    }


}

fn math(fraction1: Fraction,fraction2: Fraction, operator: char){

}