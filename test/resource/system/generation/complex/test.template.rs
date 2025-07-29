use std::collections::HashMap;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub scores: Vec<i32>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Team {
    pub name: String,
    pub members: Vec<Person>,
    pub budget: i32,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct MyType<T> {
    pub value: T,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Matrix<T> {
    pub rows: Vec<Vec<T>>,
    pub metadata: HashMap<String, T>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub enum Status<T> {
    Success(T),
    Warning(T, String),
    Error(String),
}

fn distance(point: Point) -> f64 {
    ((point.x.pow(2) + point.y.pow(2)) as f64).sqrt()
}

fn average(person: Person) -> f64 {
    if person.scores.is_empty() {
        0.0
    } else {
        person.scores.iter().sum::<i32>() as f64 / person.scores.len() as f64
    }
}

fn team_average(team: Team) -> f64 {
    if team.members.is_empty() {
        0.0
    } else {
        let total_scores: Vec<i32> = team
            .members
            .iter()
            .flat_map(|member| &member.scores)
            .cloned()
            .collect();

        if total_scores.is_empty() {
            0.0
        } else {
            total_scores.iter().sum::<i32>() as f64 / total_scores.len() as f64
        }
    }
}

fn process_data(data: HashMap<String, i32>) -> i32 {
    data.values().sum()
}

fn sum_vector(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

fn nested_process(data: HashMap<String, Vec<HashMap<String, Vec<i32>>>>) -> i32 {
    data.values()
        .flat_map(|vec| vec.iter())
        .flat_map(|map| map.values())
        .flat_map(|vec| vec.iter())
        .sum()
}

fn two_level_process(data: Vec<HashMap<String, i32>>) -> i32 {
    data.iter().flat_map(|map| map.values()).sum()
}

fn optional_process(name: Option<String>) -> String {
    match name {
        Some(n) => format!("Hello, {}!", n),
        None => "Hello, stranger!".to_string(),
    }
}

fn count_some(data: HashMap<u32, Option<MyType<String>>>) -> usize {
    data.values().filter(|v| v.is_some()).count()
}

fn matrix_sum(matrix: Matrix<i32>) -> i32 {
    matrix.rows.iter().flat_map(|row| row.iter()).sum::<i32>()
        + matrix.metadata.values().sum::<i32>()
}

fn process_status(status: Status<i32>) -> String {
    match status {
        Status::Success(value) => format!("Success: {}", value),
        Status::Warning(value, msg) => format!("Warning: {} - {}", value, msg),
        Status::Error(msg) => format!("Error: {}", msg),
    }
}

fn nested_optionals(data: Option<Vec<Option<String>>>) -> Vec<String> {
    data.unwrap_or_default().into_iter().flatten().collect()
}

fn sum_refs(x: &i32, y: &i32) -> i32 {
    *x + *y
}

pub mod geometry {
    use super::Point;

    pub fn rectangle_area(top_left: Point, bottom_right: Point) -> i32 {
        let width = (bottom_right.x - top_left.x).abs();
        let height = (top_left.y - bottom_right.y).abs();
        width * height
    }
}

fn nested_sum(
    data: std::collections::HashMap<String, std::collections::HashMap<String, i32>>,
) -> i32 {
    data.get("a").map(|inner| inner.values().sum()).unwrap_or(0)
}
