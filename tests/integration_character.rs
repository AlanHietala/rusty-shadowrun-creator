
use rustysrlib::character::{init_metatypes, MetaTypes, Attributes};


#[test]
fn it_inits_humans_correctly() {
    let expected_min = 1;
    let expected_max = 6;

    assert_eq!(expected_min, rustysrlib::character::init_metatypes()[MetaTypes::Human][Attributes::Body].min);
}