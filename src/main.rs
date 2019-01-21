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
struct ColumnDefOptional {
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

fn parse_sql(stmnt: &str) -> ParsedStmnt {

    let parsed_stmnt = SQLParser::parse(Rule::sql_grammar, stmnt)
    .expect("successful parse") // unwrap the parse result
    .next().unwrap(); // get and unwrap the `file` rule; never fails

    let ps_opt = ParsedStmntOption::None;
    let ct_opt = CreateTableOption {    
    }
    let st_opt = SelectOption::None;

    for child in parse_stmnt.into_inner()
    .flatten() {
        match child.as_rule() {
            Rule::create_kw => {
                ps_opt = ParsedStmntOption::CreateTableOption(
                    CreateTableOption {
                        table_name: None,
                        column_defs: Vec::new()
                    }
                )
            }
            Rule::select_kw => {
                ps_opt = ParsedStmntOption::SelectStmntOption(
                    SelectStmntOption {
                        table_name: None,
                        select_columns: None
                    }
                )
            }
            Rule::table_name => {
                let table_name = child.as_str();
                ps_opt.table_name = String::from(table_name)
            },
            Rule::select_columns => {
                ps_opt.select_columns = String::from(child.as_str());
            },
            Rule::column_name => {
                let column_name = Some(child.as_str());
                match ps_opt {
                    ParsedStmntOption::CreateTableOption => {
                        let cdef_opt = ColumnDefOption {
                            column_name: column_name,
                            column_type: None,
                        };
                        ps_opt.column_defs.push(cdef_opt);
                    },
                    ParsedStmntOption::SelectStmntOption => {
                        ps_opt.select_column = column_name
                    },
                    _ => (),
                }
            },
            Rule::column_type => {
                // ordering ensures column_name is set
                let mut cdef_opt = ps_opt.column_defs.last();
                cdef_opt.column_type = SqlType::from_str(child.as_str())
                                      .expect("successfully parsed sql-type");
            }
            _ => (),
        }
    },
    println!("Created: {:?}", ps_opt);
    ps_opt
}


fn main() {
    //let stmnt = "CREATE TABLE foo { bar INT , baz TEXT  }"
    let stmnt = "SELECT * FROM foo";
    parse_sql(stmnt);
}
