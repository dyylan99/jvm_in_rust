use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;

pub struct BootstrapMethodsAttribute {
    bootstrap_methods: Vec<BootstrapMethod>,
}

impl AttributeInfo for BootstrapMethodsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let num_bootstrap_methods = reader.read_u16();
        let mut bootstrap_methods = vec![];
        for _i in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod {
                bootstrap_method_ref: reader.read_u16(),
                bootstrap_arguments: reader.read_u16s(),
            });
        }
        self.bootstrap_methods = bootstrap_methods;
    }
}

pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}