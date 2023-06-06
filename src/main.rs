use std::pin::Pin;

use slint::{Component, ComponentFactoryBox, ComponentHandle};

slint::include_modules!();

fn create_components(colors: &[&str]) -> Vec<ComponentFactoryBox> {
    colors
        .iter()
        .map(|color| {
            let color = color.to_string();

            ComponentFactoryBox::new(Box::new(move || {
                let mut compiler = slint_interpreter::ComponentCompiler::new();

                let e = spin_on::spin_on(compiler.build_from_source(
                    format!(
                        r#"
    import {{ Button }} from "std-widgets.slint";

    component E1 inherits Rectangle {{
        width: 150px;
        height: 50px;
        background: {color};

        Button {{
            text: "{color}";
        }}
    }}"#
                    ),
                    std::path::PathBuf::from("generated.slint"),
                ))
                .unwrap();
                e.create().ok().map(|h| Component::from(h))
            }))
        })
        .collect()
}

fn main() -> Result<(), slint::PlatformError> {
    // Create a dynamic component!

    let ui = AppWindow::new()?;

    let components = create_components(&["Colors.red", "Colors.green", "Colors.blue"]);

    let uiw = ui.as_weak();
    ui.on_clicked(move || {
        let ui = uiw.upgrade().unwrap();
        let index = ui.get_current_component() + 1;
        let index = if index as usize >= components.len() {
            0
        } else {
            index
        };

        ui.set_current_component(index);
        ui.set_e1(components.get(index as usize).unwrap().clone());
    });

    ui.set_current_component(-1);

    ui.run()
}
