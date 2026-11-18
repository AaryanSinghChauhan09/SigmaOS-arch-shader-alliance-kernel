use sigmaos_arch_shader_alliance_kernel::{ShaderManager, ShaderSource, ShaderType};

#[test]
fn crate_links_and_runs() {
    let mut mgr = ShaderManager::new();
    let source = ShaderSource::new("alliance-vertex", ShaderType::Vertex, "void main() { gl_Position = vec4(0.0); }");
    let res = mgr.compile(source);
    assert!(res.is_ok());
    assert_eq!(mgr.list_shaders().len(), 1);
}
