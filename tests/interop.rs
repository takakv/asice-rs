use asice::Container;

// Containers from https://github.com/open-eid/SiVa
const BDOC_TM_2_SIG: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/bdoc_tm_valid_2_signatures.asice"
);

const ASICE_XADES_T: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/asiceWithXades-t-level.asice"
);

#[test]
fn opens_dual_signed_container() {
    let container = Container::open_file(BDOC_TM_2_SIG).unwrap();
    assert_eq!(container.data_files().len(), 1);
    assert_eq!(container.data_files()[0].name, "test.txt");
    assert_eq!(
        container.data_files()[0].mime_type,
        "application/octet-stream"
    );
    assert_eq!(container.signatures().len(), 2);
    assert_eq!(container.signatures()[0].name, "META-INF/signatures0.xml");
    assert_eq!(container.signatures()[1].name, "META-INF/signatures1.xml");
    assert!(container.signatures()[0].xml.contains("XAdESSignatures"));
    assert!(container.signatures()[1].xml.contains("XAdESSignatures"));
}

#[test]
fn opens_asice_xades_container() {
    let container = Container::open_file(ASICE_XADES_T).unwrap();
    assert_eq!(container.data_files().len(), 1);
    assert_eq!(container.data_files()[0].name, "test.txt");
    assert_eq!(container.data_files()[0].mime_type, "text/plain");
    assert_eq!(container.signatures().len(), 1);
    assert_eq!(container.signatures()[0].name, "META-INF/signatures0.xml");
    assert!(container.signatures()[0].xml.contains("XAdESSignatures"));
}

#[test]
fn round_trip() {
    let container = Container::open_file(BDOC_TM_2_SIG).unwrap();
    let reopened = Container::from_bytes(&container.to_bytes().unwrap()).unwrap();
    assert_eq!(
        reopened.data_files()[0].content,
        container.data_files()[0].content
    );
    assert_eq!(reopened.signatures()[0].name, "META-INF/signatures0.xml");
    assert_eq!(reopened.signatures()[0].xml, container.signatures()[0].xml);
    assert_eq!(reopened.signatures()[1].name, "META-INF/signatures1.xml");
    assert_eq!(reopened.signatures()[1].xml, container.signatures()[1].xml);
}
