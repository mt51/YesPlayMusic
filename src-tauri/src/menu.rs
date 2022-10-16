use tauri::{ utils::assets::EmbeddedAssets, Context, Menu, CustomMenuItem, MenuItem, Submenu, AboutMetadata, WindowMenuEvent, };


#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

use tauri::api::dialog::message;

pub fn init(context: &Context<EmbeddedAssets>) -> Menu {
  // 获取应用名称
  let name = &context.package_info().name;

  let app_menu = Submenu::new("", 
    Menu::new().add_native_item(
      MenuItem::About(name.into(), AboutMetadata::new())
    ).add_native_item(MenuItem::Quit)
  );

  let controls_menu = Submenu::new(
    "Controls",
    Menu::new()
      .add_item(CustomMenuItem::new("play".to_string(), "Play"))
      .add_item(CustomMenuItem::new("next".to_string(), "Next"))
      .add_item(CustomMenuItem::new("Previous".to_string(), "Previous"))
      .add_item(CustomMenuItem::new("increaseVolume".to_string(), "increase Volume"))
      .add_item(CustomMenuItem::new("decreaseVolume".to_string(), "Decrease Volume"))
      .add_item(CustomMenuItem::new("like".to_string(), "Like"))
      .add_item(CustomMenuItem::new("repeat".to_string(), "Repeat").accelerator("Alt+R"))
      .add_item(CustomMenuItem::new("shuffle".to_string(), "Shuffle").accelerator("Alt+s")),

  );

  let window_menu = Submenu::new(
    "Window",
    Menu::new()
    .add_native_item(MenuItem::Minimize)
    .add_native_item(MenuItem::CloseWindow)
    .add_native_item(MenuItem::EnterFullScreen)
    .add_native_item(MenuItem::Zoom)
  );

  Menu::new()
  .add_submenu(app_menu)
  .add_submenu(controls_menu)
  .add_submenu(window_menu)
}

pub fn handler(event: WindowMenuEvent) {
  let win = Some(event.window());

  match event.menu_item_id() {
    "play" => {
      dbg!("play");
      event.window().emit("play", Payload { message: "play".into() });
    }
    "next" => {
      dbg!("next");
    }
    "previous" => {
      dbg!("previous");
    }
    "increaseVolume" => {
      dbg!("increaseVolume");
    }
    "decreaseVolume" => {
      dbg!("decreaseVolume");
    }
    "like" => {
      dbg!("like");
    }
    "repeat" => {
      dbg!("repeat");
    }
    "shuffle" => {
      dbg!("shuffle");
    }
    _ => {}
  }
}
