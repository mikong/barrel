extern crate barrel;

use barrel::{backend::SqlVariant, types, Migration, Table};

fn main() {
    let mut m = Migration::new();

    m.create_table("users", |t: &mut Table| {
        t.add_column("name", types::varchar(255));
        t.add_column("owns_plushy_sharks", types::boolean());
    });

    /// This requires "pg" to be selected as a feature at compile-time
    println!("{:?}", m.make_from(SqlVariant::Pg));
}
