//! A contrived example showing how you can use the `stateful_native_library`
//! bindings.

use stateful_native_library::{Error, Library};

fn main() -> Result<(), Error> {
    let mut library = Library::new()?;

    // set some parameters
    library
        .set_parameters()
        .boolean("foo", false)
        .integer("some-number", 42);

    // start building the recipe
    let mut recipe_builder = library.create_recipe();
    recipe_builder.add_item("first", 1).add_item("second", 2);

    // we can add several groups using a loop
    for i in 0..5 {
        let name = format!("group_{}", i);
        let mut group_builder = recipe_builder.add_group(&name);

        for j in 0..i {
            let name = format!("group_{}_item_{}", i, j);
            group_builder.add_item(&name, i + j);
        }

        group_builder.finish();
    }

    // or use normal builder methods
    recipe_builder
        .add_group("another group")
        .add_item("another nested item", 5)
        .add_item("MOAR items", 6);

    // finish building the recipe
    let recipe = recipe_builder.build();

    // then get the outcome, periodically printing out progress messages
    let outcome = stateful_native_library::execute(recipe, |percent| {
        println!("{}%", percent)
    })?;

    println!("Got {:?}", outcome);

    Ok(())
}
