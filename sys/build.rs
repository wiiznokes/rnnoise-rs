use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::bail;

fn download_model(archive_path: &Path, model_name: &str) -> anyhow::Result<()> {
    println!("cargo:warning=Downloading {model_name}");

    let url = format!("https://media.xiph.org/rnnoise/models/{model_name}.tar.gz");

    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        bail!("download failed: {}", response.status())
    }

    let bytes = response.bytes()?;
    fs::write(&archive_path, &bytes)?;

    Ok(())
}

fn unpack(tar: &Path, dst: &Path) -> anyhow::Result<()> {
    let tar_gz = fs::File::open(tar)?;
    let decoder = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(dst)?;

    println!("cargo:warning=Model extracted to {}", dst.display());

    Ok(())
}

fn main() {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let last_model_version = "0a8755f8e2d834eff6a54714ecc7d75f9932e845df35f8b59bc52a7cfe6e8b37";

    // let model_version = fs::read_to_string("rnnoise/model_version")
    //     .unwrap()
    //     .trim()
    //     .to_string();

    let model_name = format!("rnnoise_data-{last_model_version}");

    let archive_path = dst.join(format!("{model_name}.tar.gz"));
    let extracted_archive_path = dst.join(&model_name);

    if !extracted_archive_path.exists() {
        download_model(&archive_path, &model_name).unwrap();
        unpack(&archive_path, &extracted_archive_path).unwrap();
    }

    let mut cfg = cc::Build::new();

    cfg.include("rnnoise/include");
    cfg.include("rnnoise/src");
    cfg.include(extracted_archive_path.join("src"));

    let common_c_files = [
        "denoise.c",
        "rnn.c",
        "pitch.c",
        "kiss_fft.c",
        "celt_lpc.c",
        // "dump_features.c",
        // "dump_rnnoise_tables.c",
        "nnet.c",
        "nnet_default.c",
        "parse_lpcnet_weights.c",
        // "rnnoise_data.c",
        "rnnoise_tables.c",
        // "write_weights.c",
    ];

    for f in common_c_files {
        cfg.file(Path::new("rnnoise/src").join(f));
    }

    cfg.file(extracted_archive_path.join("src/rnnoise_data.c"));

    cfg.compile("rnnoise");

    let bindings = bindgen::builder()
        .header("rnnoise/include/rnnoise.h")
        .generate()
        .unwrap();

    let mut file = File::create(dst.join("lib.rs")).unwrap();

    file.write(bindings.to_string().as_bytes()).unwrap();
}
