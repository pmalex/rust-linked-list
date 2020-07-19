use linked_list::one_linked_list::List;

extern crate test;

#[test]
fn push_pop_front() {
    let mut list = List::<u32>::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_front(4);
    list.push_front(5);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn peek_front() {
    let mut list = List::<u32>::new();

    assert_eq!(list.peek_front(), None);
    assert_eq!(list.peek_front_mut(), None);
    list.push_front(1); list.push_front(2); list.push_front(3);

    assert_eq!(list.peek_front(), Some(&3));
    assert_eq!(list.peek_front_mut(), Some(&mut 3));

    list.peek_front_mut().map(|value| {
        *value = 42
    });

    assert_eq!(list.peek_front(), Some(&42));
    assert_eq!(list.pop_front(), Some(42));
}

#[bench]
fn bench_add_two(b: &mut Bencher) {
    let mut list = List::<u32>::new();

    b.iter(|| {
        list.push_front(1); list.push_front(2); list.push_front(3);
    });
}