use mysql::*;
use mysql::prelude::*;
use plotters::prelude::*;
use serde::Deserialize;
use serde_yaml;
use std::fs;

#[derive(Debug, Deserialize)]
struct DbConfig {
    host: String,
    user: String,
    password: String,
    dbname: String,
}

fn load_db_config() -> DbConfig {
    let yaml_str = fs::read_to_string("run.yml").expect("Failed to read YAML config");
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str).unwrap();
    let db = yaml.get("db").unwrap();
    serde_yaml::from_value(db.clone()).unwrap()
}

fn create_mysql_pool(conf: &DbConfig) -> PooledConn {
    let url = format!(
        "mysql://{}:{}@{}/{}",
        conf.user, conf.password, conf.host, conf.dbname
    );
    let pool = Pool::new(url).unwrap();
    pool.get_conn().unwrap()
}

fn fetch_coner_data(conn: &mut PooledConn, year: &str, level: &str) -> Vec<(u32, u32, u32)> {
    let sql = format!(
        "SELECT round, SUM(zc+kc), SUM(zj+kj) FROM {} WHERE level='{}' GROUP BY round",
        year, level
    );
    conn.query_map(sql, |(round, corners, goals)| (round, corners, goals)).unwrap()
}

fn draw_bar_chart(rounds: &[u32], corners: &[u32], goals: &[u32], filename: &str) {
    let root = BitMapBackend::new(filename, (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let y_max = corners.iter().chain(goals.iter()).copied().max().unwrap_or(10) + 5;
    let mut chart = ChartBuilder::on(&root)
        .caption("角球 vs 进球", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0u32..rounds.len() as u32, 0u32..y_max)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let bar_width = 0.4;
    chart
        .draw_series(
            corners
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    Rectangle::new(
                        [(i as u32, 0), (i as u32, *c)],
                        BLUE.mix(0.5).filled(),
                    )
                }),
        )
        .unwrap();

    chart
        .draw_series(
            goals
                .iter()
                .enumerate()
                .map(|(i, g)| {
                    Rectangle::new(
                        [(i as u32 + 1, 0), (i as u32 + 1, *g)],
                        RED.mix(0.5).filled(),
                    )
                }),
        )
        .unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let year = args.get(1).map(|s| s.as_str()).unwrap_or("j22");
    let level = args.get(2).map(|s| s.as_str()).unwrap_or("C");

    let db_conf = load_db_config();
    let mut conn = create_mysql_pool(&db_conf);
    let data = fetch_coner_data(&mut conn, year, level);

    let (rounds, corners, goals): (Vec<u32>, Vec<u32>, Vec<u32>) = data.into_iter().unzip3();

    draw_bar_chart(&rounds, &corners, &goals, "output.png");
    println!("图表已生成: output.png");
}

trait Unzip3<T1, T2, T3> {
    fn unzip3(self) -> (Vec<T1>, Vec<T2>, Vec<T3>);
}

impl<T1, T2, T3, I> Unzip3<T1, T2, T3> for I
where
    I: Iterator<Item = (T1, T2, T3)>,
{
    fn unzip3(self) -> (Vec<T1>, Vec<T2>, Vec<T3>) {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        let mut v3 = Vec::new();
        for (a, b, c) in self {
            v1.push(a);
            v2.push(b);
            v3.push(c);
        }
        (v1, v2, v3)
    }
}
