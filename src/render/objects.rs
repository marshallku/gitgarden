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
            Objects::TreeOne | Objects::TreeTwo => (35, 60),
            _ => (16, 16),
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
        .into_iter()
    }

    pub fn get_mask_path(&self) -> Option<&'static str> {
        match self {
            Objects::FlowerThree => Some("M0.4375 0.25h0.0625V0.1875h0.0625V0.125h0.25v0.125h0.125v0.1875h-0.0625v0.0625h-0.0625v0.0625h-0.1875V0.5H0.5625V0.4375H0.5V0.375H0.4375V0.25z"),
            Objects::FlowerFour => Some(
                "M0.0625 0.3125h0.06375v-0.0625H0.5v-0.061875L0.5625 0.1875v-0.0625h0.25v0.061875\
                L0.875 0.1875v0.0625h0.0625v0.0625h0.0625v0.438125L0.9375 0.75v0.0625h-0.3125v0.0625\
                h-0.0625v0.0625h-0.121875L0.4375 0.875h-0.0625v-0.0625h-0.25v-0.0625h-0.0625v-0.4375z"
            ),
            _ => None,
        }
    }

    pub fn get_mask_id(&self) -> Option<String> {
        self.get_mask_path()
            .map(|_| format!("mask-{}", self.to_string()))
    }
}
