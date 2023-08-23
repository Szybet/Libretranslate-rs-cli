use futures::executor;
use libretranslate::{translate, translate_url, Language, Translate};

// C
use core::ffi::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn translate_text_c(
    text: *const c_char,
    from: *const c_char,
    to: *const c_char,
    url: *const c_char,
    api_key: *const c_char,
) -> *const c_char {
    let text_string = CStr::from_ptr(text).to_str().unwrap().to_string();
    let from_string = CStr::from_ptr(from).to_str().unwrap().to_string();
    let to_string = CStr::from_ptr(to).to_str().unwrap().to_string();
    let url_string = CStr::from_ptr(url).to_str().unwrap().to_string();
    let api_key_string = CStr::from_ptr(api_key).to_str().unwrap().to_string();

    let str_fin = translate_text(text_string, from_string, to_string, url_string, api_key_string);

    return Box::leak(CString::new(str_fin).unwrap().into_boxed_c_str()).as_ptr();
}

pub fn translate_text(
    text: String,
    from: String,
    to: String,
    url: String,
    api_key: String,
) -> String {
    let from_fin;
    if let Ok(lang_from) = Language::from(&from) {
        from_fin = lang_from;
    } else {
        return format!("From language not found or not supported: {}", from);
    }

    let to_fin;
    if let Ok(lang_to) = Language::from(&to) {
        to_fin = lang_to;
    } else {
        return format!("To language not found or not supported: {}", from);
    }

    let mut key = Option::None;

    if !api_key.is_empty() {
        key = Some(api_key);
    }

    let data = executor::block_on(translate_url(from_fin, to_fin, text, url, key));

    match data {
        Ok(data) => data.output,
        Err(error) => error.to_string(),
    }
}

#[test]
fn en_pn() {
    println!(
        "{}",
        translate_text(
            "I want this to be in polish".to_string(),
            "en".to_string(),
            "pl".to_string(),
            "https://translate.argosopentech.com/".to_string(),
            "".to_string()
        )
    );
}
