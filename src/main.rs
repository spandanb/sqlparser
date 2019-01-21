use core::str::FromStr;
use std::string::ParseError;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "sql.pest"]
pub struct SQLParser;


#[cfg(test)]
mod tests {
    // this imports everything in parent scope, i.e. `parse_sql` into current scope
    use super::*;

    #[test]
    fn test_parse_sql_1() {
        let stmnt = "CREATE TABLE foo { bar INT , baz TEXT  }";
        let res = parse_sql(stmnt);
        assert_eq!(res.table_name, "foo");
        assert_eq!(res.column_defs[0].column_name, "bar");
        assert_eq!(res.column_defs[0].column_type, SqlType::Int);
        assert_eq!(res.column_defs[1].column_name, "baz");
        assert_eq!(res.column_defs[1].column_type, SqlType::Text);
    }
}

#[derive(Debug)]
enum SqlType {
    Int,
    Text
}

impl FromStr for SqlType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "INT" {
            return Ok(SqlType::Int);
        } else if s == "TEXT" {
            return Ok(SqlType::Text);
        } else {
            unreachable!();
        }
    }
}

impl PartialEq for SqlType {
    fn eq(&self, other: &SqlType) -> bool {
        match (&self, other) {
            (SqlType::Int, SqlType::Int) => true,
            (SqlType::Text, SqlType::Text) => true,
            (_,_) => false,
        }
    }

}

#[derive(Debug)]
struct ColumnDef {
    column_name: String,
    column_type: SqlType
}

#[derive(Debug)]
struct CreateTable {
    table_name: String,
    column_defs: Vec<ColumnDef>,
}

#[derive(Debug)]
struct SelectStmnt {
    table_name: String,
    columns: String,
}

#[derive(Debug)]
enum ParsedStmnt {
    // The CreateTable operation and the corresponding
    // create table data
    CreateTable(CreateTable),
    SelectStmnt(SelectStmnt)
}



fn parse_sql(stmnt: &str) -> CreateTable {

    let mut create_table = CreateTable{
        table_name: String::from(""),
        column_defs: Vec::new()
    };

    let mut column_name : Option<&str> = None;

    let create_table_stmnt = SQLParser::parse(Rule::sql_grammar, stmnt)
    .expect("successful parse") // unwrap the parse result
    .next().unwrap(); // get and unwrap the `file` rule; never fails

    for create_table_child in create_table_stmnt.into_inner()
    .flatten() {
        match create_table_child.as_rule() {
            Rule::create_kw => {

            }
            Rule::table_name => {
                let table_name = create_table_child.as_str();
                create_table.table_name = String::from(table_name)
            },
            Rule::column_name => {
                column_name = Some(create_table_child.as_str());
            },
            Rule::column_type => {
                // ordering ensures column_name is set
                let column_def = ColumnDef {
                    column_name: String::from(column_name.unwrap()),
                    column_type: SqlType::from_str(create_table_child.as_str())
                                    .expect("successfully parsed sql-type"),
                };
                create_table.column_defs.push(column_def);
            }
            _ => (),
        }
    }
    println!("Created: {:?}", create_table);
    create_table
}


fn main() {
    //let stmnt = "CREATE TABLE foo { bar INT , baz TEXT  }"
    let stmnt = "SELECT * FROM foo";
    parse_sql(stmnt);
}
