#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Objects {
    FlowerOne,
    FlowerTwo,
    FlowerThree,
    FlowerFour,
    TreeOne,
    TreeTwo,
    GrassOne,
    GrassTwo,
    GrassThree,
    GrassFour,
    GrassFive,
    GrassSix,
    Dirt,
}

impl Objects {
    pub fn to_string(&self) -> String {
        match self {
            Objects::FlowerOne => "flower-1".to_string(),
            Objects::FlowerTwo => "flower-2".to_string(),
            Objects::FlowerThree => "flower-3".to_string(),
            Objects::FlowerFour => "flower-4".to_string(),
            Objects::TreeOne => "tree-1".to_string(),
            Objects::TreeTwo => "tree-2".to_string(),
            Objects::GrassOne => "grass-1".to_string(),
            Objects::GrassTwo => "grass-2".to_string(),
            Objects::GrassThree => "grass-3".to_string(),
            Objects::GrassFour => "grass-4".to_string(),
            Objects::GrassFive => "grass-5".to_string(),
            Objects::GrassSix => "grass-6".to_string(),
            Objects::Dirt => "dirt".to_string(),
        }
    }

    pub fn to_path(&self) -> String {
        match self {
            Objects::FlowerOne => "flowers/1-1.png".to_string(),
            Objects::FlowerTwo => "flowers/1-2.png".to_string(),
            Objects::FlowerThree => "flowers/1-3.png".to_string(),
            Objects::FlowerFour => "flowers/1-4.png".to_string(),
            Objects::TreeOne => "objects/tree1.png".to_string(),
            Objects::TreeTwo => "objects/tree2.png".to_string(),
            Objects::GrassOne => "field/grass1.png".to_string(),
            Objects::GrassTwo => "field/grass2.png".to_string(),
            Objects::GrassThree => "field/grass3.png".to_string(),
            Objects::GrassFour => "field/grass4.png".to_string(),
            Objects::GrassFive => "field/grass5.png".to_string(),
            Objects::GrassSix => "field/grass6.png".to_string(),
            Objects::Dirt => "field/dirt2.png".to_string(),
        }
    }

    pub fn to_size(&self) -> (u32, u32) {
        match self {
            Objects::FlowerOne => (16, 16),
            Objects::FlowerTwo => (16, 16),
            Objects::FlowerThree => (16, 16),
            Objects::FlowerFour => (16, 16),
            Objects::TreeOne => (35, 60),
            Objects::TreeTwo => (35, 60),
            Objects::GrassOne => (16, 16),
            Objects::GrassTwo => (16, 16),
            Objects::GrassThree => (16, 16),
            Objects::GrassFour => (16, 16),
            Objects::GrassFive => (16, 16),
            Objects::GrassSix => (16, 16),
            Objects::Dirt => (16, 16),
        }
    }

    pub fn iter() -> impl Iterator<Item = Objects> {
        [
            Objects::FlowerOne,
            Objects::FlowerTwo,
            Objects::FlowerThree,
            Objects::FlowerFour,
            Objects::TreeOne,
            Objects::TreeTwo,
            Objects::GrassOne,
            Objects::GrassTwo,
            Objects::GrassThree,
            Objects::GrassFour,
            Objects::GrassFive,
            Objects::GrassSix,
            Objects::Dirt,
        ]
        .iter()
        .copied()
    }
}
