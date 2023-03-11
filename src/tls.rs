use webpki_roots;
use rustls_pemfile;
use crate::common::read_write;
use std::{
    fs::File, 
    io::{self, BufReader, Error},
    path::Path,
    sync::Arc
};
use tokio::{io::split, net::{TcpListener, TcpStream}};
use tokio_rustls::{
    TlsConnector,
    rustls::ClientConfig,
    TlsAcceptor,
    rustls::{Certificate, OwnedTrustAnchor, PrivateKey, RootCertStore, ServerConfig}
};

pub async fn tls_connect(host: &str, port: u16, ca: Option<String>) -> Result<(), Error> {
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

    if let Some(ca) = ca {
        for cert in load_certs(Path::new(&ca)).unwrap() {
            root_cert_store.add(&cert)
                .map_err(|_e| {
                    Error::new(io::ErrorKind::InvalidInput, "couldn't add CA")
                })
                .unwrap();
        }
    }

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let tls_connector = TlsConnector::from(Arc::new(config));

    let server_name = host.try_into().unwrap();

    let stream = TcpStream::connect(&addr).await?;
    let stream = tls_connector.connect(server_name, stream).await?;

    let (reader, writer) = split(stream);

    read_write(reader, writer).await;

    Ok(())
}

pub async fn tls_listen(host: &str, port: u16, ca: Option<String>, cert: String, key: String) -> Result<(), Error> {
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

    if let Some(ca) = ca {
        for cert in load_certs(Path::new(&ca))? {
            root_cert_store.add(&cert)
                .map_err(|_e| {
                    Error::new(io::ErrorKind::InvalidInput, "couldn't add CA")
                })?;
        }
    }

    let certs = load_certs(Path::new(cert.as_str()))?;
    let mut keys = load_keys(Path::new(key.as_str()))?;
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .map_err(|err| Error::new(io::ErrorKind::InvalidInput, err))?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(&addr).await?;

    let (socket, _) = listener.accept().await?;

    let stream = acceptor.accept(socket).await?;

    let (reader, writer) = split(stream);

    read_write(reader, writer).await;

    Ok(())
}

fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    let f = File::open(path)?;

    return rustls_pemfile::certs(&mut BufReader::new(f))
        .map_err(|_| Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect());
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    let f = File::open(path)?;

    rustls_pemfile::rsa_private_keys(&mut BufReader::new(f))
        .map_err(|_| Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}