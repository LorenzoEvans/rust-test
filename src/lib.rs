use seed::prelude::*;
use seed::*;

#[derive(Default)]
struct Model {
    text_to_show: String,
    selected_item: ItemId,
    pub saved_items: Vec<String> // Vector to hold items.
    // index placements will be identifiers for deletion.
    
}
#[derive(Default)]
struct ItemId {
    id: i32
}
#[derive(Clone)]
enum Msg {
    ChangeText(String),
    // We need a message to indicate clearing state
    Clear,
    // We need a message to indicate deleting a single item
    Delete(usize),
    // We need a message to indicate saving a single item.
    SaveItem(String)
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        ChangeText(new_text) => model.text_to_show = new_text,
        Clear => {
            let reset_vector: Vec<String> = Vec::new();
            // Create a new vector.
            model.saved_items = reset_vector;
            // Set old vector value to new empty vector.
        }
        Delete(item_id) => {

            let mut update_vec = model.saved_items.clone();
            // Clone the old saved items vector off the heap, so as to
            // not mutate state directly.
            update_vec.remove(item_id);
            // Remove desired item from the update vector based on index.
            model.saved_items = update_vec;
            // Set old vector to new update vector.
        }
        SaveItem(string) => {
            model.saved_items.push(string);
        }
    }
}


fn view(model: &Model) -> Node<Msg> {
    div![
        input![
            attrs! {
                At::Placeholder => "Enter some text..."
            },
            input_ev(Ev::Input, Msg::ChangeText),
            // It's likely we can add more html elements here.
        ],
        div![&model.text_to_show]
    ]
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
    // This is the function imported as a JS function in the HTML file.
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
