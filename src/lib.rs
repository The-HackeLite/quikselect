use scraper::ElementRef;
use scraper::Selector;
use scraper::error::SelectorErrorKind;
use scraper::html::Html;
use scraper::html::Select;


/// Create a new selector by providing selector string
pub fn selector(selector_str: &'_ str) -> Selector {
    let selector = Selector::parse(selector_str).unwrap();
    selector
}

/// Create a new selector by providing selector string.
pub fn try_selector(selector_str: &'_ str) -> Result<Selector, SelectorErrorKind<'_>>{
    let selector = Selector::parse(selector_str);
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

/// Retrieve the inner HTML of a document
pub fn inner_html(document: &Html, selector: &Selector) -> String {
    let inner_html = document.select(selector).next().unwrap().inner_html();
    inner_html
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

/// Retrieve the inner HTML of an element ref.
pub fn element_inner_html(element: &ElementRef<'_>, selector: &Selector) -> String {
    let inner_html = element.select(selector).next().unwrap().inner_html();
    inner_html
}

/// Retrieve the inner HTML of an element ref
pub fn try_element_inner_html(element: &ElementRef<'_>, selector: &Selector) -> Option<String> {
    let inner_html_element = element.select(selector).next();
    if let Some(inner_html) = inner_html_element {
        let inner_html = inner_html.inner_html();
        Some(inner_html)
    } else {
        None
    }
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
pub fn text(document: &Html, selector: &Selector) -> String {
    let text = document.select(selector).next().unwrap().text().collect::<String>();
    text
}

/// Retrieve text from a document or fragment.
pub fn try_text(document: &Html, selector: &Selector) -> Option<String> {
    let text_element = document.select(selector).next();
    if let Some(text_element) = text_element {
        let text = text_element.text().collect::<String>();
        return Some(text);
    } else {
        return None;
    }    
}

pub fn element<'a>(document: &'a Html, selector: &Selector) -> ElementRef<'a> {
    let element_ref: scraper::ElementRef<'a> = document.select(selector).next().unwrap();
    element_ref
}

pub fn try_element<'a>(document: &'a Html, selector: &Selector) -> Option<ElementRef<'a>> {
    let element_ref: Option<scraper::ElementRef<'a>> = document.select(selector).next();
    element_ref
}

pub fn select_from_element<'a>(element: ElementRef<'a>, selector: &Selector) -> ElementRef<'a> {
    let element_ref: ElementRef<'a> = element.select(selector).next().unwrap();
    element_ref
}

pub fn try_select_from_element<'a>(element: ElementRef<'a>, selector: &Selector) -> Option<ElementRef<'a>> {
    let element_ref: Option<scraper::ElementRef<'a>> = element.select(selector).next();
    element_ref
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_selector() {
        //        assert_eq!(result, 4);
    }
}
