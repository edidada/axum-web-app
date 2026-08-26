use std::fs::File;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Mapper {
    pub namespace: String,
    pub statements: HashMap<String, String>,
}

#[derive(Debug)]
pub struct XmlMapper {
    pub mappers: HashMap<String, Mapper>,
}

impl XmlMapper {
    pub fn new() -> Self {
        XmlMapper {
            mappers: HashMap::new(),
        }
    }

    pub fn load_from_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let mut reader = EventReader::new(file);
        
        let mut current_namespace = String::new();
        let mut current_statement_id = String::new();
        let mut current_statement_content = String::new();
        let mut in_statement = false;
        let mut current_mapper = None;

        for event in reader {
            match event {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    if name.local_name == "mapper" {
                        for attr in attributes {
                            if attr.name.local_name == "namespace" {
                                current_namespace = attr.value;
                                current_mapper = Some(Mapper {
                                    namespace: current_namespace.clone(),
                                    statements: HashMap::new(),
                                });
                            }
                        }
                    } else if name.local_name == "select" || name.local_name == "insert" || name.local_name == "update" || name.local_name == "delete" {
                        for attr in attributes {
                            if attr.name.local_name == "id" {
                                current_statement_id = attr.value;
                            }
                        }
                        in_statement = true;
                        current_statement_content.clear();
                    }
                }
                Ok(XmlEvent::Characters(content)) => {
                    if in_statement {
                        current_statement_content.push_str(&content);
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "select" || name.local_name == "insert" || name.local_name == "update" || name.local_name == "delete" {
                        in_statement = false;
                        if let Some(mapper) = &mut current_mapper {
                            mapper.statements.insert(current_statement_id.clone(), current_statement_content.trim().to_string());
                        }
                    } else if name.local_name == "mapper" {
                        if let Some(mapper) = current_mapper {
                            self.mappers.insert(mapper.namespace.clone(), mapper);
                        }
                        current_mapper = None;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn get_statement(&self, namespace: &str, statement_id: &str) -> Option<&String> {
        self.mappers.get(namespace)?.statements.get(statement_id)
    }
}

pub fn load_xml_mappers() -> Result<XmlMapper, Box<dyn std::error::Error>> {
    let mut xml_mapper = XmlMapper::new();
    xml_mapper.load_from_file("src/xml/paginate.xml")?;
    Ok(xml_mapper)
}