use enum_map::{enum_map, Enum, EnumMap};

pub fn init_metatypes() -> EnumMap<MetaTypes, AttributeRanges> {
  let metatype_data = enum_map! {
    MetaTypes::Human => AttributeRanges {
      body_min: 1, 
      body_max: 6,
    } 
  };

  return metatype_data;
}

struct Character {
  attribute_points_to_add: u32,
  attribute_points_added: EnumMap<Attributes, u32>,
  attribute_ranges: AttributeRanges,
}

impl Character {
  fn increase_attribute(&mut self, attribute: Attributes) {
      self.attribute_points_added[attribute] += 1
  }  
}

#[derive(Enum)]
enum Attributes {
  Body,
}

pub struct AttributeRanges {
  body_min: u32,
  body_max: u32,
}

#[derive(Enum)]
pub enum MetaTypes {
  Human,
}