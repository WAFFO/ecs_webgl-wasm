use engine::mesh_manager::MeshManager;


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