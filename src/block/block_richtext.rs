//! A rich text block holds a list of formatted message elements

use crate::block::block_elements::BlockElement;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object containing formatted text
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RichTextBlock {
    pub elements: Vec<BlockElement>,
    pub block_id: Option<String>,
}

impl RichTextBlock {
    pub fn builder(elements: Vec<BlockElement>) -> RichTextBlockBuilder {
        RichTextBlockBuilder::new(elements)
    }
}

#[derive(Debug, Default)]
pub struct RichTextBlockBuilder {
    pub elements: Vec<BlockElement>,
    pub block_id: Option<String>,
}

impl RichTextBlockBuilder {
    pub fn new(elements: Vec<BlockElement>) -> RichTextBlockBuilder {
        RichTextBlockBuilder {
            elements,
            ..Default::default()
        }
    }
    pub fn block_id(mut self, block_id: String) -> RichTextBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn build(self) -> RichTextBlock {
        RichTextBlock {
            elements: self.elements,
            block_id: self.block_id,
        }
    }
}
