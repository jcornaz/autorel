use autorel_core::SemverScope;

#[test]
fn type_ord_is_from_smallest_to_biggest_scope() {
    let expected = vec![
        None,
        Some(SemverScope::Fix),
        Some(SemverScope::Feature),
        Some(SemverScope::Breaking),
    ];

    let mut actual = expected.clone();
    actual.sort();

    assert_eq!(expected, actual);
}
