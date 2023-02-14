use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

fn main() {
    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    let url = vec!["https://www.husky.io/quanto-custa"];
    let _ = tab.navigate_to(url[0]).unwrap();
    tab.wait_until_navigated().unwrap();

    tab.wait_for_element("input#in_source_amount")
        .unwrap()
        .click()
        .unwrap();

    tab.type_str("5021").unwrap();

    let final_value = tab
        .find_element("input#in_target_amount")
        .unwrap()
        .get_description()
        .unwrap();

    println!("{}", serde_json::to_string(&final_value).unwrap());
}
