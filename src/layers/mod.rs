mod image;
mod null;
mod pre_comp;
mod shape;
mod solid;
mod text;

pub use self::{image::*, null::*, pre_comp::*, shape::*, solid::*, text::*};

use serde::{de::Deserializer, Deserialize};

#[derive(Debug)]
pub enum Layer {
    PreComp(PreComp),
    Solid(Solid),
    Image(Image),
    Null(Null),
    Shape(Shape),
    Text(Text),
}

impl<'de> Deserialize<'de> for Layer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // lol https://github.com/serde-rs/serde/pull/1392
        use serde::__private::de::{ContentDeserializer, TaggedContentVisitor};
        use serde_repr::Deserialize_repr;

        #[derive(Debug, Deserialize_repr)]
        #[repr(u8)]
        enum LayerTag {
            PreComp = 0,
            Solid = 1,
            Image = 2,
            Null = 3,
            Shape = 4,
            Text = 5,
        }

        let tagged = deserializer.deserialize_any(TaggedContentVisitor::<LayerTag>::new(
            "ty",
            "internally tagged enum Layer",
        ))?;
        match tagged.tag {
            LayerTag::PreComp => {
                log::info!("parsing `PreComp`");
                PreComp::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))
                    .map(Layer::PreComp)
            }
            LayerTag::Solid => {
                log::info!("parsing `Solid`");
                Solid::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))
                    .map(Layer::Solid)
            }
            LayerTag::Image => {
                log::info!("parsing `Image`");
                Image::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))
                    .map(Layer::Image)
            }
            LayerTag::Null => {
                log::info!("parsing `Null`");
                Null::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))
                    .map(Layer::Null)
            }
            LayerTag::Shape => {
                log::info!("parsing `Shape`");
                Shape::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))
                    .map(Layer::Shape)
            }
            LayerTag::Text => {
                log::info!("parsing `Text`");
                Text::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))
                    .map(Layer::Text)
            }
        }
    }
}
