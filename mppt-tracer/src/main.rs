use postgres::{Client, NoTls};

// example from the rust crate docs
fn postgres_tutorial() -> Result<(), Box<dyn::std::error::Error>> {
    println!("starting postgres tutorial function");

    let mut client = Client::connect("postgresql://postgres:(password)@localhost/(localdb)", NoTls)?;
    client.batch_execute("
        CREATE TABLE person (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            data BYTEA
        )
    ")?;

    println!("Finished making table");

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    )?;
    
    for row in client.query("SELECT id, name, data FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data)
    }
    Ok(())
}

fn main() {
    println!("running main");
    postgres_tutorial().expect("something went wrong");
}
