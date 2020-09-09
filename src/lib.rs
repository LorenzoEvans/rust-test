use seed::prelude::*;
use seed::*;

#[derive(Default)]
struct Model {
    text_to_show: String,
    // saved_items: Vec<String> // Vector to hold items.
                                // index placements will be identifiers for deletion.

}

#[derive(Clone)]
enum Msg {
    ChangeText(String),
    // We need a message to indicate clearing state
    // Clear,
    // We need a message to indicate deleting a single item
    // Delete(i32)
    // We need a message to indicate saving a single item.
    // SaveText(String)
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        ChangeText(new_text) => model.text_to_show = new_text,
        // ClearMessages
        // DeleteItem
        // SaveItem
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
