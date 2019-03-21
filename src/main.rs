extern crate mysql;
use structopt::StructOpt;

use mysql as my;

const LIMIT: usize = 8;

fn show_tables(pool: &my::Pool) -> Vec<String> {
    let tables = pool.prep_exec("SHOW tables", ()).unwrap();
    tables.map(|x| my::from_row(x.unwrap())).collect()
}

fn count_table(pool: &my::Pool, table_name: &str) -> usize {
    let mut count = pool
        .prep_exec(format!("select count(*) FROM {}", table_name), ())
        .unwrap();
    my::from_row(count.next().unwrap().unwrap())
}

fn describe_table(
    pool: &my::Pool,
    table_name: &str,
) -> Vec<(String, String, String, String, Option<String>, String)> {
    let table = pool
        .prep_exec(format!("describe {}", table_name), ())
        .unwrap();
    table.map(|x| my::from_row(x.unwrap())).collect()
}

fn distinct_column(pool: &my::Pool, table_name: &str, row_name: &str) -> Vec<(my::Value, usize)> {
    let distinct = pool
        .prep_exec(
            format!(
                "SELECT `{}` AS G, count(*) AS C FROM `{}` GROUP BY G ORDER BY C DESC",
                row_name, table_name
            ),
            (),
        )
        .unwrap();

    distinct.map(|x| my::from_row(x.unwrap())).collect()
}

fn distinct_column_sumary(vec: Vec<(my::Value, usize)>, table_size: usize) -> String {
    let num_options = vec.len();

    if num_options == 0 {
        return "".to_string();
    }

    let is_all_diff = vec[0].1 == 1;

    if num_options > LIMIT {
        if is_all_diff {
            return "all different".to_string();
        } else if (vec[0].1 as f64) < (0.5 * table_size as f64) {
            return "almost all different".to_string();
        }
    }

    let max = 180; // TODO get `tput cols`
    let mut out = String::new();

    let mut sep = "";

    for value in vec {
        let value_str = if is_all_diff {
            value.0.as_sql(true)
        } else {
            format!("{}: {}", value.0.as_sql(true), value.1)
        };

        if out.len() + value_str.len() > max {
            out.push_str("....");
            break;
        }

        out.push_str(sep);
        sep = ", ";
        out.push_str(&value_str);
    }

    if out == "...." {
        return "very long value".to_string();
    }

    out
}

#[cfg(test)]
mod tests {
    use super::distinct_column_sumary;
    use mysql::Value::Bytes;
    use mysql::Value::Int;

    #[test]
    fn test_distinct_column_sumary() {
        let zero = distinct_column_sumary(vec![], 0);
        assert_eq!(zero, "");

        let all_different = distinct_column_sumary(vec![(Int(1), 1); 10], 10);
        assert_eq!(all_different, "all different");

        let almost_all_different = distinct_column_sumary(vec![(Int(1), 2); 10], 20);
        assert_eq!(almost_all_different, "almost all different");

        let one = distinct_column_sumary(vec![(Int(1), 1)], 1);
        assert_eq!(one, "1");

        let one = distinct_column_sumary(vec![(Int(1), 2)], 2);
        assert_eq!(one, "1: 2");

        let two = distinct_column_sumary(vec![(Int(1), 1); 2], 2);
        assert_eq!(two, "1, 1");

        let two = distinct_column_sumary(vec![(Int(1), 2); 2], 4);
        assert_eq!(two, "1: 2, 1: 2");

        let key = Bytes(vec![82; 2]);
        let long_key = Bytes(vec![82; 300]);
        let long_three = distinct_column_sumary(vec![(key, 1), (long_key, 1)], 2);
        assert_eq!(long_three, "'RR'....");

        let key = Bytes(vec![82; 2]);
        let long_key = Bytes(vec![82; 300]);
        let long_three = distinct_column_sumary(vec![(key, 10), (long_key, 2)], 12);
        assert_eq!(long_three, "'RR': 10....");

        let very_long_value = distinct_column_sumary(vec![(Bytes(vec![82; 300]), 1); 2], 2);
        assert_eq!(very_long_value, "very long value");
    }
}

/// Execute a count_grouped_by in all rows of all tables of a database; alias, `BI` for terminal.
#[derive(StructOpt, Debug)]
struct Cli {
    /// DSN to use for connection (example mysql://root@localhost:3306/database).
    url: String,
}

fn main() {
    let args = Cli::from_args();

    let pool = my::Pool::new(args.url).unwrap();

    let table_names = show_tables(&pool);

    for table_name in table_names {
        let table_size = count_table(&pool, &table_name);
        println!("\n\n\nTable: {:?} ({})\n==================================================================", table_name, table_size);

        let table = describe_table(&pool, &table_name);
        for row in table {
            let d = distinct_column(&pool, &table_name, &row.0);
            println!(
                " *  {:20} {:20} {}",
                row.0,
                row.1,
                distinct_column_sumary(d, table_size)
            );
        }
    }
}
