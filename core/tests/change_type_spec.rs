use autorel_core::ChangeType;

#[test]
fn type_ord_is_from_smallest_to_biggest_scope() {
    let expected = vec![
        None,
        Some(ChangeType::Fix),
        Some(ChangeType::Feature),
        Some(ChangeType::Breaking),
    ];

    let mut actual = expected.clone();
    actual.sort();

    assert_eq!(expected, actual);
}
