use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/script.js")]
extern "C" {
    pub type Counter;

    #[wasm_bindgen(constructor)]
    fn new() -> Counter;

    #[wasm_bindgen(method, getter)]
    fn number(this: &Counter) -> u32;

    #[wasm_bindgen(method, setter)]
    fn set_number(this: &Counter, number: u32) -> Counter;

    #[wasm_bindgen(method)]
    fn render(this: &Counter) -> String;
}

pub struct CounterState {
    pub counter: JsValue,
}

struct CounterText;

#[wasm_bindgen]
pub fn run_game(counter: Counter) {
    let mut app = bevy::prelude::App::build();

    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(setup.system())
        .insert_non_send_resource(CounterState {
            counter: counter.into(),
        })
        .add_system(change_text.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/The-Humble.ttf");

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Percent(45.0),
                    left: Val::Percent(45.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "".to_string(),
                TextStyle {
                    font: font.clone(),
                    color: Color::BLACK,
                    font_size: 72.0,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(CounterText);
}

fn change_text(mut q: Query<&mut Text, With<CounterText>>, state: NonSend<CounterState>) {
    for mut text in q.single_mut() {
        let counter: &Counter = state.counter.unchecked_ref();
        text.sections[0].value = counter.number().to_string();
    }
}