use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    prelude::*,
};

#[derive(Default)]
pub struct VertexCountDiagnosticsPlugin;

pub struct VertexCountDiagnosticsSettings {
    pub only_visible: bool,
}

impl Default for VertexCountDiagnosticsSettings {
    fn default() -> Self {
        Self { only_visible: true }
    }
}

impl Plugin for VertexCountDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VertexCountDiagnosticsSettings>()
            .add_startup_system(Self::setup_system)
            .add_system(Self::diagnostic_system);
    }
}

impl VertexCountDiagnosticsPlugin {
    pub const VERTEX_COUNT: DiagnosticId =
        DiagnosticId::from_u128(8139414220128000606581257525911227370);

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::VERTEX_COUNT, "vertex_count", 20));
    }

    pub fn diagnostic_system(
        meshes: Res<Assets<Mesh>>,
        meshed_entities: Query<(&Handle<Mesh>, Option<&Visibility>)>,
        diagnostics: Option<ResMut<Diagnostics>>,
        settings: Res<VertexCountDiagnosticsSettings>,
    ) {
        if let Some(mut diagnostics) = diagnostics {
            let vertex_count: usize = meshed_entities
                .iter()
                .map(|(mesh, visibility)| {
                    if let Some(mesh) = meshes.get(mesh) {
                        if !(settings.only_visible
                            && visibility.is_some()
                            && !visibility.unwrap().is_visible)
                        {
                            mesh.count_vertices()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum();

            diagnostics.add_measurement(Self::VERTEX_COUNT, || vertex_count as f64);
        }
    }
}
