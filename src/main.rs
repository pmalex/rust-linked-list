use linked_list::one_linked_list::List;

fn main() {
    let mut list = List::<i32>::new();

    list.push_front(13);
    list.push_front(-7);

    println!("{:?}", list);
}
