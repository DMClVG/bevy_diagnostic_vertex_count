[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_diagnostic_vertex_count)](https://crates.io/crates/bevy_diagnostic_vertex_count/)

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
