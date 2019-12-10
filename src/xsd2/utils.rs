use roxmltree::*;

pub fn find_child<'a, 'input>(node: &roxmltree::Node<'a, 'input>, tag_name: &str) -> Option<roxmltree::Node<'a, 'input>> {
    node.children().find(|e| e.is_element() && e.tag_name().name() == tag_name)
}

pub fn find_element<'a, 'input>(node: &roxmltree::Node<'a, 'input>, tag_name: &str) -> Option<roxmltree::Node<'a, 'input>> {
    match node.
        traverse().
        find(|e| match e {
            Edge::Open(x) => x.is_element() && x.tag_name().name() == tag_name,
            _ => false
        })
    {
        Some(Edge::Open(node)) => Some(node.clone()),
        _ => None
    }
}

pub fn get_documentation<'a>(node: &roxmltree::Node<'a, '_>) -> Option<&'a str> {
    find_child(node, "annotation").
        and_then(|node| find_child(&node, "documentation")).
        and_then(|node| node.text())
}

pub fn get_node_type<'a>(node: &roxmltree::Node<'a, '_>) -> &'a str {
    match node.attribute("type") {
        Some(name) => name,
        None => match node.attribute("ref") {
            Some(s) => s,
            None => "_UNSUPPORTED_TYPE"
        }
    }
}

pub fn get_node_name<'a>(node: &roxmltree::Node<'a, '_>) -> &'a str {
    match node.attribute("name") {
        Some(name) => name,
        None => match node.attribute("ref") {
            Some(s) => s,
            None => "_UNSUPPORTED_NAME"
        }
    }
}

pub type MinOccurs = usize;
pub enum MaxOccurs {
    Bounded(usize),
    Unbounded,
    None
}

pub fn max_occurs(node: &roxmltree::Node<'_, '_>) -> MaxOccurs {
    match node.attribute("maxOccurs") {
        Some(s) => match s {
            "unbounded" => MaxOccurs::Unbounded,
            s => s.
                parse::<usize>().
                ok().
                map(|val| MaxOccurs::Bounded(val)).
                unwrap_or(MaxOccurs::None)
        },
        None => MaxOccurs::None
    }
}

pub fn min_occurs(node: &roxmltree::Node<'_, '_>) -> MinOccurs {
    node.attribute("minOccurs").and_then(|v| v.parse::<usize>().ok()).unwrap_or(1)
}

pub type AnyAttribute<'a, 'input> = roxmltree::Node<'a, 'input>;