# SqlParser

This implements a simple sql parser capable of parsing the following

1. Create statement
```
CREATE TABLE foo {
    col_1 type_1,
    col_2 type_2,
    ...
    col_c type_c
}
```
Here type can be int or text.

2. Insert statement
```
INSERT INTO foo (col_1, col_2, ... col_c) VALUES (v_1, v_2, v_c)
```
This has two key limitations:
    - only a single row can be inserted at a time
    - every column must be specified

3. Select statement
```
SELECT * FROM foo
```
This only only support star variant

