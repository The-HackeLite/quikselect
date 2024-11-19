use scraper::error::SelectorErrorKind;
use scraper::html::Html;
use scraper::html::Select;
use scraper::ElementRef;
use scraper::Selector;
/// Create a new selector by providing selector string.
pub fn try_selector(selector_str: &'_ str) -> Result<Selector, SelectorErrorKind<'_>> {
    let selector = Selector::parse(selector_str);
    selector
}

/// Retrieve the inner HTML of a document
pub fn try_inner_html(document: &Html, selector: &Selector) -> Option<String> {
    let inner_html_element = document.select(selector).next();
    if let Some(inner_html) = inner_html_element {
        let inner_html = inner_html.inner_html();
        Some(inner_html)
    } else {
        None
    }
}
/// Retrieve the inner HTML of an element ref
pub fn try_inner_html_from_element(element: &ElementRef<'_>, selector: &Selector) -> Option<String> {
    let inner_html_element = element.select(selector).next();
    if let Some(inner_html) = inner_html_element {
        let inner_html = inner_html.inner_html();
        Some(inner_html)
    } else {
        None
    }
}

/// Retrieve the attribute of a document or selector.
pub fn try_attr(document: &Html, selector: &Selector, attribute: &'_ str) -> Option<String> {
    let attr_element = document.select(selector).next();
    if let Some(attr_element) = attr_element {
        let attr = attr_element.attr(attribute);
        if let Some(attr) = attr {
            let attr = attr.to_owned();
            return Some(attr);
        } else {
            return None;
        }
    } else {
        return None;
    }
}
/// Retrieve text from a document or fragment.
pub fn try_text(document: &Html, selector: &Selector) -> Option<String> {
    let text_element = document.select(selector).next();
    if let Some(text_element) = text_element {
        let text = text_element.text().collect::<String>().trim().to_owned();
        return Some(text);
    } else {
        return None;
    }
}

pub fn try_element<'a>(document: &'a Html, selector: &Selector) -> Option<ElementRef<'a>> {
    let element_ref: Option<scraper::ElementRef<'a>> = document.select(selector).next();
    element_ref
}

/// Retrieve text from an element.
pub fn try_text_from_element<'a>(element: ElementRef<'a>, selector: &Selector) -> Option<String> {
    let text_element = element.select(selector).next();
    if let Some(text_element) = text_element {
        let text = text_element.text().collect::<String>().trim().to_owned();
        return Some(text);
    } else {
        return None;
    }
}

/// Retrieve the attribute of an element.
pub fn try_attr_from_element<'a>(element: ElementRef<'a>, attribute: &'_ str) -> Option<String> {
    let attr_element = element.attr(attribute);
    if let Some(attr) = attr_element {
        let attr = attr.to_owned();
        return Some(attr);
    } else {
        return None;
    }
}

/// Get a selection as an iterator from an element
pub fn try_select_from_element<'a>(
    element: ElementRef<'a>,
    selector: &Selector,
) -> Option<ElementRef<'a>> {
    let element_ref: Option<scraper::ElementRef<'a>> = element.select(selector).next();
    element_ref
}
