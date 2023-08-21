//! manage and codegen depedencies during codegen
use common::{get_mission_params, open_file};
use quote::format_ident;
use seds_rs::{
    codegen::{
        dependency::{AstNode, QualifiedNameIter},
        format::{format_pascal_case, format_snake_case},
    },
    eds::{ast::QualifiedName, raw, resolve::Resolve},
};

mod common;

#[test]
fn test_spacepacket() {
    let contents = open_file("eds/cFE/modules/core_api/eds/ccsds_spacepacket.xml");
    let rpf: raw::PackageFile = serde_xml_rs::from_str(&contents).unwrap();

    let ectx = get_mission_params();
    let pf = rpf.resolve(&ectx).unwrap();

    let qni = QualifiedNameIter::new(AstNode::PackageFile(&pf));

    let mut qnames: Vec<&QualifiedName> = qni.into_iter().collect();
    qnames.dedup();
    for path in qnames.iter() {
        let segments = path.0.split('/').collect::<Vec<_>>();

        match segments.len() {
            1 => (),
            2 => {
                // Module and identifier
                let module_ident = format_ident!("{}", segments[0]);
                let snake_module = format_snake_case(&module_ident).unwrap();
                let ident = format_ident!("{}", segments[1]);
                let pascal_ident = format_pascal_case(&ident).unwrap();
                println!(
                    "use {}::{};",
                    snake_module.to_string(),
                    pascal_ident.to_string()
                );
            }
            _ => (),
        }
    }
}
