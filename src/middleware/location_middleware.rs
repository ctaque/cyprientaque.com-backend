use super::super::utils::token_utils;
use actix_service::{Service, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::Method,
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future, FutureExt,
};
use reqwest::Client;
use serde;
use std::{
    cell::RefCell,
    env,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

pub struct Location;

#[derive(serde::Deserialize)]
struct IpDataResponse {
    ip: String,
    city: String,
    region_code: String,
    continent_code: String,
    country_code: String,
}

impl<S, B> Transform<S> for Location
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LocationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LocationMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}
pub struct LocationMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for LocationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        // 5c6011abdbb5bdf3ab8c794680f285d8ddd7b2461845ea4ab264af17

        let is_prod = env::var("ENVIRONMENT").unwrap_or("development".to_string())
            == "production".to_string();

        let mut srv = self.service.clone();

        let qp = req.query_string().find("access");

        let access_allowed = match qp {
            Some(_) => true,
            None => false,
        };

        let contains_static = req.path().contains("/static");

        async move {
            let client = reqwest::Client::new();
            let conn_info = req.connection_info().clone();
            let real_ip = match is_prod {
                true => conn_info.realip_remote_addr().unwrap().clone().to_string(),
                false => "2a01:e34:ec7c:6560:a3d0:4b8e:6a46:4a98".to_string(),
            };
            let resp = client.get(&format!("https://api.ipdata.co/{}?api-key=5c6011abdbb5bdf3ab8c794680f285d8ddd7b2461845ea4ab264af17", real_ip))
                .send()
                .await
                .expect("failed to get reqwest response")
                .json::<IpDataResponse>()
                .await
                .expect("failed to parse IpData Json");

            if "FR" != resp.country_code.as_str() || access_allowed || contains_static {
                let fut = srv.call(req);
                let res = fut.await?;
                Ok(res)
            } else {
                let tpl = r#"
                    <!doctype html>
                    <html lang="fr">
                    <head>
                    <meta charset="utf-8">
                    <title>www.cyprientaque.com</title>
                    <link rel="preconnect" href="https://fonts.googleapis.com">
                    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                    <link href="https://fonts.googleapis.com/css2?family=Rubik:ital,wght@0,400;0,500;1,400&display=swap" rel="stylesheet">                    <style>
                        body{
                            font-family: 'Rubik', sans-serif;
                            margin: 0;
                        }
                        main{
                            max-width: 50rem;
                            margin: 0 auto;
                            padding: 2rem;
                        }
                        h1, h2, h3, h4 {
                            font-weight: 400;
                        }
                        a{
                            text-decoration: none;
                        }
                        p{
                            text-align: justify;
                        }
                        footer{
                            margin: 0 auto;
                            width: fit-content;
                        }
                    </style>
                    </head>
                    <body>
                    <main>
                        <h1>Cyprien Taque, Développeur web</h1>
                        <h3>Quelques projets :</h3>
                        <ul>
                            <li>
                                Billetterie de cinéma : <a href="https://www.cineoffice.com" target="_blank">La suite cinéoffice</a>.
                            </li>
                            <li>
                                Billetterie de cinéma (vad) : <a href="https://nantesleconcorde.cine.boutique" target="_blank"> La billetterie Ciné Boutique</a>.
                            </li>
                        </ul>
                        <h3>Je travaille avec ces outils :</h3>
                        <ul>
                            <li>
                                <p>Rust • Typescript • React • Redux • Nodejs • Postgresql • ExtJs</p>
                            </li>
                            <li>
                                <p>Fedora • Docker • Git • TDD + divers</p>
                            </li>
                        </ul>
                        <p>Side project: <a href="https://www.red-tomato.tv" target="_blank">red-tomato.tv</a></p>
                        <footer><span style="font-size: 1.5em;">💻️🤓🤖</span></footer>
                        <footer><span>Hello, world !</span></footer>
                        </main>
                    </body>
                    </html>
                    "#;
                Ok(req.into_response(
                    HttpResponse::Ok().body(tpl).into_body()
                ))
            }
        }.boxed_local()
    }
}
