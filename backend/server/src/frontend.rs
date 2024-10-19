use axum::{
    response::{Html, IntoResponse},
    Json,
};

pub async fn _html() -> impl IntoResponse {
    Html(
        r#"
        <script>
            fetch('http://localhost:4000/json')
              .then(response => response.json())
              .then(data => console.log(data));
        </script>
        "#,
    )
}

pub async fn json() -> impl IntoResponse {
    Json(vec!["one", "two", "three"])
}
