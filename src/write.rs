use color_eyre::{ eyre::WrapErr, Result} ;
use std::io::{ Read, Write, SeekFrom, Seek };
use std::path::PathBuf;
use edit::{ edit_file, Builder };


const TEMPLATE: &[u8; 2] = b"# ";

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<()> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&garden_path)
        .wrap_err("Failed to create wip file")?
        .keep()
        .wrap_err("Failed to keep tempfile")?;

    file.write_all(TEMPLATE)?;
    edit_file(filepath)?;
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0));
    file.read_to_string(&mut contents)?;

    let document_title = title.or_else(|| {
        contents
            .lines()
            .find(|v| v.starts_with("# "))
            .map(|maybe_line| {
                maybe_line
                    .trim_start_matches("# ")
                    .to_string()
            })
    });

    dbg!(contents, document_title);
    todo!()
}
