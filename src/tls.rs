use std::fs::File;
use std::io;
use std::io::{BufReader, Error};
use std::path::Path;
use std::sync::Arc;
use tokio::io::split;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsConnector, rustls::ClientConfig, TlsAcceptor};
use tokio_rustls::rustls::{Certificate, OwnedTrustAnchor, PrivateKey, RootCertStore, ServerConfig};
use webpki_roots;
use crate::common::read_write;
use rustls_pemfile;

pub async fn tls_connect(host: &String, port: &u16) -> Result<(), Error> {
    let addr = format!("{}:{}", host, port);

    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(
        |ta| {
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        },
    ));

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let tls_connector = TlsConnector::from(Arc::new(config));

    let server_name = host.as_str().try_into().unwrap();

    let stream = TcpStream::connect(&addr).await?;
    let stream = tls_connector.connect(server_name, stream).await?;

    let (reader, writer) = split(stream);

    read_write(reader, writer).await;

    Ok(())
}

