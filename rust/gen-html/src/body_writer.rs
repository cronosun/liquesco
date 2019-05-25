use crate::reference::Reference;
use liquesco_schema::core::TypeRef;
use minidom::Element;
use crate::usage::Usage;
use liquesco_processing::schema::SchemaReader;
use liquesco_processing::names::Names;
use liquesco_schema::core::Type;

pub trait BodyWriter {
    type T : Type + Sized;
    fn write(ctx : &mut Context<Self::T>) -> Element;
}

pub struct Context<'a, T> {
    pub schema: &'a SchemaReader,
    pub r#type : &'a T,
    pub type_ref : TypeRef,
    pub names : &'a mut Names,
    pub usage: &'a mut Usage,
}

impl<'a, T> Context<'a, T> {
    pub fn set_uses(&mut self, uses_what : TypeRef) {
        self.usage.set_uses(self.type_ref, uses_what);
    }

    pub fn link(&mut self, target : TypeRef) -> Element {
      let type_info = self.schema.type_info(target);
      Reference {
          type_info: &type_info,
          names : self.names,
      }.link()
    }
}