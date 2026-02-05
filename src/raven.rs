use std::{
    fs::{self, File},
    path::Path,
};

use scraper::{ElementRef, Html, Selector};
use xmltree::{Element, EmitterConfig, XMLNode};

use crate::models::{Package, PackageType};

const MAVEN_REPO_URL: &str = "https://mvnrepository.com";

pub fn get_remote_packages(
    package_name: &str,
    package_type: PackageType,
) -> Result<Vec<Package>, ureq::Error> {
    let processed_name = package_name.replace(" ", "+");
    let url = format!("{MAVEN_REPO_URL}/search?q={processed_name}");

    let html = get_html(&url)?;
    let grp_selector = Selector::parse("p.im-subtitle").unwrap();

    let mut packages = Vec::new();
    for element in html.select(&grp_selector) {
        let mut childrens = element.children().filter_map(ElementRef::wrap);

        let grp = childrens.next().unwrap().inner_html();

        let title = childrens.next().unwrap();
        let name = title.inner_html();
        let link = title.attr("href").unwrap();

        packages.push(Package::new(
            name,
            grp,
            link.to_string(),
            package_type.clone(),
        ));
    }

    Ok(packages)
}

pub fn update_package_version(package: &mut Package) -> Result<(), ureq::Error> {
    let url = format!("{}{}", MAVEN_REPO_URL, package.url);

    let html = get_html(&url)?;
    let version_selector = Selector::parse("a.vbtn.release").unwrap();

    let version = html.select(&version_selector).next().unwrap();
    package.version = version.inner_html();

    Ok(())
}

pub fn write_deps_to_xml(package: &Package, file_path: &Path) -> Result<(), String> {
    let parent_tag_name = if let PackageType::Dependency = package.package_type {
        "dependencies"
    } else {
        "plugins"
    };

    let mut pom_xml = get_pom_xml(file_path)?;

    let deps = pom_xml.get_mut_child(parent_tag_name);
    let cnt = package.to_xml_string();

    match deps {
        Some(deps) => {
            let cur_dep = Element::parse(cnt.as_bytes()).unwrap();

            for dep in deps.children.iter() {
                if let Some(ele1) = dep.as_element()
                    && is_package_equal(ele1, &cur_dep)
                {
                    return Err("Package Already Added".to_string());
                }
            }

            deps.children.push(XMLNode::Element(cur_dep));
        }
        None => {
            let wrapped_cnt = format!("<{parent_tag_name}>{cnt}</{parent_tag_name}>");
            let packages = Element::parse(wrapped_cnt.as_bytes()).unwrap();

            // let project = pom_xml
            //     .get_mut_child("project")
            //     .ok_or("project tag not found, invalid pom.xml")?;
            // project.children.push(XMLNode::Element(packages));
            pom_xml.children.push(XMLNode::Element(packages));
        }
    }

    pom_xml
        .write_with_config(
            File::create("pom.xml").map_err(|e| format!("cant write to pom.xml {e}"))?,
            EmitterConfig::new()
                .line_separator("\n")
                .perform_indent(true)
                .normalize_empty_elements(true),
        )
        .map_err(|e| format!("write error: {e}"))?;

    Ok(())
}

fn get_pom_xml(file_path: &Path) -> Result<Element, String> {
    let pom_content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {e}"))?;

    Element::parse(pom_content.as_bytes()).map_err(|e| format!("Failed to parse XML: {e}"))
}

fn get_html(url: &str) -> Result<Html, ureq::Error> {
    let page = ureq::get(url).call()?.body_mut().read_to_string()?;

    Ok(scraper::html::Html::parse_document(&page))
}

fn is_package_equal(ele1: &Element, ele2: &Element) -> bool {
    for (a, b) in ele1.children.iter().zip(ele2.children.iter()) {
        let a = a.as_element().unwrap();
        let b = b.as_element().unwrap();

        if a.get_text() != b.get_text() {
            return false;
        }
    }

    true
}
