use slint::{ComponentFactory, ComponentHandle};

slint::include_modules!();

fn create_components(colors: &[&str]) -> Vec<ComponentFactory> {
    colors
        .iter()
        .enumerate()
        .map(|(index, color)| {
            let color = color.to_string();

            ComponentFactory::new(move |window| {
                let mut compiler = slint_interpreter::ComponentCompiler::new();

                let e = spin_on::spin_on(compiler.build_from_source(
                    format!(
                        r#"
    import {{ Button }} from "std-widgets.slint";

    export component E{index} inherits Rectangle {{
        background: {color};
        width: 100px + 10px * {index};
        height: 60px + 10px * {index};
 
        Button {{
            text: "{color}";
            width: 80px + 10px * {index};
            height: 40px + 10px * {index};
            clicked => {{debug("Button of component with index {index} clicked!");}}
        }}
    }}"#
                    ),
                    std::path::PathBuf::from("generated.slint"),
                ))
                .unwrap();
                e.embed_into_existing_window(window).ok()
            })
        }).chain(std::iter::once(ComponentFactory::default()))
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
        ui.set_switch_count(ui.get_switch_count().wrapping_add(1));
        eprintln!("XXX Switching to index {index}");
        ui.set_e1(components.get(index as usize).unwrap().clone());
    });

    ui.set_current_component(-1);

    ui.run()
}
