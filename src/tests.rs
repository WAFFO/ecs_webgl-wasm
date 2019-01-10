use render_mod::Renderer;

use glm;
use glm::Vec3;



#[test]
fn test_matrix() {
    let mut vec: Vec<f32> = Vec::new();

    let mat = glm::Mat4::identity();
    println!("Mat: {:?}", mat);

    let projection =
        glm::proj(&mat,&Vec3::new(0.5,0.5,0.5));
    println!("Projection: {:?}", projection);

}