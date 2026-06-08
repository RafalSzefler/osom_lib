use osom_lib_numbers::iterators::ConstPermutationGenerator;

#[test]
fn test_permutations() {
    let mut generator = ConstPermutationGenerator::<3>::new();
    assert_eq!(generator.next(), Some([0, 1, 2]));
    assert_eq!(generator.next(), Some([0, 2, 1]));
    assert_eq!(generator.next(), Some([1, 0, 2]));
    assert_eq!(generator.next(), Some([1, 2, 0]));
    assert_eq!(generator.next(), Some([2, 0, 1]));
    assert_eq!(generator.next(), Some([2, 1, 0]));
    assert_eq!(generator.next(), None);
}
