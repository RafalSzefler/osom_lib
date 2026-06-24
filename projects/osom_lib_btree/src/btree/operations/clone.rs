use osom_lib_try_clone::TryClone;

use crate::{
    btree::{BTree, BTreeConfig},
    errors::BTreeTryCloneError,
};

impl<TKey, TValue, TConfig> TryClone for BTree<TKey, TValue, TConfig>
where
    TKey: Ord + TryClone,
    TValue: TryClone,
    TConfig: BTreeConfig + TryClone,
{
    type Error = BTreeTryCloneError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let new_config = self.config.try_clone().map_err(|_| BTreeTryCloneError::OtherError)?;
        let mut new_tree = Self::with_config(new_config);

        for kvp in self.iter() {
            let key = kvp.key.try_clone().map_err(|_| BTreeTryCloneError::KeyCloningError)?;
            let value = kvp
                .value
                .try_clone()
                .map_err(|_| BTreeTryCloneError::ValueCloningError)?;
            new_tree
                .try_insert(key, value)
                .map_err(|_| BTreeTryCloneError::OtherError)?;
        }

        Ok(new_tree)
    }
}

impl<TKey, TValue, TConfig> Clone for BTree<TKey, TValue, TConfig>
where
    TKey: Ord + TryClone,
    TValue: TryClone,
    TConfig: BTreeConfig + TryClone,
{
    fn clone(&self) -> Self {
        self.try_clone().expect("[BTree::clone] failure")
    }
}
