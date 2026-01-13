use log::error;

use crate::core::{layer::Layer};

#[derive(Default)]
pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    layer_count: usize,
}

impl LayerStack {

    pub fn new() -> Self {
        Self { layers:Vec::new(), layer_count: 0 }
    }

    /// 添加一个普通层到栈中
    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(self.layer_count, layer);
        self.layer_count += 1;
    }

    /// 弹出指定位置的普通层
    pub fn pop_layer(&mut self, layer_index: usize) -> Option<Box<dyn Layer>> {
        if layer_index >= self.layer_count {
            error!("普通层索引超出范围");
            return None;
        }

        let actual_index = self.layer_count;
        self.layer_count -= 1;
        Some(self.layers.remove(actual_index))
    }

    /// 添加一个覆盖层到栈顶
    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }

    /// 弹出指定位置的覆盖层
    pub fn pop_overlay(&mut self, overlay_index: usize) -> Option<Box<dyn Layer>> {
        if overlay_index < self.layer_count || overlay_index >= self.layers.len() {
            error!("顶层索引超出范围");
            return None;
        }

        Some(self.layers.remove(overlay_index))
    }

    /// 获取所有层的可变引用（用于更新）
    pub fn layers_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Layer>> + DoubleEndedIterator {
        self.layers.iter_mut()
    }

    /// 获取所有层的不可变引用（用于读取）
    pub fn layers(&self) -> impl Iterator<Item = &Box<dyn Layer>> + DoubleEndedIterator {
        self.layers.iter()
    }

        /// 获取普通层数量
    pub fn layer_count(&self) -> usize {
        self.layer_count
    }

    /// 获取覆盖层数量
    pub fn overlay_count(&self) -> usize {
        self.layers.len() - self.layer_count
    }

    /// 获取总层数
    pub fn total_count(&self) -> usize {
        self.layers.len()
    }

    /// 安全地交换两个普通层的位置
    pub fn swap_layers(&mut self, index1: usize, index2: usize) -> Result<(), &'static str> {
        if index1 >= self.layer_count && index2 >= self.layer_count {
            self.layers.swap(index1, index2);
            Ok(())
        } else {
            Err("不能交换普通层和覆盖层")
        }
    }
}