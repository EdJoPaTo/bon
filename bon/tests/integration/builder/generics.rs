use crate::prelude::*;

#[test]
fn generic_struct() {
    #[builder]
    #[derive(Debug)]
    #[allow(unused)]
    struct Sut<'a, 'b, T, U, const N: usize> {
        a: &'a str,
        b: &'b str,
        c: T,
        d: U,
        e: [u8; N],
    }

    let actual = Sut::builder().a("a").b("b").c(42).d("d").e([0; 3]).build();

    assert_debug_eq(
        actual,
        expect![[r#"Sut { a: "a", b: "b", c: 42, d: "d", e: [0, 0, 0] }"#]],
    );
}

#[test]
fn return_type_only_generic_param() {
    #[builder]
    fn sut<T: Default>() -> T {
        T::default()
    }

    let _: i32 = sut().call();
}

#[test]
fn unsized_generics_in_params() {
    #[builder]
    fn sut<T: ?Sized>(arg: &T) {
        let _ = arg;
    }

    sut().arg(&42).call();
}

#[test]
fn unsized_generics_in_return_type() {
    #[builder]
    fn sut<T: ?Sized>(arg: &T) {
        let _ = arg;
    }

    sut().arg(&42).call();
}

// This is based on the issue https://github.com/rust-lang/rust/issues/129701
#[test]
fn assoc_types_work_in_params() {
    trait Trait {
        type Assoc;
    }

    #[builder]
    fn sut<T: Trait>(_val: T::Assoc) {}

    impl Trait for () {
        type Assoc = ();
    }

    sut::<()>().val(()).call();
}

// This is based on the issue https://github.com/rust-lang/rust/issues/129701
#[test]
fn assoc_types_work_in_return_type() {
    trait Trait {
        type Assoc;
    }

    #[builder]
    fn sut<T: Trait>() -> T::Assoc
    where
        T::Assoc: Default,
    {
        T::Assoc::default()
    }

    impl Trait for () {
        type Assoc = ();
    }

    sut::<()>().call();
}

// This is based on the issue https://github.com/elastio/bon/issues/16
#[test]
fn self_only_generic_param() {
    struct Sut<'a, 'b: 'a, T> {
        bar: Option<T>,
        str: &'a str,
        other_ref: &'b (),
    }

    #[bon]
    impl<T> Sut<'_, '_, T> {
        #[builder]
        fn new() -> Self {
            Self {
                bar: None,
                str: "littlepip",
                other_ref: &(),
            }
        }
    }

    // Make sure `new` method is hidden
    Sut::<core::convert::Infallible>::__orig_new();

    // Make sure the builder type name matches the type of builder when
    // `#[builder]` is placed on top of a struct
    let _: SutBuilder<'_, '_, core::convert::Infallible> = Sut::builder();

    let actual = Sut::<core::convert::Infallible>::builder().build();

    assert!(actual.bar.is_none());
    assert_eq!(actual.str, "littlepip");
    let () = actual.other_ref;
}

#[test]
fn impl_block_with_self_in_const_generics() {
    #[derive(Default)]
    struct Sut<const N: usize>;

    impl<const N: usize> Sut<N> {
        const fn val(&self) -> usize {
            42
        }
    }

    #[bon]
    impl Sut<{ Sut::<3>.val() }>
    where
        Self:,
    {
        #[builder]
        fn method(self) -> usize {
            self.val()
        }
    }

    assert_eq!(Sut::<42>.method().call(), 42);
}

#[test]
fn generics_with_lifetimes() {
    #[builder]
    fn sut<T>(arg: &&&&&T) {
        let _ = arg;
    }

    sut().arg(&&&&&&&&&&42).call();
}
