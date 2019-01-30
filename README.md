# Variation
A procedural macro to generate enum to variant conversion methods.

## Methods generated

#### `is_*` methods
An `is_variant` method is generated for each for variant in an enum.

```rust
use variation::Variation;

#[derive(Variation)]
enum Type {
    Unit,
    Integer(i32),
}

fn main() {
    let return_type = Type::Unit;

    assert!(return_type.is_unit());
    assert!(!return_type.is_integer());
}
```

#### `as_*` & `as_*_mut` methods
Variants that have one or more inner types have `as` and `as_mut` allowing you
to get a immutable or mutable reference to the inner types. Variants with a
single inner type will return `&{mut} T`. Variants that have more than one inner
type will return a tuple with a reference to each type.

```rust
use variation::Variation;

#[derive(Variation)]
enum Type {
    Unit,
    Integer(i32),
    Real(i32, u32),
}

fn main() {
    let mut return_type = Type::Integer(5);
    let real_value = Type::Real(3, 14);

    assert_eq!(Some(&mut 5), return_type.as_integer_mut());
    assert_eq!(Some((&3, &14)), real_value.as_real());
    assert_eq!(None, real_value.as_integer());
}
```

#### `into_*` methods
Variants that have one or more inner types have an `into` method, allowing you
to attempt to convert a enum into its inner values. This method will panic when
called on a variant that does not match the method.

```rust
use variation::Variation;

#[derive(Variation)]
enum Type {
    Unit,
    Integer(i32),
    Real(i32, u32),
}

fn main() {
    let mut return_type = Type::Integer(5);
    let real_value = Type::Real(3, 14);
    let unit = Type::Unit;

    assert_eq!(5, return_type.into_integer());
    assert_eq!((3, 14), real_value.into_real());
    // Panics
    unit.into_integer();

}
```
