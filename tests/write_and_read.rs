use evp::{exporters::Exporter, prelude::*};
use outdir_tempdir::TempDir;
use tracing_subscriber::EnvFilter;

#[test]
fn test_write_and_read() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let dir = TempDir::new();
    let path = dir.path().join("test.evp");

    let secret_path = format!("{}/tests/ecdsa_private_key.p8", env!("CARGO_MANIFEST_DIR"));
    let algo = attesting::Algorithm::ES256;
    tracing::info!("Loading private key from: {secret_path}");
    let secret = attesting::Secret::ecdsa_keypair_from_file(algo, &secret_path)
        .expect("Failed to read key from file");

    {
        // Create package
        let mut package = EvidencePackage::new(
            path.clone(),
            "EVP Test Package",
            &[("EVP Test Tool", "evp-test@example.com")],
        )
        .expect("Failed to create package");
        let image_media = package
            .add_media(MediaFile::from(include_bytes!("image.jpg")))
            .expect("Failed to read image as media file")
            .hash();
        let audio_media = package
            .add_media(MediaFile::from(include_bytes!("pour.mp3")))
            .expect("Failed to read audio as media file")
            .hash();
        let tc_id = {
            let tc = package
                .create_test_case("Test Case")
                .expect("Failed to create test case");
            tc.evidence_mut().push(Evidence::new(
                "text/plain",
                EvidenceData::Text {
                    content: "Hello, world!".to_string(),
                },
            ));
            tc.evidence_mut().push(Evidence::new(
                "text/markdown",
                EvidenceData::Text {
                    content: include_str!("test.md").to_string(),
                },
            ));
            tc.evidence_mut().push(Evidence::new(
                "image/jpeg",
                EvidenceData::Media { hash: image_media },
            ));
            tc.evidence_mut().push(Evidence::new(
                "audio/mp3",
                EvidenceData::Media { hash: audio_media },
            ));
            *tc.id()
        };
        package
            .attest_test_case(
                tc_id,
                &secret,
                attesting::RegisteredHeader {
                    algorithm: algo,
                    ..Default::default()
                },
            )
            .expect("Failed to attest");
        package.save().expect("Failed to save package");
    }

    {
        // Read package
        let mut package = EvidencePackage::open(path).expect("Failed to read package");

        // Export package to formats
        #[cfg(feature = "exporter-excel")]
        let _ = evp::exporters::excel::ExcelExporter
            .export_package(&mut package, dir.path().join("export.xlsx"));
        #[cfg(feature = "exporter-html")]
        let _ = evp::exporters::html::HtmlExporter
            .export_package(&mut package, dir.path().join("export.html"));

        let cases = package
            .test_case_iter()
            .expect("Failed to load test cases")
            .cloned()
            .collect::<Vec<_>>();
        let mut count = 0;
        for tc in cases {
            count += 1;
            let attestations = package
                .test_case_attestations(*tc.id())
                .expect("Failed to parse attestations");
            assert_eq!(attestations.len(), 1);
            tracing::debug!("{attestations:?}");
            let (ats, same) = &attestations[0];
            assert_eq!(*same, true);
            let _ = ats
                .decode(&secret, algo)
                .expect("Failed to validate attestation");
            assert_eq!(tc.metadata().title(), "Test Case");
            assert_eq!(tc.evidence().len(), 4);
            let ev = tc.evidence()[0].clone();
            assert_eq!(ev.kind(), "text/plain");
            assert_eq!(
                String::from_utf8_lossy(&ev.data(&mut package)),
                "Hello, world!"
            );
        }
        assert_eq!(count, 1);
    }
}
