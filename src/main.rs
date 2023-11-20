use scraper::Selector;


//Below is all the useful information we want to extract - to make things easier for ourselves
// we'll make a struct called Recipe to store our info (Rust is not an OOP language, no classes)
struct Recipe {
    title: Option<String>, //span.card__title-text
    star_rating: Option<String>, //div.mntl-recipe-star-racting_1-0
    ratings_total: Option<u32>, //div.recipe-card-meta__rating-count-number
    url: Option<String>, //data-tax-levels href
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

    for html_recipe in document.select(&html_recipe_selector) {

        //create a new recipe object
        let url = html_recipe.attr("href")
        .map(str::to_owned);

        let image = html_recipe
        .select(&scraper::Selector::parse("img").unwrap())
        .next()
        .and_then(|img| img.value().attr("data-src"))
        .map(str::to_owned);
  
        println!("{:?}", image);
    }
   // println!("{:?}", html_recipes);
   // We want a dynamic array of our recipes (contiguous growable array is Vec)
   let mut recipes: Vec<Recipe> = Vec::new();

   // Iterate over list of recipes and scrape the data we want


}