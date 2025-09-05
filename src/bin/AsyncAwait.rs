use trpl::Html;
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

     //the future returned by the page title is passed directly into run and once it is completed
    //match on the resulting Option<String>
    trpl::run(async {
        let url = &args[1];
        match page_title(url).await{
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} has no title"),
        }
    })


}
//First, we define a function named page_title and mark it with the async keyword
//We use the trpl::get() function to fetch whatever URL is passed in and add the await keyword to await
//a response. To get the text of the response we call its text method
//and once again await it with the await keyword
//Both the steps are asynchronous
//For the get function, we have to wait for the server to send back the first part of its response
//which will include HTTP Headers, cookies
//Once there is a response_text, it can be parsed into an instance of the Html type using Html::parse
//instead of a raw string, there is now a datatype that can be used to work with HTM
//One can use the select_first method to find the first instance of a given CSS selector
//By passing the string "title" one gets the first <title> element in the document if there is one
//Because there may be no matching element, select_first returns  an Option<ElementRef>
//Finally there is use of Option::map method, to let one work with the item on the option
//In the body of the function, we supply to map, we call inner_html on the title_element
//to get its content which is a string