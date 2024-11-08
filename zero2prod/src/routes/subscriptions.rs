use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// an extractor in actix_web is something that can extract things like data and url from the
// request, the data extracter can be passed in as an arguement
// Extractor must implement FromRequest
// So before calling this function, in the back, "from_request" method is invoked in this case
// especiically Form::from_request
// from_request deserializes the data into FormData struct we defined, according to the rules of
// URL encoding and leveragin serde_urlencoded and Deserialize implementation of FormData which was
// auto geneated with #[derive(serde::Deserialize)]
// Any error is returned to caller with 400 BAD request
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form,pool), // skip these arguements from tracing
    fields(
        subscriber_email=%form.email,
        subscriber_name=%form.name,
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>, // web::data is an extractor, actix internally uses hashmap of data with their type identifier,
                             // in our sace type pgConnection, it searches in its hashmap if we have a data of that type, and if
                             // it finds it passes into the function
) -> impl Responder {
    // no need to do these because of Using macro
    //let request_id = Uuid::new_v4();
    /*let request_span = tracing::info_span!(
        "addin new subscriber",
        %request_id,
        subscriber_email=%form.email,
        subscriber_name=%form.name,
    );*/
    //let _req = request_span.enter();

    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
    // tracing::instrument macro handles asynchronous
    /*let query_span = tracing::info_span!("Executing db query");

        match sqlx::query!(
            r#"
    INSERT INTO subscriptions (id,email,name ,subscribed_at) VALUES ($1,$2,$3,$4)
            "#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now(),
        )
        .execute(pool.get_ref()) // execute(connection.get_ref()) does not implement Executor, because it only allows one connection to change stuffs in db
        .instrument(query_span)
        .await
        {
            Ok(_) => {
                tracing::info!("Subscriber already successfully added",);
                HttpResponse::Ok().finish()
            }
            Err(e) => {
                tracing::error!("subscriber could not be added");
                HttpResponse::InternalServerError().finish()
            }
        }*/
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO subscriptions (id,email,name ,subscribed_at) VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool) // execute(connection.get_ref()) does not implement Executor, because it only allows one connection to change stuffs in db
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
/*
In this code, tracing and tracing::instrument are used to manage structured, high-level logging and provide context for the asynchronous operations, particularly around HTTP requests and database interactions. Here’s how and why they’re used:

    tracing crate:
        tracing provides tools for structured and contextual logging, which is especially useful in asynchronous contexts (like in an async function) where logs from different tasks can interleave. Using tracing, you can capture detailed information about what’s happening in each step, helping to troubleshoot, monitor, and understand code behavior.
        Logs generated with tracing also allow for adding structured metadata, like request IDs, user emails, or operation names. These are logged in a structured way (instead of simple strings), making it easier to filter and analyze logs programmatically.

    tracing::info_span!:
        The info_span! macro creates a "span" that represents a named operation. Spans can be nested, which helps in grouping related logs together. For instance, when subscribing a user, two spans are created:
            request_span logs details specific to the entire subscribe request, including a request_id, the subscriber_email, and subscriber_name.
            query_span logs the database query operation itself, adding context to each log related to the query.
        Spans can carry metadata (key-value pairs like %request_id), allowing you to track specific values throughout the operation. Here, request_span holds the request ID and subscriber information, making it easier to correlate log messages specific to a single request.

    tracing::Instrument trait:
        The .instrument(query_span) call on the database query allows that operation to inherit the context of query_span. This means any logs or errors within that span are associated with query_span, helping distinguish between different operations in the logs.
        Using .instrument() on async operations like await ensures that when the operation resumes, it has the correct tracing context.

    Logging with contextual information:
        In the success and error cases within the match block, the tracing::info! and tracing::error! macros log the result of the subscription attempt. Each log includes the request_id to make tracking specific requests easier. For example, if an error occurs, the request_id can be used to quickly trace back to the specific request and query that failed.
*/
