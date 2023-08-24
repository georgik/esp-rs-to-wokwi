mod cli;

use cli::Opt;
use std::fs::File;
use std::io::Read;
use structopt::StructOpt;
use syn::{visit::Visit, ItemFn, ExprCall, LocalInit, Stmt, Attribute};

struct SpiVisitor {
    pin_names: Vec<String>,
}

impl<'ast> Visit<'ast> for SpiVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast ItemFn) {
        if item_fn.sig.ident == "main" && item_fn.attrs.iter().any(|attr| has_entry_attribute(attr)) {
            for stmt in &item_fn.block.stmts {
                if let Stmt::Local(local) = stmt {
                    if let Some(LocalInit { expr, .. }) = &local.init {
                        let expr = &**expr;
                        if let syn::Expr::Call(ExprCall { args, .. }) = expr {
                            // Assuming the GPIO pins are the 2nd to 5th arguments of the constructor
                            for (i, arg) in args.iter().skip(1).take(4).enumerate() {
                                if let syn::Expr::Path(path) = arg {
                                    let last_segment = path.path.segments.last().unwrap();
                                    let pin_ident = last_segment.ident.to_string();
                                    if pin_ident.starts_with("gpio") {
                                        let pin_number = &pin_ident["gpio".len()..];
                                        self.pin_names
                                            .push(format!("[ \"esp:D{}\", \"lcd1:{}\", [] ],", pin_number, self.pin_names[i]));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn has_entry_attribute(attr: &Attribute) -> bool {
    attr.path().is_ident("entry")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let mut file = File::open(&opt.file_path)?;

    let mut code = String::new();
    file.read_to_string(&mut code)?;

    // Parse the code into an AST
    let ast: syn::File = syn::parse_str(&code)?;

    // Define the names corresponding to the pin positions in the SPI constructor
    let pin_names = vec!["SCK".into(), "MOSI".into(), "MISO".into(), "CS".into()];

    // Traverse the AST to find the desired information
    let mut visitor = SpiVisitor { pin_names };
    visitor.visit_file(&ast);

    for pin in visitor.pin_names {
        println!("{}", pin);
    }

    Ok(())
}
