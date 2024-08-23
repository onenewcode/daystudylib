use std::{marker::PhantomData, slice, sync::Arc, vec};

use serde::{de::Visitor, ser::SerializeStruct,Deserializer, Serializer};
#[derive(Debug)]
pub struct Tensor<T> {
    data: Arc<Box<[T]>>,
    shape: Vec<usize>,
    offset: usize,
    length: usize,
}

impl<T: Copy + Clone + Default> Tensor<T> {
    pub fn new(data: Vec<T>, shape: &Vec<usize>) -> Self {
        let length = data.len();
        Tensor {
            data: Arc::new(data.into_boxed_slice().try_into().unwrap()),
            shape: shape.clone(),
            offset: 0,
            length: length,
        }
    }

    pub fn default(shape: &Vec<usize>) -> Self {
        let length = shape.iter().product();
        let data = vec![T::default(); length];
        Self::new(data, shape)
    }

    pub fn data(&self) -> &[T] {
        &self.data[self.offset..][..self.length]
    }

    pub unsafe fn data_mut(&mut self) -> &mut [T] {
        let ptr = self.data.as_ptr().add(self.offset) as *mut T;
        slice::from_raw_parts_mut(ptr, self.length)
    }

    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    pub fn size(&self) -> usize {
        self.length
    }

    // Reinterpret the tensor as a new shape while preserving total size.
    pub fn reshape(&mut self, new_shape: &Vec<usize>) -> &mut Self {
        let new_length: usize = new_shape.iter().product();
        if new_length != self.length {
            let old_shape = self.shape.clone();
            panic!("New shape {new_shape:?} does not match tensor of {old_shape:?}");
        }
        self.shape = new_shape.clone();
        self
    }

    pub fn slice(&self, start: usize, shape: &Vec<usize>) -> Self {
        let new_length: usize = shape.iter().product();
        assert!(self.offset + start + new_length <= self.length);
        Tensor {
            data: self.data.clone(),
            shape: shape.clone(),
            offset: self.offset + start,
            length: new_length,
        }
    }
}

// Some helper functions for testing and debugging
impl Tensor<f32> {
    #[allow(unused)]
    pub fn close_to(&self, other: &Self, rel: f32) -> bool {
        if self.shape() != other.shape() {
            return false;
        }
        let a = self.data();
        let b = other.data();

        return a.iter().zip(b).all(|(x, y)| float_eq(x, y, rel));
    }
    #[allow(unused)]
    pub fn print(&self) {
        println!(
            "shpae: {:?}, offset: {}, length: {}",
            self.shape, self.offset, self.length
        );
        let dim = self.shape()[self.shape().len() - 1];
        let batch = self.length / dim;
        for i in 0..batch {
            let start = i * dim;
            println!("{:?}", &self.data()[start..][..dim]);
        }
    }
}
// 手动实现序列化,
// 强制我们的对象实现需要的特征
impl<T: Clone + serde::ser::Serialize> serde::Serialize for Tensor<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Tensor", 4)?;
        //
        state.serialize_field("data", &self.data.to_vec())?;
        state.serialize_field("shape", &self.shape)?;
        state.serialize_field("offset", &self.offset)?;
        state.serialize_field("length", &self.length)?;
        state.end()
    }
}
// 手动实现反序列化,
impl<'de,T:Deserializer<'de>> serde::Deserialize<'de> for Tensor<T> {
    fn deserialize<D>(deserializer: D) -> Result<Tensor<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// 访问者结构体
        struct TensorVisitor<T>
        {
            // 用于标记泛型，不使用
            marker: PhantomData<T>,
        }

        impl<'de, T:Deserializer<'de>> Visitor<'de> for TensorVisitor<T>
        {
            type Value = Tensor<T>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a generic struct with a field of type T")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>, {
                        let mut data = Vec::<T>::new();
                        let mut shape = vec![];
                        let mut offset = 0;
                        let mut length = 0;
                        while let Some(key) = map.next_key::<&str>()? {
                            match key {
                                "data" => {
                                    data = map.next_value()?;
                                }
                                "shape" => {}
                            }
                        }
                        Ok(Tensor { data: Arc::new(Box::new([])), shape: vec![1] , offset: 0, length: 0 })
            }
        }

        deserializer.deserialize_struct(
            "Tensor",
            &["data", "shape", "offset", "length"],
            TensorVisitor{
                marker: PhantomData,
            },
        )
    }
}

#[inline]
pub fn float_eq(x: &f32, y: &f32, rel: f32) -> bool {
    (x - y).abs() <= rel * (x.abs() + y.abs()) / 2.0
}
mod tests {
    use super::*;

    #[test]
    fn se() {
        let t:Tensor<f32>=Tensor::default(&vec![1]);
        let s=serde_json::to_string(&t).unwrap();
        println!("{:?}",s);
        // let tm=serde_json::from_str::<Tensor<f32>>(&s).unwrap();
        match serde_json::from_str::<Tensor<f32>>(&s) {
            Ok(data) => println!("Deserialized data: {:?}", data),
            Err(e) => {
                println!("Failed to deserialize: {}", e);
            }
        }
        // println!("{:?}",serde_json::to_string(&tm).unwrap());
        
    }
}
fn main() {
    
}