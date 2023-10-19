use std::{
    io::{self, BufReader},
    pin::Pin,
    sync::Arc,
};

use axum::{routing::get, Router};
use futures_util::future::poll_fn;
use hyper::{
    server::{
        accept::Accept,
        conn::{AddrIncoming, Http},
    },
    Request,
};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::{Certificate, OwnedTrustAnchor, PrivateKey, RootCertStore, ServerConfig},
    TlsAcceptor,
};
use tower::MakeService;

#[tokio::main]
async fn main() {
    let _cert_store = build_cert_store();

    // WARNING!
    // If I see you doing this in production code without a very good reason (and documents
    // verifying and signing off on those choices in triplicate), I will personally
    // marry your favourite computer to a sledgehammer and force Windows ME on you.
    let tls_key = include_bytes!("../cert/key.pem");
    let tls_cert = include_bytes!("../cert/cert.pem");
    // You have been warned.
    // WARNING!

    let tls_config = tls_server_config(tls_key, tls_cert).expect("failed to setup tls");

    let acceptor = TlsAcceptor::from(tls_config);

    let listener = TcpListener::bind("[::1]:3000")
        .await
        .expect("failed to listen on [::1]:3000");
    let mut listener =
        AddrIncoming::from_listener(listener).expect("failed to setup connection stream");

    let protocol = Arc::new(Http::new());

    let mut app = Router::<()>::new()
        .route("/", get(handler))
        .into_make_service();

    loop {
        let stream = poll_fn(|cx| Pin::new(&mut listener).poll_accept(cx))
            .await
            .unwrap()
            .unwrap();

        let acceptor = acceptor.clone();

        let protocol = protocol.clone();

        let svc = MakeService::<_, Request<hyper::Body>>::make_service(&mut app, &stream);

        tokio::spawn(async move {
            if let Ok(stream) = acceptor.accept(stream).await {
                let _ = protocol.serve_connection(stream, svc.await.unwrap()).await;
            }
        });
    }
}

async fn handler() -> &'static str {
    "Hello, TLS!"
}

fn tls_server_config(key: &[u8], cert: &[u8]) -> Result<Arc<ServerConfig>, io::Error> {
    let mut key_reader = BufReader::new(key);
    let mut cert_reader = BufReader::new(cert);

    let key = PrivateKey(pkcs8_private_keys(&mut key_reader)?.remove(0));
    let certs = certs(&mut cert_reader)?
        .into_iter()
        .map(Certificate)
        .collect();

    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("bad certificate/key");

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Ok(Arc::new(config))
}

fn build_cert_store() -> RootCertStore {
    let mut cert_store = RootCertStore::empty();
    cert_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    cert_store
}
