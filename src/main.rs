use axum::routing::get;

mod fallback;
use fallback::fallback;

mod shutdown_signal;
use shutdown_signal::shutdown_signal;

pub mod data;

pub mod models {
    pub mod person;
}

pub mod views {
    pub mod html;
}

pub mod controllers {
    pub mod get_persons;
    pub mod get_persons_similarity;
}

pub mod services {
    pub mod similarity;
}

#[tokio::main]
pub async fn main() {
     // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(
            fallback
        )
        .route("/persons",
            get(crate::controllers::get_persons::get_persons)
        );


    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
