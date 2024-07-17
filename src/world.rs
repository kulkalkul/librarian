// #[derive(Copy, Clone, PartialEq, Eq, Hash)]
// pub struct Entity {
//     id: u64,
// }

// #[derive(Copy, Clone, PartialEq, Eq, Hash)]
// pub struct Component {
//     id: u64,
// }

// impl Component {
//     const RELATION_BITS: u32 = 32;
//     const RELATION_MASK: u64 = const { 2u64.pow(Self::RELATION_BITS) - 1 };

//     fn new(id: u64, relation: u64) -> Self {
//         let id = id << Self::RELATION_BITS | relation;
//         Self { id }
//     }

//     pub fn id(&self) -> u64 {
//         self.id >> Self::RELATION_BITS
//     }
//     pub fn relation(&self) -> u64 {
//         self.id & Self::RELATION_MASK
//     }
// }

// struct UniqueComponent {
//     relations: u32,
// }

// struct Archetype {
//     id: usize,
// }

// pub struct World {
//     components: Vec<Component>,
//     /// Amount of relations unique to per component
//     unique_components: Vec<UniqueComponent>,
//     archetypes: Vec<Archetype>,
// }

// impl World {
//     pub fn new_component(&mut self) -> Component {
//         let id = self.components.len();
//         // 4 billion component&relation pairs are unlikely
//         assert!(id < const { u32::MAX as usize });
//         self.unique_components
//             .push(UniqueComponent { relations: 0 });

//         let component = Component::new(id as u64, 0);
//         self.components.push(component);

//         component
//     }
//     pub fn new_relation(&mut self, component: &Component) -> Component {
//         let id = component.id();

//         // If this doesn't exist, we have bigger problems. It can panic
//         let unique = &mut self.unique_components[id as usize]; // as usize is okay because ID_BITS are 32bit
//         unique.relations += 1;

//         let empty_relation = unique.relations;
//         let component = Component::new(id, empty_relation as u64);
//         self.components.push(component);

//         component
//     }
//     pub fn create_entity<const N: usize>(&mut self, []) {

//     }
// }

// impl Default for World {
//     fn default() -> Self {
//         Self {
//             components: Vec::with_capacity(256),
//             unique_components: Vec::with_capacity(32),
//             archetypes: Vec::with_capacity(64),
//         }
//     }
// }
