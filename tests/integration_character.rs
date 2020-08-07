use rustysrlib::character::{init_metatypes, Attributes, MetaTypes};

#[test]
fn it_inits_humans_correctly() {
    let expected_min = 1;
    let expected_max = 6;

    assert_eq!(
        expected_min,
        init_metatypes()[MetaTypes::Human][Attributes::Body].min
    );

    assert_eq!(
        expected_max,
        init_metatypes()[MetaTypes::Human][Attributes::Body].max
    );
}
