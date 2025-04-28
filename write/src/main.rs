use editpe::{
    ResourceEntry, ResourceEntryName, ResourceTable,
    constants::RT_RCDATA,
};
use std::path::PathBuf;

fn main() {
    let bin_path = PathBuf::from("../read/target/release/read.exe");
    let bytes = std::fs::read(&bin_path).unwrap();
    let mut image = editpe::Image::parse(bytes).unwrap();
    let mut resource_dir = image.resource_directory().cloned().unwrap_or_default();
    let mut text = String::new();
    for i in 0..10 {
        text.push_str(&format!("{}\n", i));
    }
    write_resource(&mut resource_dir, "ABCDEFG2", text.clone().into());
    write_resource(&mut resource_dir, "ABCDEFG1", text.clone().into());
    write_resource(&mut resource_dir, "ABCDEFGH", text.into());
    image.set_resource_directory(resource_dir).unwrap();

    std::fs::write("output.exe", image.data()).unwrap();
    eprintln!("Done");
}

fn write_resource(resource_dir: &mut editpe::ResourceDirectory, name: &str, sectdata: Vec<u8>) {
    let root = resource_dir.root_mut();
    if root.get(ResourceEntryName::ID(RT_RCDATA as u32)).is_none() {
        root.insert(
            ResourceEntryName::ID(RT_RCDATA as u32),
            ResourceEntry::Table(ResourceTable::default()),
        );
    }
    let rc_table = match root
        .get_mut(ResourceEntryName::ID(RT_RCDATA as u32))
        .unwrap()
    {
        ResourceEntry::Table(table) => table,
        ResourceEntry::Data(_) => {
            panic!("RCDATA is not a table");
        }
    };
    let name = name.to_uppercase();
    rc_table.insert(
        editpe::ResourceEntryName::from_string(name.clone()),
        ResourceEntry::Table(ResourceTable::default()),
    );

    let rc_table = match rc_table
        .get_mut(editpe::ResourceEntryName::from_string(name))
        .unwrap()
    {
        ResourceEntry::Table(table) => table,
        ResourceEntry::Data(_) => {
            panic!("Resource entry is not a table");
        }
    };
    let mut entry = editpe::ResourceData::default();
    entry.set_data(sectdata);

    rc_table.insert(ResourceEntryName::ID(0), ResourceEntry::Data(entry));
}
