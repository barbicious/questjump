pub mod texture_registry;
pub mod tile_registry;

#[macro_export]
macro_rules! impl_registry_indexers {
    ($value:ident, $registry:ident) => {
        use std::ops::Index;
        
        impl Index<usize> for $registry {
            type Output = $value;
        
            fn index(&self, index: usize) -> &$value {
                &self.values[index]
            }
        }
        
        impl Index<&str> for $registry {
            type Output = usize;
        
            fn index(&self, index: &str) -> &usize {
                &self.names[index]
            }
        }

    };
}