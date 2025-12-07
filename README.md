# 🎓 STUDENT GRADUATION REPORT CARD FULLSTACK RUST PROJECT

This was an exciting & challenging project to learn more about Rust, backend and frontend. The aim was to gain understanding between the related facets of a project...
As my first programming language, I'm starting to slowly connect the dots in patterns of logic and this opened a new pathway to see more.

This is a fullstack project incorporating;
- Rust for backend logic and API
- Axum for HTTP routing
- JSON for communication between the channels
- HTML, CSS and Javascript for frontend polish

---
What this program does...
- Calculates students' average scores
- Calculate their final grade
- Determines their eligibility to graduate based on their final grade
- Showcases the data through an API
- Diplays the data in a browser page - frontend

I was able to further cement a few important aspects while doing this project: -
- How Rust structs are crucial for representing real-world Data
- How APIs work within the backend 
- Serializing Rust data into **JSON**
- Building **HTTP API** using Axum
- How frontend fetches data from the backend and how they relate
- Styling within the frontend and syntax
- Structuring of a complex project

---

## TECH STACK

**BACKEND**
- Rust
- Axum
- Tokio
- Serde
- Chrono

**FRONTEND**
- HTML
- CSS
- Vanilla JavaScript

---

## PROJECT STRUCTURE
-studentreportcard_api
|-src/
    |_main(Backend)
|-frontend
    |_index.html(frontend)
|-.gitignore
|-cargo.toml
|-cargo.lock
|_README.md    

---

Future TODO
- Add multiple students 
- Add more depth and variety with the aesthetics
- Deploy publicly

---

## HOW TO RUN

## ▶️ How to Run

1. Clone the repository  
2. Navigate into the project folder  
3. Start the server:

```bash
cargo run

pen your browser and visit:

Frontend:
http://127.0.0.1:3000

API endpoint (JSON):
http://127.0.0.1:3000/student

