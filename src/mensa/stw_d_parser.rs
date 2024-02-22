use super::{Food, Menu};
use anyhow::{anyhow, Ok, Result};
use chrono::{DateTime, Local};
use leptos::leptos_dom::logging::console_log;
use reqwest::Client;
use scraper::{Html, Selector};

pub async fn get_menu_data(id: &str, client: &Client, date: DateTime<Local>) -> Result<Menu> {
    let url = format!("https://www.stw-d.de/gastronomie/speiseplaene/{}", id);

    let text = client.get(url).send().await?.text().await?;

    let html = Html::parse_document(text.as_str());

    Ok(get_food_from_html(html, date)?)
}

fn get_food_from_html(html: Html, date: DateTime<Local>) -> Result<Menu> {
    let date_formatted = format!("div[data-date=\"{}\"]", date.format("%d.%m.%Y"));

    let selector =
        Selector::parse(&date_formatted).map_err(|_| anyhow!("failed to parse selector"))?;

    let day = html
        .select(&selector)
        .next()
        .ok_or(anyhow!("no day found"))?;

    let essens_selector =
        Selector::parse("div.counter").map_err(|_| anyhow!("failed to parse selector"))?;

    let essen = day.select(&essens_selector);

    let mut food_vec: Vec<Food> = Vec::new();

    for e in essen {
        console_log(&format!("e: {:?}", e.html()));
        let name_selector =
            Selector::parse(r#"li"#).map_err(|_| anyhow!("failed to parse selector 1"))?;
        let url_selector =
            Selector::parse("div").map_err(|_| anyhow!("failed to parse selector 2"))?;

        let mut is_vegan = false;

        let mut food_name = e
            .select(&name_selector)
            .next()
            .ok_or(anyhow!("no name found"))?
            .inner_html()
            .split(',')
            .collect::<Vec<_>>()[0]
            .to_string();
        let url = e
            .select(&url_selector)
            .next()
            .ok_or(anyhow!("no url found"))?
            .inner_html();

        let img_url = url.split("url(").collect::<Vec<_>>()[1]
            .split(')')
            .collect::<Vec<_>>()[0]
            .replace('\"', "");

        if food_name.contains(" [V] ") {
            is_vegan = true;
            food_name = food_name.replace(" [V] ", "");
        }

        let food_name_truncated = if let Some(index) = food_name.find('(') {
            food_name[..index].to_string()
        } else {
            food_name
        };

        food_vec.push(Food {
            name: food_name_truncated,
            image_url: img_url,
            vegan: is_vegan,
        })
    }

    for food in &food_vec {
        console_log(&format!("food: {:?}", food));
    }

    Ok(Menu::Open(food_vec))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_menu_data_with_date() {
        let client = Client::new();
        let date = chrono::Local::now();
        let menu = get_menu_data("essenausgabe-sued-duesseldorf", &client, date).await;
        assert!(menu.is_ok());
    }
}
