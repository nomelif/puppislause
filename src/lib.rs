extern crate rand;
extern crate sha1;
extern crate crypto;
extern crate rand_chacha;
use rand::{SeedableRng, Rng};
use rand_chacha::ChaChaRng;
use crypto::digest::Digest;
use crypto::sha1::Sha1;



pub fn encode(seed : &str) -> String{

  let mut hasher = Sha1::new();
  hasher.input_str(seed);

  let mut seed_arr : [u8; 32] = [0; 32];
  seed_arr.copy_from_slice(&(hasher.result_str())[..32].as_bytes());

  let mut rng: ChaChaRng = SeedableRng::from_seed(seed_arr);
  let adj: Vec<&str> = include_str!("adj").trim().split("\n").collect();
  let subst: Vec<&str> = include_str!("subst").trim().split("\n").collect();
  let adjective = adj[rng.gen::<u64>() as usize % adj.len()];
  let noun1 = subst[rng.gen::<u64>() as usize % adj.len()];
  let noun2 = subst[rng.gen::<u64>() as usize % adj.len()];
  let compound = if noun1.chars().last().unwrap() == noun2.chars().next().unwrap() { format!("{}-{}", noun1, noun2) } else { format!("{}{}", noun1, noun2) };
  format!("{} {}",  adjective, compound)
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic_functionality() {
        assert_eq!("helppokäyttöinen arvokala-asiavirhe", super::encode("abcdabcdabcdabcd-abcdabcdabcdabc"));
        assert_eq!("maaliton arpanoppa-autourheilu", super::encode("From this height the blanket, just about ready to slide off  completely,  could  hardly  stay  in  place"));
    }

    #[test]
    fn hyphenation() {
        assert_eq!("aloitekykyinen hilpariherätemiina", super::encode("Poppelis puppelis pox"));
        assert_eq!("itämurteinen elastisuushaudesidos", super::encode("One  morning,  as  Gregor  Samsa  was waking  up  from anxious dreams, he discovered that in bed he had been changed into a monstrous verminous bug."));
        assert_eq!("käyrävartinen avovesibiokemia", super::encode("He lay on his armour-hard back and saw, as he lifted his head up a little, his brown, arched abdomen divided up into rigid bow-like sections."));
        assert_eq!("kummallinen hiukkassuihkugneissigraniitti", super::encode("His  numerous legs, pitifully thin in comparison to the rest of his circumference, flickered helplessly before his eyes."));
    }
}
