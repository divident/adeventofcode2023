use std::{fs, cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandScore {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

static ORDER: &'static [char] = &['A', 'K', 'Q',  'T', '9','8', '7', '6', '5', '4', '3', '2', 'J'];


#[derive(Debug, Ord)]
struct Hand {
    cards: String,
    bid: u32,
    score: HandScore
}

impl Hand {
    pub fn new(cards: String, bid: u32) -> Self {

        let mut cards_count: HashMap<char, i32> = HashMap::new();
        for c in cards.chars().into_iter() {
           cards_count.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
        
        let mut score =HandScore::HighCard;
        let mut score_vec: Vec<&i32> = cards_count.iter().filter(|(k, v)| **k != 'J').map(|(k, v)| v).collect();

        score_vec.sort();
        score_vec.reverse();

        let j_count = cards_count.get(&'J').unwrap_or(&0);
        for v in score_vec.into_iter(){
            if *v == 5 {
                score = HandScore::FiveOfKind;
                break;
            } else if *v == 4 {
                score = HandScore::FourOfKind;
                break;
            } else if *v == 3 {
                score = HandScore::ThreeOfKind;
            } else if score == HandScore::ThreeOfKind && *v == 2 {
                score = HandScore::FullHouse;
                break;
            } else if score == HandScore::OnePair && *v == 3 {
                score = HandScore::FullHouse;
                break;
            } else if *v == 2 && score == HandScore::OnePair {
                score = HandScore::TwoPair;
                break;
            }  else if *v == 2 {
                score = HandScore::OnePair;
            }
        }

        if score == HandScore::FourOfKind && *j_count > 0{
            score = HandScore::FiveOfKind;
        } else if score == HandScore::ThreeOfKind {
            if *j_count == 1 {
                score = HandScore::FourOfKind;
            } else if  *j_count == 2 {
               score = HandScore::FiveOfKind; 
            }
        } else if score == HandScore::TwoPair {
            if *j_count == 1 {
                score = HandScore::FullHouse;
            } 
        } else if score == HandScore::OnePair {
            if * j_count == 1 {
                score = HandScore::ThreeOfKind;
            }
            else if *j_count == 2 {
                score = HandScore::FourOfKind;
            } else if *j_count == 3 {
                score = HandScore::FiveOfKind;
            }
        } else if score == HandScore::HighCard {
            if *j_count == 1 {
                score = HandScore::OnePair;
            } else if *j_count == 2 {
                score = HandScore::ThreeOfKind;
            } else if *j_count == 3 {
                score = HandScore::FourOfKind;
            }
            else if *j_count ==4 {
                score = HandScore::FiveOfKind;
            }
        }

        if *j_count == 5 || *j_count == 4{
            score = HandScore::FiveOfKind;
        } 
         

        return Hand { cards: cards, bid: bid, score: score };

    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
//score=249506014 too low
//score=249722254 too high
// score=249620106
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score == other.score {

            if self.eq(other) {
                return Some(Ordering::Equal);
            } else {
                for (a, b) in self.cards.chars().into_iter().zip(other.cards.chars().into_iter()).into_iter() {
                    let a_idx = ORDER.into_iter().position(|x| *x == a).unwrap();
                    let b_idx = ORDER.into_iter().position(|x| *x == b).unwrap();

                    if a_idx > b_idx {
                        return Some(Ordering::Greater);
                    } else if a_idx < b_idx {
                        return Some(Ordering::Less);
                    }
                }
                return Some(Ordering::Equal);
            }
        } else {
            if self.score > other.score {
                return Some(Ordering::Greater);
            } else {
               return Some(Ordering::Less) ;
            }
        }



    }
}

impl Eq for Hand {}

fn main() {

    let content = fs::read_to_string("data/input2.txt").expect("File not found");

    let lines = content.split("\n");

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines.into_iter() {
        let data: Vec<&str> = line.split(" ").collect();

        hands.push(
            Hand::new(
                String::from(data[0]),
                data[1].parse::<u32>().expect("Invalid num {data[1]}")
            )
        );    
    }
    hands.sort();
    let mut score: u64 = 0;
    let mut rank:u64 = hands.len() as u64;
    for hand in hands.iter() {
        //print!("bid={} * rank={rank}\n", hand.bid);
        score = score + (hand.bid as u64 * rank);
        rank -= 1;
    }
    for hand in hands.iter() {
        println!("{:?}", hand);
    }
    print!("score={score}\n");

}
