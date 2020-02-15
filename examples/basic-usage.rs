//! A contrived example showing how you can use the `stateful_native_library`
//! bindings.

use stateful_native_library::{Error, Library};

fn main() -> Result<(), Error> {
    let mut library = Library::new()?;

    library
        .set_parameters()
        .boolean("foo", false)
        .integer("some-number", 42);

    let mut recipe_builder = library.create_recipe();
    recipe_builder.add_item("first", 1).add_item("second", 2);

    for i in 0..5 {
        let name = format!("group_{}", i);
        let mut group_builder = recipe_builder.add_group(&name);

        for j in 0..i {
            let name = format!("group_{}_item_{}", i, j);
            group_builder.add_item(&name, i + j);
        }

        group_builder.finish();
    }

    let mut another_group_builder = recipe_builder.add_group("another group");
    another_group_builder
        .add_item("another nested item", 5)
        .add_item("MOAR items", 6);
    another_group_builder.finish();

    let recipe = recipe_builder.build();

    let outcome = stateful_native_library::execute(recipe, |percent| {
        println!("{}%", percent)
    })?;

    println!("Got {:?}", outcome);

    Ok(())
}
