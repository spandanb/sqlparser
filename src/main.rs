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
    fn test_parse_create_table() {
        let stmnt = "CREATE TABLE foo { bar INT , baz TEXT  }";
        if let ParsedStmnt::CreateTable(res) = parse_sql(stmnt) {
            assert_eq!(res.table_name, "foo");
            assert_eq!(res.column_defs[0].column_name, "bar");
            assert_eq!(res.column_defs[0].column_type, SqlType::Int);
            assert_eq!(res.column_defs[1].column_name, "baz");
            assert_eq!(res.column_defs[1].column_type, SqlType::Text);
        }
    }

    #[test]
    fn test_parse_select_stmnt() {
        let stmnt = "SELECT * FROM foo";
        if let ParsedStmnt::SelectStmnt(res) = parse_sql(stmnt) {
            assert_eq!(res.table_name, "foo");
            assert_eq!(res.select_columns, "*");
        }
    }

    #[test]
    fn test_parse_insert_stmnt() {
        let stmnt = "INSERT INTO foo (cola, colb, colc) VALUES (v1, v2, v3)";
        if let ParsedStmnt::InsertStmnt(res) = parse_sql(stmnt) {
            assert_eq!(res.table_name, "foo");
            assert_eq!(res.columns[0], "cola");
            assert_eq!(res.columns[1], "colb");
            assert_eq!(res.columns[2], "colc");
            assert_eq!(res.values[0], "v1");
            assert_eq!(res.values[1], "v2");
            assert_eq!(res.values[2], "v3");
        }
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
struct InsertStmnt {
    table_name: String,
    columns: Vec<String>,
    // conversion to correct type requires info
    // which is not available from the insert stmnt
    values: Vec<String>
}

#[derive(Debug)]
struct InsertStmntOption {
    table_name: Option<String>,
    columns: Vec<Option<String>>,
    values: Vec<Option<String>>
}

#[derive(Debug)]
enum ParsedStmnt {
    // the operation and the corresponding data
    CreateTable(CreateTable),
    SelectStmnt(SelectStmnt),
    InsertStmnt(InsertStmnt),
}

fn unwrap_column_defs(column_defs: & mut Vec<Option<ColumnDefOption>>) -> Vec<ColumnDef> {
    // recursive function to avoid borrow checker
    // this is hard to use generically since it does a deep unwrap
    // a better approach might be to implement (non-)optional variant
    // via a trait
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

fn unwrap_vec<T>(vec: & mut Vec<Option<T>>) -> Vec<T> {
    // unwrap a  vector of options
    if vec.len() == 0 {
        return Vec::<T>::new();
    }
    let ele = vec.pop();
    let mut result = unwrap_vec(vec);
    result.push(ele.unwrap().unwrap());
    result
}

fn parse_create_table_stmnt(pairs: pest::iterators::FlatPairs<'_, Rule>) -> CreateTable {
    // parse create table stmnt
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

fn parse_insert_stmnt(pairs: pest::iterators::FlatPairs<'_, Rule>) -> InsertStmnt {
    let mut is_opt = InsertStmntOption {
            table_name: None,
            columns: Vec::new(),
            values: Vec::new()
    };

    for child in pairs {
        match child.as_rule() {
            Rule::table_name => {
                let table_name = child.as_str();
                is_opt.table_name = Some(String::from(table_name))
            },
            Rule::column_name => {
                let column_name = Some(String::from(child.as_str()));
                is_opt.columns.push(column_name);
            },
            Rule::value => {
                let column_value = Some(String::from(child.as_str()));
                is_opt.values.push(column_value);
            },
            _ => (),
        }
    }
    InsertStmnt {
        table_name: is_opt.table_name.unwrap(),
        columns: unwrap_vec(&mut is_opt.columns),
        values: unwrap_vec(&mut is_opt.values),
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
    SelectStmnt {
        table_name: st_opt.table_name
                        .unwrap(),
        select_columns: st_opt.select_columns
                        .unwrap()
    }
}

fn parse_sql(stmnt: &str) -> ParsedStmnt {
    // parses sql statement
    // collecting the parsed the results is done
    // in parse_<STMNT> functions

    let parsed_stmnt = SQLParser::parse(Rule::sql_grammar, stmnt)
    .expect("grammar parse failed") // unwrap the parse result
    .next()
    .unwrap(); // get and unwrap the `sql_grammar` rule;

    let mut result : Option<ParsedStmnt> = None;

    let pairs = parsed_stmnt.into_inner();
    // split the parser based on the first word
    for child in pairs.clone()
                .flatten() {
        match child.as_rule() {
            Rule::create_kw => {
                let create_table = parse_create_table_stmnt(pairs.clone().flatten());
                result = Some(ParsedStmnt::CreateTable(create_table))
            },
            Rule::select_kw => {
                let select_stmnt = parse_select_stmnt(pairs.clone().flatten());
                result = Some(ParsedStmnt::SelectStmnt(select_stmnt))
            },
            Rule::insert_kw => {
                let insert_stmnt = parse_insert_stmnt(pairs.clone().flatten());
                result = Some(ParsedStmnt::InsertStmnt(insert_stmnt))
            },
            _ => (),
        }
    }
    result.unwrap()
}


fn main() {
    // let stmnt = "CREATE TABLE foo { bar INT , baz TEXT  }";
    // let stmnt = "SELECT * FROM foo";
    let stmnt = "INSERT INTO foo (bar, baz) VALUES ( abcd, def )";
    let result = parse_sql(stmnt);
    println!("Parsed: {:?}", result);
}
