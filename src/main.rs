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
    column_defs: Vec<Option<ColumnDefOption>>,
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

fn unwrap_column_defs(column_defs: & mut Vec<Option<ColumnDefOption>>) -> Vec<ColumnDef> {
    // recursive function avoid borrow checker and looped
    // value mutations
    if column_defs.len() == 0 {
        return Vec::<ColumnDef>::new();
    }
    let cdef = column_defs.pop()
                .unwrap()
                .unwrap();
    let cd = ColumnDef {
        column_name: cdef.column_name.unwrap(),
        column_type: cdef.column_type.unwrap()
    };
    let mut result = unwrap_column_defs(column_defs);
    result.push(cd);
    result
}

fn parse_create_table_stmnt(pairs: pest::iterators::FlatPairs<'_, Rule>) -> CreateTable {
    //splitting the parser up so, so I can practice macros latter
    let mut ct_opt = CreateTableOption {
            table_name: None,
            column_defs: Vec::new()
    };
    let mut column_name : Option<String> = None;

    for child in pairs {
        match child.as_rule() {
            Rule::table_name => {
                let table_name = child.as_str();
                ct_opt.table_name = Some(String::from(table_name))
            },
            Rule::column_name => {
                column_name = Some(String::from(child.as_str()));
            },
            Rule::column_type => {
                // ordering ensures column_name is set
                ct_opt.column_defs.push(
                    Some(
                        ColumnDefOption {
                            column_name: column_name.clone(),
                            column_type: Some(SqlType::from_str(child.as_str()).unwrap()),
                        }
                    )
                );
            }
            _ => (),
        }
    }
    let column_defs = unwrap_column_defs(&mut ct_opt.column_defs);
    CreateTable {
        table_name: ct_opt.table_name.unwrap(),
        column_defs: column_defs
    }
}

fn parse_select_stmnt(pairs: pest::iterators::FlatPairs<'_, Rule>) -> SelectStmnt {
    let mut st_opt = SelectStmntOption {
            table_name: None,
            select_columns: None
    };
    for child in pairs {
        match child.as_rule() {
            Rule::table_name => {
                st_opt.table_name = Some(String::from(child.as_str()))
            },
            Rule::star => {
                st_opt.select_columns = Some(String::from(child.as_str()));
            },
            _ => (),
        }
    }
    // println!("SelectStmnt is {:?}", st_opt);
    // materialize the option
    SelectStmnt {
        table_name: st_opt.table_name
                        .unwrap(),
        select_columns: st_opt.select_columns
                        .unwrap()
    }
}

fn parse_sql(stmnt: &str) -> ParsedStmnt {
    let parsed_stmnt = SQLParser::parse(Rule::sql_grammar, stmnt)
    .expect("successful parse") // unwrap the parse result
    .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut result : Option<ParsedStmnt> = None;

    let pairs = parsed_stmnt.into_inner();
    // split the parser based on the first word
    for child in pairs.clone()
                .flatten() {
        match child.as_rule() {
            Rule::create_kw => {
                // println!("IN create_table");
                let create_table = parse_create_table_stmnt(pairs.clone().flatten());
                result = Some(ParsedStmnt::CreateTable(create_table))
            }
            Rule::select_kw => {
                // println!("IN select_kw");
                let select_stmnt = parse_select_stmnt(pairs.clone().flatten());
                result = Some(ParsedStmnt::SelectStmnt(select_stmnt))
            }
            _ => (),
        }
    }
    println!("Created: {:?}", result);
    result.unwrap()
}


fn main() {
    let stmnt = "CREATE TABLE foo { bar INT , baz TEXT  }";
    //let stmnt = "SELECT * FROM foo";
    parse_sql(stmnt);
}
