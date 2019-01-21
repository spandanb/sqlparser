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
struct ColumnDefOption {
    column_name: Option<String>,
    column_type: Option<SqlType>
}


#[derive(Debug)]
struct CreateTable {
    table_name: String,
    column_defs: Vec<ColumnDef>,
}

#[derive(Debug)]
struct  CreateTableOption {
    table_name: Option<String>,
    column_defs: Vec<Option<ColumnDef>>,
}

#[derive(Debug)]
struct SelectStmnt {
    table_name: String,
    select_columns: String,
}

#[derive(Debug)]
struct SelectStmntOption {
    table_name: Option<String>,
    select_columns: Option<String>,
}

#[derive(Debug)]
enum ParsedStmnt {
    // The CreateTable operation and the corresponding
    // create table data
    CreateTable(CreateTable),
    SelectStmnt(SelectStmnt),
}

#[derive(Debug)]
enum ParsedStmntOption {
    // The CreateTable operation and the corresponding
    // create table data
    CreateTableOption(CreateTableOption),
    SelectStmntOption(SelectStmntOption),
    None
}

fn parse_create_table_stmnt(child: pest::iterators::Pair<'_, Rule>) -> CreateTable {
    //splitting the parser up so, so I can practice macros latter
    let ct_opt = CreateTableOption {
            table_name: None,
            column_defs: Vec::new()
    };

    match child.as_rule() {
        Rule::table_name => {
            let table_name = child.as_str();
            ct_opt.table_name = Some(String::from(table_name))
        },
        Rule::column_name => {
            let column_name = Some(child.as_str());

            let cdef_opt = ColumnDefOption {
                column_name: Some(column_name),
                column_type: None,
            };
            ct_opt.column_defs.push(Some(cdef_opt));
        },
        Rule::column_type => {
            // ordering ensures column_name is set
            let mut cdef_opt = ct_opt.column_defs.last();
            cdef_opt.unwrap().column_type = Some(SqlType::from_str(child.as_str())
                                  .expect("successfully parsed sql-type"));
        }
        _ => (),
    }
    ct_opt
}

fn parse_select_stmnt(child: pest::iterators::Pair<'_, Rule>) -> SelectStmnt {
    let st_opt = SelectStmntOption {
            table_name: None,
            select_columns: None
    }
    match child.as_rule() {
        Rule::table_name => {
            st_opt.table_name = Some(String::from(child.as_str()))
        },
        Rule::star => {
            ps_opt.select_columns = Some(String::from(child.as_str()));
        },
    }
    st_opt
}

fn parse_sql(stmnt: &str) -> ParsedStmnt {
    let parsed_stmnt = SQLParser::parse(Rule::sql_grammar, stmnt)
    .expect("successful parse") // unwrap the parse result
    .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut result = ParsedStmnt::None;

    for child in parsed_stmnt.into_inner()
    .flatten() {
        // split the parser based on the first word
        match child.as_rule() {
            Rule::create_kw => {
                let create_table = parse_create_table_stmnt(child);
                result = ParsedStmnt::CreateTable(create_table)
            }
            Rule::select_kw => {
                let st_opt = parse_select_stmnt(child);
                result = ParsedStmnt::SelectStmnt(st_opt)
            }
            _ => (),
        }
    },
    println!("Created: {:?}", ps_opt);
    result
}


fn main() {
    //let stmnt = "CREATE TABLE foo { bar INT , baz TEXT  }"
    let stmnt = "SELECT * FROM foo";
    parse_sql(stmnt);
}
