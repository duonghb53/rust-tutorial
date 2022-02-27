pub struct Allergies {
    pub score: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score > (*allergen as u32).into()
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score / 256;
        if score == 0 {
            score = self.score;
        } else if score > 1 && self.score % 256 == 0 {
            score = 256;
        } else {
            score = self.score % 256;
        }
        let mut vec = Vec::new();
        let mut allergies = Allergies::new(score);

        while allergies.score >= 1 {
            if self.is_allergic_to(&Allergen::Cats) {
                vec.push(Allergen::Cats);
                allergies = Allergies::new(self.score - 128);
            } else if self.is_allergic_to(&Allergen::Pollen) {
                vec.push(Allergen::Pollen);
                allergies = Allergies::new(self.score - 64);
            } else if self.is_allergic_to(&Allergen::Chocolate) {
                vec.push(Allergen::Chocolate);
                allergies = Allergies::new(self.score - 32);
            } else if self.is_allergic_to(&Allergen::Tomatoes) {
                vec.push(Allergen::Tomatoes);
                allergies = Allergies::new(self.score - 16);
            } else if self.is_allergic_to(&Allergen::Strawberries) {
                vec.push(Allergen::Strawberries);
                allergies = Allergies::new(self.score - 8);
            } else if self.is_allergic_to(&Allergen::Shellfish) {
                vec.push(Allergen::Shellfish);
                allergies = Allergies::new(self.score - 4);
            } else if self.is_allergic_to(&Allergen::Peanuts) {
                vec.push(Allergen::Peanuts);
                allergies = Allergies::new(self.score - 2);
            } else if self.is_allergic_to(&Allergen::Eggs) {
                vec.push(Allergen::Eggs);
                allergies = Allergies::new(self.score - 1);
            }
        }

        vec
    }
}

fn main() {

}

#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn check(){
        is_not_allergic_to_anything();
        is_allergic_to_eggs();
    }
}


use Allergen::*;
pub fn compare_allergy_vectors(expected: &[Allergen], actual: &[Allergen]) {
    for element in expected {
        if !actual.contains(element) {
            panic!(
                "Allergen missing\n  {:?} should be in {:?}",
                element, actual
            );
        }
    }
    if actual.len() != expected.len() {
        panic!(
            "Allergy vectors are of different lengths\n  expected {:?}\n  got {:?}",
            expected, actual
        );
    }
}
#[test]
pub fn is_not_allergic_to_anything() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Peanuts));
    assert!(!allergies.is_allergic_to(&Allergen::Cats));
    assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
}
#[test]
#[ignore]
fn is_allergic_to_eggs() {
    assert!(Allergies::new(1).is_allergic_to(&Allergen::Eggs));
}
#[test]
#[ignore]
pub fn is_allergic_to_egg_shellfish_and_strawberries() {
    let allergies = Allergies::new(5);
    assert!(allergies.is_allergic_to(&Allergen::Eggs));
    assert!(allergies.is_allergic_to(&Allergen::Shellfish));
    assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
}
#[test]
#[ignore]
fn no_allergies_at_all() {
    let expected = &[];
    let allergies = Allergies::new(0).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]
#[ignore]
fn allergic_to_just_eggs() {
    let expected = &[Allergen::Eggs];
    let allergies = Allergies::new(1).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]
#[ignore]
fn allergic_to_just_peanuts() {
    let expected = &[Allergen::Peanuts];
    let allergies = Allergies::new(2).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]
#[ignore]
fn allergic_to_just_strawberries() {
    let expected = &[Allergen::Strawberries];
    let allergies = Allergies::new(8).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]
#[ignore]
fn allergic_to_eggs_and_peanuts() {
    let expected = &[Allergen::Eggs, Allergen::Peanuts];
    let allergies = Allergies::new(3).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]
#[ignore]
fn allergic_to_eggs_and_shellfish() {
    let expected = &[Allergen::Eggs, Allergen::Shellfish];
    let allergies = Allergies::new(5).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]
#[ignore]
fn allergic_to_many_things() {
    let expected = &[
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(248).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]
#[ignore]
fn allergic_to_everything() {
    let expected = &[
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(255).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]
#[ignore]
fn scores_over_255_do_not_trigger_false_positives() {
    let expected = &[
        Allergen::Eggs,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(509).allergies();
    compare_allergy_vectors(expected, &allergies);
}
