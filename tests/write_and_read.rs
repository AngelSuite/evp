use evp::prelude::*;
use outdir_tempdir::TempDir;
use tracing_subscriber::EnvFilter;

#[test]
fn test_write_and_read() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let dir = TempDir::new();
    let mut path = dir.path().to_path_buf();
    path.push("test.evp");

    {
        // Create package
        let mut package = EvidencePackage::new(
            path.clone(),
            "EVP Test Package",
            &[("EVP Test Tool", "evp-test@example.com")],
        )
        .expect("Failed to create package");
        let tc = package
            .create_test_case("Test Case")
            .expect("Failed to create test case");
        tc.evidence_mut().push(Evidence::new(
            "text/plain",
            EvidenceData::Text {
                content: "Hello, world!".to_string(),
            },
        ));
        package.save().expect("Failed to save package");
    }

    {
        // Read package
        let package = EvidencePackage::open(path).expect("Failed to read package");
        let it = package.test_case_iter().expect("Failed to load test cases");
        let mut count = 0;
        for tc in it {
            count += 1;
            assert_eq!(tc.metadata().title(), "Test Case");
            assert_eq!(tc.evidence().len(), 1);
            for ev in tc.evidence() {
                assert_eq!(ev.kind(), "text/plain");
                // assert_eq!(String::from_utf8_lossy(&ev.data(&mut package)), "");
            }
        }
        assert_eq!(count, 1);
    }
}
