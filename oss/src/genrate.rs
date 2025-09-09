use sqlparser::ast::{ColumnOption, Statement};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

fn main() {
    let sql = r#"
        CREATE TABLE user (
            id bigint AUTO_INCREMENT,
            name varchar(255) NULL COMMENT 'The username',
            UNIQUE name_index (name),
            PRIMARY KEY (id)
        ) ENGINE = InnoDB COLLATE utf8mb4_general_ci COMMENT 'user table';

          CREATE TABLE user1 (
            id1 bigint AUTO_INCREMENT,
            name1 varchar(255) NULL COMMENT 'The username',
            UNIQUE name_index (name1),
            PRIMARY KEY (id1)
        ) ENGINE = InnoDB COLLATE utf8mb4_general_ci COMMENT 'user table';
    "#;

    let dialect = MySqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql);

    for statement in ast {
        for stmt in statement {
            match stmt {
                Statement::CreateTable(create_table) => {
                    for table in create_table.name.0 {
                        println!("表名:{}", table);
                    }

                    for column in create_table.columns {
                        if let Some(comment) = column.options.iter().find_map(|opt| {
                            if let ColumnOption::Comment(s) = &opt.option {
                                Some(s.as_str())
                            } else {
                                None
                            }
                        }) {
                            println!(
                                "字段名: {}, 字段类型: {}, 字段描述: {}",
                                column.name, column.data_type, comment
                            );
                        } else {
                            println!("字段名: {}, 字段类型: {}", column.name, column.data_type);
                        }
                    }

                    for pk in create_table.constraints {
                        println!("约束条件:{}", pk);
                    }
                }
                _ => {}
            }
        }
    }
}
