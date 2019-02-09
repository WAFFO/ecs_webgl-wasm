use engine::mesh_manager::MeshManager;
use engine::mesh_manager::mesh::*;


#[test]
fn test_storage() {
    let mut manager = MeshManager::new();

    manager.load(String::from("debug_box"));

    println!("{:?}", manager.get_storage());

    manager.load(String::from("debug_box"));

    println!("{:?}", manager.get_storage());

    manager.load(String::from("debug_d20"));

    println!("{:?}", manager.get_storage());
    println!("{:?}", manager.get(String::from("debug_box")));
    println!("{:?}", manager.get(String::from("debug_d20")));

}

#[test]
fn test_mesh_exploder() {
    let mesh_indexed = MeshIndexed {
        vertices: vec![
            0.0, 1.0, 2.0,
            3.0, 4.0, 5.0,
            6.0, 7.0, 8.0,
        ],
        colors: vec![
            0.5, 1.5, 2.5, 1.0,
        ],
        indices: vec![
            2, 1, 0,
        ],
    };

    let mesh = Mesh {
        vertices: vec![
            6.0, 7.0, 8.0,
            3.0, 4.0, 5.0,
            0.0, 1.0, 2.0,
        ],
        colors: vec![
            0.5, 1.5, 2.5, 1.0,
            0.5, 1.5, 2.5, 1.0,
            0.5, 1.5, 2.5, 1.0,
        ],
    };

    assert_eq!(mesh_indexed.explode(), mesh);

    let mesh_indexed = MeshIndexed {
        vertices: vec![
            0.0, 1.0, 2.0,
            3.0, 4.0, 5.0,
            6.0, 7.0, 8.0,
        ],
        colors: vec![
            0.1, 1.1, 2.1, 1.1,
            0.2, 1.2, 2.2, 1.2,
        ],
        indices: vec![
            2, 1, 0,
            0, 1, 2,
        ],
    };

    let mesh = Mesh {
        vertices: vec![
            6.0, 7.0, 8.0,
            3.0, 4.0, 5.0,
            0.0, 1.0, 2.0,
            0.0, 1.0, 2.0,
            3.0, 4.0, 5.0,
            6.0, 7.0, 8.0,
        ],
        colors: vec![
            0.1, 1.1, 2.1, 1.1,
            0.1, 1.1, 2.1, 1.1,
            0.1, 1.1, 2.1, 1.1,
            0.2, 1.2, 2.2, 1.2,
            0.2, 1.2, 2.2, 1.2,
            0.2, 1.2, 2.2, 1.2,
        ],
    };

    assert_eq!(mesh_indexed.explode(), mesh);
}