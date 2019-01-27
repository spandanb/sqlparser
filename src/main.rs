use sqlparser::parse_sql;

fn main() {
    // let stmnt = "CREATE TABLE foo { bar INT , baz TEXT  }";
    // let stmnt = "SELECT * FROM foo";
    let stmnt = "INSERT INTO foo (bar, baz) VALUES ( abcd, def )";
    let result = parse_sql(stmnt);
    println!("Parsed: {:?}", result);
}
