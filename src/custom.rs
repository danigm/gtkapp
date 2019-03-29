use gtk;

use gtk::prelude::*;

use glib;
use glib::{glib_object_subclass, glib_object_impl};
use glib::prelude::*;
use glib::subclass;
use glib::subclass::prelude::*;

use std::cell::RefCell;

// Static array for defining the properties of the new type.
static PROPERTIES: [subclass::Property; 1] = [subclass::Property("name", |name| {
    glib::ParamSpec::string(
        name,
        "Name",
        "Name of this object",
        None,
        glib::ParamFlags::READWRITE,
    )
})];

// This is the struct containing all state carried with
// the new type. Generally this has to make use of
// interior mutability.
pub struct CustomButton {
    name: RefCell<Option<String>>,
}

// ObjectSubclass is the trait that defines the new type and
// contains all information needed by the GObject type system,
// including the new type's name, parent type, etc.
impl ObjectSubclass for CustomButton {
    // This type name must be unique per process.
    const NAME: &'static str = "CustomButton";

    // The parent type this one is inheriting from.
    type ParentType = glib::Object;

    // The C/FFI instance and class structs. The simple ones
    // are enough in most cases and more is only needed to
    // expose public instance fields to C APIs or to provide
    // new virtual methods for subclasses of this type.
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    // This macro defines some boilerplate.
    glib_object_subclass!();

    // Called right before the first time an instance of the new
    // type is created. Here class specific settings can be performed,
    // including installation of properties and registration of signals
    // for the new type.
    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        klass.install_properties(&PROPERTIES);
    }

    // Called every time a new instance is created. This should return
    // a new instance of our type with its basic values.
    fn new() -> Self {
        Self {
            name: RefCell::new(None),
        }
    }
}

// Trait that is used to override virtual methods of glib::Object.
impl ObjectImpl for CustomButton {
    // This macro defines some boilerplate.
    glib_object_impl!();

    // Called whenever a property is set on this instance. The id
    // is the same as the index of the property in the PROPERTIES array.
    fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
        let prop = &PROPERTIES[id];

        match *prop {
            subclass::Property("name", ..) => {
                let name = value.get();
                self.name.replace(name);
            }
            _ => unimplemented!(),
        }
    }

    // Called whenever a property is retrieved from this instance. The id
    // is the same as the index of the property in the PROPERTIES array.
    fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id];

        match *prop {
            subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
            _ => unimplemented!(),
        }
    }

    // Called right after construction of the instance.
    fn constructed(&self, obj: &glib::Object) {
        // Chain up to the parent type's implementation of this virtual
        // method.
        self.parent_constructed(obj);

        // And here we could do our own initialization.
    }
}


impl CustomButton {
    pub fn new() -> glib::Object {
        glib::Object::new(CustomButton::get_type(), &[]).unwrap()
    }
}

pub fn test() {
    // Create an object instance of the new type.
    let obj = glib::Object::new(CustomButton::get_type(), &[]).unwrap();

    // Get the name property and change its value.
    assert_eq!(obj.get_property("name").unwrap().get::<&str>(), None);
    obj.set_property("name", &"test").unwrap();
    assert_eq!(
        obj.get_property("name").unwrap().get::<&str>(),
        Some("test")
    );
    println!("{:?}", obj.get_property("name").unwrap().get::<&str>());
}
