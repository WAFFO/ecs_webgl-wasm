use render_mod::Renderer;

use glm;
use glm::Vec3;



#[test]
fn test_matrix() {
    let mut vec: Vec<f32> = Vec::new();

    let projection =
//            glm::proj(&matrix, &Vec3::new(600.0,600.0,400.0));
//        glm::mat4(
//            2.0/600.0, 0.0, 0.0, 0.0,
//            0.0, -2.0/600.0, 0.0, 0.0,
//            0.0, 0.0, 2.0 / 400.0, 0.0,
//            -1.0, 1.0, 0.0, 1.0,
//        );
        glm::perspective(1.0, 15.0, 0.0, 400.0);
    println!("Projection: {:?}", projection);
    let view_translate  =
        glm::translate(&glm::Mat4::identity(), &Vec3::new(0.0, 0.0, 0.0));
    println!("view_translate: {:?}", view_translate);
    let rotate_x =
        glm::rotate_x(&view_translate, 1.0);
    println!("ROTATE X: {:?}", rotate_x);
    let scale =
        glm::scale(&glm::Mat4::identity(), &Vec3::new(1.0, 1.0, 1.0));
    println!("SCALE: {:?}", scale);

    let matrix = projection * rotate_x * scale;
    println!("MATRIX: {:?}", matrix);

    for x in matrix.iter() {
        vec.push(*x);
    }
    println!("VEC: {:?}", vec);

}