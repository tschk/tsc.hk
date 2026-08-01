use std::path::{Path, PathBuf};

use crepuscularity_core::context::TemplateContext;
use crepuscularity_native::render_template_to_ir_with_path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("ir-gen has no parent directory")?
        .to_path_buf();

    let mut args = std::env::args().skip(1);
    let input = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("index.crepus"));
    let output = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("src/generated/view-ir.json"));

    let source = std::fs::read_to_string(&input)?;
    let ctx = TemplateContext::new();
    let ir = render_template_to_ir_with_path(&source, &ctx, Some(Path::new(&input)))?;

    if ir.version != crepuscularity_native::IR_VERSION {
        return Err(format!(
            "unexpected IR version {} (expected {})",
            ir.version,
            crepuscularity_native::IR_VERSION
        )
        .into());
    }

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut json = serde_json::to_string_pretty(&ir)?;
    json.push('\n');
    std::fs::write(&output, json)?;
    println!("wrote {}", output.display());
    Ok(())
}
