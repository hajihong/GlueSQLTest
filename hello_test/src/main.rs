use gluesql::prelude::*;

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "DROP TABLE IF EXISTS Glue;",
        "CREATE TABLE Glue (id INTEGER, name TEXT);",
        "INSERT INTO Glue VALUES (100, \"HAJIHONG\");",
        "INSERT INTO Glue VALUES (200, \"KIMKYEUNGMIN\");",
        "SELECT * FROM Glue WHERE id > 100;",
        "DELETE FROM Glue WHERE id = 200;",
        "ALTER TABLE Glue RENAME TO Glue2;",
        "ALTER TABLE Glue2 RENAME COLUMN id TO username;",
        "INSERT INTO Glue2 VALUES (300, \"HAJIWON\");",
        "SELECT * FROM Glue2",
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}