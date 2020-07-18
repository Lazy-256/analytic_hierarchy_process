#![windows_subsystem = "windows"]
extern crate azul;

use azul::prelude::*;
#[cfg(debug_assertions)]
use azul::{prelude::*, widgets::button::Button, widgets::label::Label, widgets::text_input::TextInput, prelude::Dom, prelude::NodeType::* };
use std::time::Duration;

#[derive(Debug)]
struct HierarchyDataModel {
    name: azul::widgets::text_input::TextInputState,
    labels: Vec<Vec<String>>,
}

impl Default for HierarchyDataModel {
    fn default() -> Self {
        Self {
            name: azul::widgets::text_input::TextInputState::new("Name of the Hierarchy..."),          
            labels: vec![vec!["Goal".to_string()],
                vec!["Criteria 1".to_string(),"Criteria 2".to_string()],
                vec!["Alt 1".to_string(),"Alt 2".to_string(),"Alt 3".to_string()]], 
           
        }
    }
}
//VIEW -------------------------------------------------------------------------------------------------------------------------------

//css стили для нашего DOM
//const CUSTOM_CSS: &str = "";

//Трейт для элементов потомков корневого DataModel
// trait Layout<T> {
//     //Создает елемент DOM на основе данных потомка с типом Т родителя.
//     fn layout(&self, info: azul::prelude::LayoutInfo<T>, root: &T) -> azul::prelude::Dom<T> where T: Sized + azul::prelude::Layout;
// }

impl Layout for HierarchyDataModel {
    fn layout(&self, info: azul::prelude::LayoutInfo<Self>) -> azul::prelude::Dom<Self> {
        let hierarchy_name = TextInput::new()
            .bind(info.window, &self.name, &self)
            .dom(&self.name)
            .with_id("input_field");

        let hierarchy_name_block = Dom::new(Div).with_id("filename_wrapper")
            .with_child( Label::new("Hierarchy Name:").dom().with_id("input_label") )
            .with_child(hierarchy_name);
        
        let control_pannel_block = Dom::new(Div).with_id("control_pannel_wrapper")
            .with_child(hierarchy_name_block);

        let mut hierarchy_block = Dom::new(Div).with_class("hierarchy");

        let h_root = &self.labels[0][0];
        
        let h_root_block = Dom::new(Div).with_class("level")
            .with_child(Label::new( h_root.to_string() ).dom().with_class("item"))
            .with_child(Dom::new(Div).with_class("hierarchy-level-form")
                .with_child(Dom::new(Div).with_class("flex-form-group xs")
                    .with_child(Dom::new(Div).with_class("flex-field")
                        .with_child(Button::with_label("Add item").dom().with_class("btn btn-xs")))
                    .with_child(Dom::new(Div).with_class("flex-field")
                        .with_child(Button::with_label("Del item").dom().with_class("btn btn-xs")))
                    .with_child(Dom::new(Div).with_class("flex-field")
                        .with_child(Button::with_label("Estimate").dom().with_class("btn btn-xs")))
                    )
                );

        hierarchy_block = hierarchy_block.with_child(h_root_block);    
        for lev in &self.labels[1..] {
            let mut lev_block = Dom::new(Div).with_class("level");

            for item in lev {
                lev_block = lev_block.with_child(Label::new( item.to_string() ).dom().with_class("item"));
            }
            lev_block = lev_block.with_child(Dom::new(Div).with_class("hierarchy-level-form")
                .with_child(Dom::new(Div).with_class("flex-form-group xs")
                    .with_child(Dom::new(Div).with_class("flex-field")
                        .with_child(Button::with_label("Add item").dom().with_class("btn btn-xs").with_callback(On::MouseUp, Callback(add_hierarchy_layer))))
                    .with_child(Dom::new(Div).with_class("flex-field")
                        .with_child(Button::with_label("Del item").dom().with_class("btn btn-xs").with_callback(On::MouseUp, Callback(del_hierarchy_layer))))
                    .with_child(Dom::new(Div).with_class("flex-field")
                        .with_child(Button::with_label("Estimate").dom().with_class("btn btn-xs")))
                    )
                );
            hierarchy_block = hierarchy_block.with_child(lev_block);
            //println!("H: {}", &hierarchy_block.debug_dump());
        }         

        let hierarchy_main_pannel_block = Dom::new(Div).with_class("hierarchy_main_pannel_wrapper")
            .with_child(hierarchy_block);

        //Создаем корневой DOM элемент в который помещяем наши UI элементы
        let result = Dom::new(Div).with_id("wrapper")
            .with_child(hierarchy_main_pannel_block)
            .with_child(control_pannel_block);

        println!("{}", result.debug_dump());

        result
    }
}

fn add_hierarchy_layer(state: &mut AppState<HierarchyDataModel>, _info: &mut CallbackInfo<HierarchyDataModel>) -> UpdateScreen{
    state.data.modify(|data|{
        let h_layer:Vec<String> = vec!["-1-".to_string(),"-2-".to_string(),"-3-".to_string()];
        data.labels.push(h_layer);
    });
    Redraw
}

fn del_hierarchy_layer(state: &mut AppState<HierarchyDataModel>, _info: &mut CallbackInfo<HierarchyDataModel>) -> UpdateScreen{
    //let node_id = _info.hit_dom_node;
    let current_row = _info.target_index_in_parent().unwrap();
    println!("Current row: {}", &current_row.to_string());  

    state.data.modify(|data|{
        data.labels.remove(current_row);
    });
    Redraw
}

//Запускает цикл отрисовки GUI и обработки ввода пользователя
pub fn run() {
    let mut app = azul::prelude::App::new(HierarchyDataModel::default(), azul::prelude::AppConfig::default()).unwrap();
    macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/src/hot_reload.css")) }
    
    #[cfg(debug_assertions)]
    let window = { 
        let style = css::hot_reload(CSS_PATH!(), Duration::from_millis(200));
        app.create_hot_reload_window(azul::prelude::WindowCreateOptions::default(), style).unwrap() 
    };

    #[cfg(not(debug_assertions))]
    let window = { 
        let style = css::from_str(include_str!(CSS_PATH!())).unwrap();
        app.create_window(azul::prelude::WindowCreateOptions::default(), style).unwrap() 
    };

    app.run(window).unwrap();
}
