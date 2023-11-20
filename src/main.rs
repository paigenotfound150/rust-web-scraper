
//Below is all the useful information we want to extract - to make things easier for ourselves
// we'll make a struct called Recipe to store our info (Rust is not an OOP language, no classes)
#[derive(Debug)]
struct Recipe {
    url: Option<String>, //href
    title: Option<String>, //span.card__title-text
    image: Option<String>, //img data-src
}


fn main() {
    // Get target webpage and parse into html
    let response = reqwest::blocking::get("https://www.allrecipes.com/recipes/198/holidays-and-events/thanksgiving/");
    let html_content = response.unwrap().text().unwrap();
    // Above Reqwest is sending an HTTP GET to the url . blocking ensures it happens so that execution only occurs when you get a response from the server. 
    // 
    
    // Get the data we need, AKA each recipe
    //Below converts to an html tree that's easier to parse
    let document = scraper::Html::parse_document(&html_content);

    //parse defines a Scraper selector object. It's passed to select to select those elemnts from the html document
    let html_recipe_selector = scraper::Selector::parse("a.mntl-card").unwrap();

       // We want a dynamic array of our recipes (contiguous growable array is Vec)
    let mut recipes: Vec<Recipe> = Vec::new();
    for html_recipe in document.select(&html_recipe_selector) {

        //create a new recipe object
        let url = html_recipe.attr("href")
        .map(str::to_owned);

        let title = html_recipe
        .select(&scraper::Selector::parse("span.card__title-text").unwrap())
        .next()
        .map(|span| span.text().collect::<String>());

        let image = html_recipe
        .select(&scraper::Selector::parse("img").unwrap())
        .next()
        .and_then(|img| img.value().attr("data-src"))
        .map(str::to_owned);
  
        let new_recipe = Recipe {
            url, 
            title,
            image
        };
        recipes.push(new_recipe);
    }

    let path = std::path::Path::new("recipes.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer.write_record(&["url","title","image"]).unwrap();

    for recipe in recipes {
        let url = recipe.url.unwrap();
        let title = recipe.title.unwrap();
        let image = recipe.image.unwrap();
        writer.write_record(&[url,title,image]).unwrap();
    }
    writer.flush().unwrap();


}