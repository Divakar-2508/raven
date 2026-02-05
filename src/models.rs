#[derive(Debug, Clone)]
pub enum PackageType {
    Dependency,
    Plugin,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub group: String,
    pub url: String,
    pub version: String,
    pub package_type: PackageType,
}

impl Package {
    pub fn new(name: String, group: String, url: String, package_type: PackageType) -> Self {
        Self {
            name,
            group,
            url,
            version: String::new(), // filled later
            package_type,
        }
    }

    pub fn to_xml_string(&self) -> String {
        match self.package_type {
            PackageType::Dependency => {
                format!(
                    "<dependency>\
                        <groupId>{}</groupId>\
                        <artifactId>{}</artifactId>\
                        <version>{}</version>\
                     </dependency>",
                    self.group, self.name, self.version
                )
            }
            PackageType::Plugin => {
                format!(
                    "<plugin>\
                        <groupId>{}</groupId>\
                        <artifactId>{}</artifactId>\
                        <version>{}</version>\
                     </plugin>",
                    self.group, self.name, self.version
                )
            }
        }
    }
}
