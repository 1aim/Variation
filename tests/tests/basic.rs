use variation::Variation;

#[derive(Variation)]
enum Foo {
    Bar,
    Baz,
    Point(i32, i32),
}

#[test]
fn is_implementation() {
    assert!(Foo::Bar.is_bar());
    assert!(Foo::Baz.is_baz());
    assert!(Foo::Point(0, 0).is_point());
}

#[derive(Variation)]
enum Type {
    String(String),
    Number(usize),
    Bool(bool),
    Tuple(String, bool)
}

#[test]
fn as_implementation() {
    assert!(Type::Bool(true).as_bool().unwrap());
    assert_eq!(Some(&5), Type::Number(5).as_number());
    assert_eq!(None, Type::String(String::new()).as_number());
    assert_eq!(Some((&String::new(), &true)), Type::Tuple(String::new(), true).as_tuple());
}

#[test]
fn as_mut_implementation() {
    let mut b = Type::Bool(false);

    {
        let ref_b = b.as_bool_mut().unwrap();
        *ref_b = true;
    }

    assert!(*b.as_bool_mut().unwrap());
}

#[test]
fn into_implementation() {
    let b = Type::Bool(true);
    let num = Type::Number(5);

    assert!(b.into_bool());
    assert_eq!(5, num.into_number());

}
