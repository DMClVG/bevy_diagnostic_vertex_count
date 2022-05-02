# USE
```rust
use bevy::{
    prelude::*,
    diagnostic::LogDiagnosticsPlugin,
};
use bevy_diagnostic_vertex_count::{VertexCountDiagnosticsPlugin, VertexCountDiagnosticsSettings};

fn main() {
    App::new()
        .insert_resource(VertexCountDiagnosticsSettings { 
            only_visible: true // Set whether only visible meshes should be diagnosed. Defaults to true
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default()) // prints our diagnostics to the console
        .add_plugin(VertexCountDiagnosticsPlugin)
        .run();
}
```

# LICENSE
Dual-licensed under APACHE-2.0 or MIT