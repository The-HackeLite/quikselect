use scraper;
use scraper::error::SelectorErrorKind;


/// Create a new selector by providing selector string
pub fn selector(selector_str: &'_ str) -> scraper::Selector {
    let selector = scraper::Selector::parse(selector_str).unwrap();
    selector
}

/// Create a new selector by providing selector string.
pub fn try_selector(selector_str: &'_ str) -> Result<scraper::Selector, SelectorErrorKind<'_>>{
    let selector = scraper::Selector::parse(selector_str);
    selector
}

pub fn parse_document(document: &'_ str) -> scraper::html::Html {
    let document = scraper::html::Html::parse_document(document);
    document
}

/// Retrieve the inner HTML of a document or fragment
pub fn inner_html(document: scraper::html::Html, selector: scraper::Selector) -> String {
    let inner_html = document.select(&selector).next().unwrap().inner_html();
    inner_html
}

/// Retrieve the inner HTML of a document or fragment.
pub fn try_inner_html(document: scraper::html::Html, selector: scraper::Selector) -> Option<String> {
    let inner_html_element = document.select(&selector).next();
    if let Some(inner_html) = inner_html_element {
        let inner_html = inner_html.inner_html();
        Some(inner_html)
    } else {
        None
    }
}

/// Retrieve the attribute of a document or selector.
pub fn attr(document: scraper::html::Html, selector: scraper::Selector, attribute: &'_ str) -> String {
    let attr_element = document
                                    .select(&selector)
                                    .next()
                                    .unwrap()
                                    .attr(attribute)
                                    .unwrap()
                                    .to_owned();
    attr_element
}

/// Retrieve the attribute of a document or selector.
pub fn try_attr(document: scraper::html::Html, selector: scraper::Selector, attribute: &'_ str) -> Option<String> {
    let attr_element = document.select(&selector).next();
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
pub fn text(document: scraper::html::Html, selector: scraper::Selector) -> String {
    let text = document.select(&selector).next().unwrap().text().collect::<String>();
    text
}

/// Retrieve text from a document or fragment.
pub fn try_text(document: scraper::html::Html, selector: scraper::Selector) -> Option<String> {
    let text_element = document.select(&selector).next();
    if let Some(text_element) = text_element {
        let text = text_element.text().collect::<String>();
        return Some(text);
    } else {
        return None;
    }    
}

pub fn element<'a>(document: &'a scraper::html::Html, selector: scraper::Selector) -> scraper::ElementRef<'a> {
    let element_ref: scraper::ElementRef<'a> = document.select(&selector).next().unwrap();
    element_ref
}

pub fn try_element<'a>(document: &'a scraper::html::Html, selector: scraper::Selector) -> Option<scraper::ElementRef<'a>> {
    let element_ref: Option<scraper::ElementRef<'a>> = document.select(&selector).next();
    element_ref
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector() {
        //        assert_eq!(result, 4);
    }
}
