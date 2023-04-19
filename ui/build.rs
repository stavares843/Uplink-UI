use glob::glob;
use rsass::{compile_scss, output};
use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let version = rustc_version::version().unwrap();

    if cfg!(feature = "production_mode") {
        // the command: `rustup install 1.68.2` will ensure that the compiler matches
        if version.major != 1 || version.minor != 68 || version.patch != 2 {
            panic!("rustc version != 1.68.2");
        }
    }

    println!("cargo:rustc-env=RUSTC_VERSION={}", version);

    #[cfg(windows)]
    {
        //https://github.com/rust-lang/rfcs/blob/master/text/1665-windows-subsystem.md
        println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
        let mut res = winres::WindowsResource::new();
        res.set("ProductName", "uplink");
        res.set("FileDescription", "uplink");
        res.set(
            "LegalCopyright",
            "Creative Commons Attribution-NonCommercial 1.0",
        );
        res.set_icon("./extra/windows/uplink.ico");
        res.compile()
            .expect("Failed to run the Windows resource compiler (rc.exe)");
    }

    // Create the file that will hold the compiled CSS.
    let scss_output = "./src/compiled_styles.css";
    let mut scss = File::create(scss_output)?;

    // Create the string that will hold the concatenated contents of all SCSS files.
    let mut contents =
        String::from("/* This file is automatically generated, edits will be overwritten. */\n");

    // Use glob to read all SCSS files in the `src` directory and its subdirectories.
    let entries = glob("src/**/*.scss").map_err(|e| format!("Failed to read glob pattern: {e}"))?;

    // Concatenate the contents of each SCSS file into the `contents` string.
    for entry in entries {
        let path = entry?;
        let data = fs::read_to_string(path)?;
        contents += data.as_ref();
    }

    // Set the format for the compiled CSS.
    let format = output::Format {
        style: output::Style::Compressed,
        ..Default::default()
    };

    // Compile the SCSS string into CSS.
    let css = compile_scss(contents.as_bytes(), format)?;

    // Write the compiled CSS to the `scss` file.
    scss.write_all(&css)?;
    scss.flush()?;

    // zip the 'prism_langs' directory for building an installer
    let zip_dest = Path::new("wix").join("prism_langs.zip");
    let file = File::create(zip_dest).expect("failed to create zip file");

    let src_dir = Path::new("extra").join("prism_langs");
    let walkdir = WalkDir::new(&src_dir);
    let it = walkdir.into_iter();
    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        &src_dir.to_string_lossy(),
        file,
        zip::CompressionMethod::BZIP2,
    )
    .expect("failed to zip assets");

    if !cfg!(target_os = "windows") {
        return Ok(());
    }

    // make things for the wix installer
    let zip_stuff = |name: &str| {
        let zip_name = format!("{name}.zip");
        let error_message = format!("failed to zip {name}");
        let zip_dest = Path::new("wix").join(zip_name);
        let file = File::create(zip_dest).expect("failed to create zip file");

        let src_dir = Path::new("extra").join(name);
        let walkdir = WalkDir::new(&src_dir);
        let it = walkdir.into_iter();
        zip_dir(
            &mut it.filter_map(|e| e.ok()),
            &src_dir.to_string_lossy(),
            file,
            zip::CompressionMethod::BZIP2,
        )
        .expect(&error_message);
    };

    zip_stuff("prism_langs");

    Ok(())
}
// taken from here: https://github.com/zip-rs/zip/blob/master/examples/write_dir.rs
fn zip_dir<T>(
    it: &mut dyn Iterator<Item = walkdir::DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + std::io::Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = zip::write::FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
