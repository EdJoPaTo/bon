use bon::builder;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let map: BTreeMap<String, String> = bon::map! {
        "Hello": "Blackjack",
        "Hello": "Littlepip",
    };

    let set: BTreeSet<String> = bon::set!["mintals", "guns", "mintals", "roses"];

    #[builder]
    struct SkipGeneratesNoSetter {
        #[builder(skip)]
        x: u32,

        #[builder(skip = 4)]
        y: u32,
    }

    SkipGeneratesNoSetter::builder().x(42).build();
    SkipGeneratesNoSetter::builder().y(42).build();
}

#[builder]
struct TupleStruct(u32, u32);

#[builder]
fn destructuring((x, y): (u32, u32)) {
    let _ = x;
    let _ = y;
}

#[builder]
fn unnecessary_into_false(#[builder(into = false)] _x: u32) {}

#[builder(on(String, into))]
fn unnecessary_into(#[builder(into)] _x: String) {}

#[builder(on(&dyn std::fmt::Debug, into))]
fn invalid_type_pattern() {}

#[builder(on(fn(#[attr] a: u32), into))]
fn attrs_in_on_type_pattern() {}

#[builder(on)]
fn incomplete_on() {}

#[builder(on())]
fn incomplete_on2() {}

#[builder(on(_))]
fn incomplete_on3() {}

#[builder(on(_,))]
fn incomplete_on4() {}

#[builder(start_fn())]
struct EmptyStartFn {}

#[builder]
struct ConflictingAttrs {
    #[builder(skip, into)]
    x: u32,
}

#[builder]
struct ConflictingAttrs2 {
    #[builder(skip, name = bar)]
    x: u32,
}

#[builder]
struct ConflictingAttrs3 {
    #[builder(skip, default = 42)]
    z: u32,
}

#[builder]
fn skip_on_fn_is_unsupporetd(
    #[builder(skip)] _x: u32,
    #[builder(skip = "skip".to_owned())] _y: String,
    #[builder(skip = vec![42])] _z: Vec<u32>,
) {
}

#[builder]
struct TupleStructsAreUnsupported(u32, u32);

#[builder]
enum EnumsAreUnsupported {}

#[builder]
fn destructuring_in_fn_is_unsupported((_, _): (u32, u32)) {}
