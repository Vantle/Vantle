use expression::Expression;
use std::collections::HashMap;

fn evaluate(terms: HashMap<String, f64>, point: Vec<f64>) -> f64 {
    let expression = utility::unwrap(Expression::parse(&terms));
    expression.evaluate(&point)
}

fn parse(terms: HashMap<String, f64>) -> bool {
    Expression::parse(&terms).is_ok()
}
