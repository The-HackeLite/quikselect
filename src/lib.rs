use scraper::html::Html;
use scraper::html::Select;
use scraper::ElementRef;
use scraper::Selector;
mod option;

pub use option::*;

/// Create a new selector by providing selector string
pub fn selector(selector_str: &'_ str) -> Selector {
    let selector = Selector::parse(selector_str).unwrap();
    selector
}

/// Get a selection as an iterator
pub fn select<'a, 'b>(document: &'a Html, selector: &'b Selector) -> scraper::html::Select<'a, 'b> {
    let selection: Select<'a, 'b> = document.select(selector);
    selection
}

pub fn parse_document(document: &'_ str) -> Html {
    let document = Html::parse_document(document);
    document
}

/// Retrieve the inner HTML of an element
pub fn element_inner_html(element: &ElementRef) -> String {
    let inner_html = element.inner_html();
    inner_html
}

/// Retrieve text from an element
pub fn element_inner_text(element: &ElementRef) -> String {
    let text = element.text()
        .collect::<String>()
        .trim()
        .to_owned();
    text
}

/// Retrieve the inner HTML of a document
pub fn inner_html(document: &Html, selector: &Selector) -> String {
    let inner_html = document.select(selector).next().unwrap().inner_html();
    inner_html
}

/// Retrieve the inner HTML of an element ref
pub fn inner_html_from_element(element: &ElementRef<'_>, selector: &Selector) -> String {
    let inner_html = element.select(selector).next().unwrap().inner_html();
    inner_html
}

/// Retrieve the attribute of a document or selector.
pub fn attr(document: &Html, selector: &Selector, attribute: &'_ str) -> String {
    let attr_element = document
        .select(selector)
        .next()
        .unwrap()
        .attr(attribute)
        .unwrap()
        .to_owned();
    attr_element
}

/// Retrieve text from a document or fragment.
pub fn text(document: &Html, selector: &Selector) -> String {
    let text = document
        .select(selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_owned();
    text
}

pub fn element<'a>(document: &'a Html, selector: &Selector) -> ElementRef<'a> {
    let element_ref: scraper::ElementRef<'a> = document.select(selector).next().unwrap();
    element_ref
}

/// Retrieve text from an element.
pub fn text_from_element<'a>(element: ElementRef<'a>, selector: &Selector) -> String {
    let text = element
        .select(selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_owned();
    text
}

/// Retrieve the attribute of an element.
pub fn attr_from_element<'a>(element: ElementRef<'a>, attribute: &'_ str) -> String {
    let attr_element = element.attr(attribute).unwrap().to_owned();
    attr_element
}

/// Get a selection as an iterator from an element
pub fn select_from_element<'a>(element: ElementRef<'a>, selector: &Selector) -> ElementRef<'a> {
    let element_ref: ElementRef<'a> = element.select(selector).next().unwrap();
    element_ref
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_selector() {
        //        assert_eq!(result, 4);
    }
}
