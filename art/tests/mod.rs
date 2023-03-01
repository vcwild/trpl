use art::mix;
use art::PrimaryColor;

use art::kinds::SecondaryColor;

#[test]
fn primary_colors_mix_to_secondary() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    assert_eq!(mix(red, yellow), SecondaryColor::Orange);
}
