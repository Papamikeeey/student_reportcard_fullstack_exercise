use axum::{routing::get, Router, Json};
use serde::Serialize;
use chrono::{Local};
use axum::response::Html;




#[derive(Serialize)]
struct Student {
    name: String,
    age: u32,
    subjects: Vec<String>,
    marks: Vec<u32>,
    average_score: f32,
    average_grade: String,
    graduation: bool,
    timestamp: String,
}


async fn frontend() -> Html<&'static str> {
    Html(include_str!("../frontend/index.html"))
}




async fn student_route() -> Json<Student> {
    let marks = vec![93, 83, 79, 87, 91];

    let mean_score = {
        let sum: u32 = marks.iter().sum();
        let count = marks.len() as f32;
        (sum as f32 / count *100.0).round() /100.0
    };

    let grade = match mean_score {
        90.0..100.0 => "A",
        85.0..89.9  => "A-",
        75.0..84.9  => "B+",
        70.0..74.9  => "B",
        65.0..69.9  => "B-",
        60.0..64.9  => "C+",
        55.0..59.9  => "C",
        50.0..54.9  => "C-",
        _  => "D",
    };

    let graduation = matches!(grade, "A" | "A-" | "B+" | "B" | "B-" | "C+" | "C" | "C-");

    let student_one = Student {

        name: "Johnson Omurwa".to_string(),
        age: 27,
        subjects: vec![
            "Programming".to_string(),
            "Algebra".to_string(),
            "Design".to_string(),
            "Web Development".to_string(),
            "Software".to_string(),
        ],
        marks,
        average_score: mean_score,
        average_grade: grade.to_string(),
        graduation,
        timestamp: Local::now().to_string(),

    };

    Json(student_one)

}

#[tokio::main]
async fn main() {
    
    let app = Router::new().route("/", get(frontend)).route("/student", get(student_route));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind TCP litsener");

    println!("Server running at http://127.0.0.1:3000");


    axum::serve(listener, app)
        .await
        .expect("server error");
}
