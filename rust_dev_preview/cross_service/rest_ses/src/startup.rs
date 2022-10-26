use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{get, scope, Data},
    App, HttpServer,
};
use tracing_actix_web::TracingLogger;

use crate::{
    client::{RdsClient, SesClient},
    healthz::healthz,
    report, work_item,
};

pub fn run(
    listener: TcpListener,
    rds_client: RdsClient,
    ses_client: SesClient,
) -> Result<Server, std::io::Error> {
    let rds_client = Data::new(rds_client);
    let ses_client = Data::new(ses_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/healthz", get().to(healthz))
            .service(
                scope("/api")
                    .service(work_item::collection::scope())
                    .service(report::send_report),
            )
            .app_data(rds_client.clone())
            .app_data(ses_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
