
//Below is all the useful information we want to extract - to make things easier for ourselves
// we'll make a struct called Recipe to store our info (Rust is not an OOP language, no classes)
#[derive(Debug)]
struct Recipe {
    url: String, //href
    title: String, //span.card__title-text
    image: String, //img data-src
    bio: String, // id = article-heading_1-0
    times: String,
    ingredients: String,
}

#[derive(Debug)]
struct RecipeDetail {
    times: String, // div.mntl-recipe-details__value"
    bio: String, // id = article-heading_1-0
    ingredients: String, // li class = mntl-structured-ingredients__list-item
}


fn main() {

    // This function is responsible for going into each recipe's url and retrieving the 
    // bio, preparation times, and the ingredients.
    fn fetchDetails(url: String) -> RecipeDetail {
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&response);

        //Retrieivng bio
        let bio_selector = scraper::Selector::parse("p#article-subheading_1-0").unwrap();
        let bio_contents = document.select(&bio_selector).next().map(|x| x.inner_html());
        let bio = match bio_contents {
            None => "No bio",
            Some(ref x) => x,
        }.to_string();

        //Retrieving prep times
        let times_selector = scraper::Selector::parse("div.mntl-recipe-details__value").unwrap();
        let times_string = document.select(&times_selector).map(|x| x.inner_html());
        let mut times = "".to_owned();
        times_string
            .zip(1..5)
            .for_each(|(item, id)| {
                times.push_str(&item);
                //Delimiter
                times.push_str("!");
        });
        
        let ingredients_selector = scraper::Selector::parse("li.mntl-structured-ingredients__list-item").unwrap();
        let ingredients_string = document.select(&ingredients_selector).map(|x| x);

        // Have to grab each individual span for the ingredients
        let mut ingredients = "".to_owned();
        for ingredient_html in ingredients_string {
            let mut ingredient = "".to_owned();
            let span_selector = scraper::Selector::parse("span").unwrap();
            let span_string = ingredient_html.select(&span_selector).map(|x| x.inner_html());
            span_string.for_each(|span| {
                ingredient.push_str(&span);
                ingredient.push_str(" ");
            });
            println!("{}", ingredient);
            ingredients.push_str(&ingredient);
            //Delimiter
            ingredients.push_str("!");
        }

        let new_recipe_details = RecipeDetail {
            times,
            bio,
            ingredients,
        };
        return new_recipe_details;
    }





    // This function is responsible for scraping the allrecipes ingredients page, and retrieving a list of recipes that
    // contains the recipe title, url and image.
    fn fetchData(url: String, fileName: String) {

        let response = reqwest::blocking::get(url);
        let html_content = response.unwrap().text().unwrap();
        // Above Reqwest is sending an HTTP GET to the url . blocking ensures it happens so that execution only occurs when you get a response from the server. 
    
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
            .map(str::to_owned)
            .unwrap();

            let title = html_recipe
            .select(&scraper::Selector::parse("span.card__title-text").unwrap())
            .next()
            .map(|span| span.text().collect::<String>())
            .unwrap();

            let image_string = html_recipe
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("data-src"))
            .map(str::to_owned);

            let image = match image_string {
                None => "NO image",
                Some(ref x) => x,
            }.to_string();
    

            //Here we call our fetchDetails function, and grab more details (bio, prep times) from the recipe's 
            //actual url to add to our recipe object.
            let crawler_url = url.clone();
            let recipe_details = fetchDetails(crawler_url);  //call function with url
            let bio = recipe_details.bio;
            let times = recipe_details.times;
            let ingredients = recipe_details.ingredients;
  
            let new_recipe = Recipe {
                url, 
                title,
                image,
                bio,
                times,
                ingredients,
            };
            recipes.push(new_recipe);
        }
        let pathStr = format!("../ui-scraper/public/recipes/{}.csv", fileName);
        let path = std::path::Path::new(&pathStr);
        let mut writer = csv::Writer::from_path(path).unwrap();

        writer.write_record(&["url","title","image", "bio", "times", "ingredients"]).unwrap();

        for recipe in recipes {
            let url = recipe.url;
            let title = recipe.title;
            let image = recipe.image;
            let bio = recipe.bio;
            let times = recipe.times;
            let ingredients = recipe.ingredients;

            writer.write_record(&[url,title,image, bio, times, ingredients]).unwrap();
        }
        writer.flush().unwrap();
    }

    fetchData("https://www.allrecipes.com/recipes/198/holidays-and-events/thanksgiving/".to_owned(), "holidays".to_owned());
    fetchData("https://www.allrecipes.com/recipes/201/meat-and-poultry/chicken/".to_owned(), "chicken".to_owned());
    fetchData("https://www.allrecipes.com/recipes/200/meat-and-poultry/beef/".to_owned(), "beef".to_owned());
    fetchData("https://www.allrecipes.com/recipes/93/seafood/".to_owned(), "seafood".to_owned());
    fetchData("https://www.allrecipes.com/recipes/95/pasta-and-noodles/".to_owned(), "pasta".to_owned());
    fetchData("https://www.allrecipes.com/recipes/1059/fruits-and-vegetables/vegetables/".to_owned(), "veggies".to_owned());
}