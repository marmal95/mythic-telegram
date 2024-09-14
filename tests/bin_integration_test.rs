#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use assert_cmd::prelude::*;
    use mythic_telegram::file;
    use predicates::prelude::*;
    use std::{ffi::OsStr, fs, path::PathBuf, process::Command};

    #[derive(Debug)]
    struct TestData {
        pub working_dir: PathBuf,
        pub image_path: PathBuf,
        pub encoded_image_path: PathBuf,
        pub original_secret_file_path: PathBuf,
        pub decoded_secret_file_path: PathBuf,
    }

    impl TestData {
        fn new(test: &str) -> Self {
            let working_dir = format!("tests/test_working_dir_{}", test);
            fs::create_dir_all(&working_dir).unwrap();
            fs::copy(
                "tests/data/image.png",
                format!("{}/image.png", &working_dir),
            )
            .unwrap();

            Self {
                working_dir: PathBuf::from(&working_dir),
                image_path: format!("{}/{}", working_dir, "image.png").into(),
                encoded_image_path: format!("{}/{}", working_dir, "encoded_image.png").into(),
                original_secret_file_path: "./tests/data/secret.png".into(),
                decoded_secret_file_path: format!("{}/{}", working_dir, "secret.png").into(),
            }
        }
    }

    impl Drop for TestData {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.working_dir).unwrap();
        }
    }

    #[test]
    fn run_without_args() -> Result<()> {
        let mut cmd = Command::cargo_bin("mythic-telegram")?;
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage: mythic-telegram <COMMAND>"));
        Ok(())
    }

    #[test]
    fn run_encode_without_args() -> Result<()> {
        let mut cmd = Command::cargo_bin("mythic-telegram")?;
        cmd.arg("encode");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage: mythic-telegram encode --image-file <IMAGE_FILE> --secret-file <SECRET_FILE> <COMMAND>"));
        Ok(())
    }

    #[test]
    fn run_decode_without_args() -> Result<()> {
        let mut cmd = Command::cargo_bin("mythic-telegram")?;
        cmd.arg("decode");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Usage: mythic-telegram decode --image-file <IMAGE_FILE>",
        ));
        Ok(())
    }

    #[test]
    fn run_encode_without_mode() -> Result<()> {
        let mut cmd = Command::cargo_bin("mythic-telegram")?;
        cmd.args(&["encode", "--image-file", "tests/data/image.png"]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("error: 'mythic-telegram encode' requires a subcommand but one was not provided"))
            .stderr(predicate::str::contains("Usage: mythic-telegram encode --image-file <IMAGE_FILE> --secret-file <SECRET_FILE> <COMMAND>"));
        Ok(())
    }

    #[test]
    fn run_encode_rbg_mode_without_arg() -> Result<()> {
        let mut cmd = Command::cargo_bin("mythic-telegram")?;
        cmd.args(&[
            "encode",
            "--image-file",
            "tests/data/image.png",
            "--secret-file",
            "tests/data/secret.png",
            "rgb",
        ]);
        cmd.assert().failure().stderr(predicate::str::contains("mythic-telegram encode --image-file <IMAGE_FILE> --secret-file <SECRET_FILE> rgb --bits-per-channel <1/2/4>"));
        Ok(())
    }

    #[test]
    fn run_encode_decode_alpha() -> Result<()> {
        test_encode_decode("run_encode_decode_alpha", &["alpha"])
    }

    #[test]
    fn run_encode_decode_rgb_4bits() -> Result<()> {
        test_encode_decode(
            "run_encode_decode_rgb_4bits",
            &["rgb", "--bits-per-channel", "4"],
        )
    }

    fn test_encode_decode<I, S>(name: &str, additional_encode_args: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let test_data = TestData::new(name);
        let original = file::read_bytes(&test_data.original_secret_file_path)?;

        let mut cmd = Command::cargo_bin("mythic-telegram")?;
        cmd.args(&[
            "encode",
            "--image-file",
            test_data.image_path.to_str().unwrap(),
            "--secret-file",
            test_data.original_secret_file_path.to_str().unwrap(),
        ]);
        cmd.args(additional_encode_args);
        cmd.assert().success();

        let mut cmd = Command::cargo_bin("mythic-telegram")?;
        cmd.args(&[
            "decode",
            "--image-file",
            test_data.encoded_image_path.to_str().unwrap(),
        ]);
        cmd.assert().success();

        let decoded = file::read_bytes(&test_data.decoded_secret_file_path)?;
        assert_eq!(original, decoded);

        Ok(())
    }
}
