use enum_map::{enum_map, Enum, EnumMap};
use enum_iterator::IntoEnumIterator;

pub fn init_metatypes() -> EnumMap<MetaTypes, EnumMap<Attributes, AttributeRanges>> {
  let metatype_data = enum_map! {
    MetaTypes::Human => enum_map! {
      Attributes::Body => AttributeRanges {
        min: 1, 
        max: 6,
      }
    }
  };

  return metatype_data;
}

struct Character {
  attribute_points_to_add: u32,
  attribute_points_added: EnumMap<Attributes, u32>,
  attribute_ranges: EnumMap<Attributes, AttributeRanges>
}

impl Character {
  fn can_increase_attribute(&mut self, attribute: Attributes) {
  }

  fn is_attribute_at_max(&mut self) -> bool {

    let attributes = Attributes::into_enum_iter();
    let mut is_attribute_at_max = false;
    for attribute in attributes {
      let AttributeRanges {max, min} = self.attribute_ranges[attribute];
      let added = self.attribute_points_added[attribute];

      if min + added == max {
        is_attribute_at_max = true;
        break;
      }

    }

    return is_attribute_at_max;
  }

  fn increase_attribute(&mut self, attribute: Attributes) {
      if !self.is_attribute_at_max() {
        self.attribute_points_added[attribute] += 1;
      }
  }  
}

#[derive(Enum, IntoEnumIterator, Copy, Clone)]
pub enum Attributes {
  Body,
}

pub struct AttributeRanges {
  min: u32,
  max: u32,
}

#[derive(Enum)]
pub enum MetaTypes {
  Human,
}