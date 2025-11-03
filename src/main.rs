use lopdf::Document; 

fn main() {
    let file = "peugeot_partner2.pdf"; 
    let doc = Document::load(file);
    match doc {
        Ok(document) => {
            let pages = document.get_pages();
            println!("Total pages: {:?}", &pages.len());
        }
        Err(err) => {
            eprint!("{err}")
        }
    }
}
