// This file was generated by gir (becf3b4) from gir-files (11e0e6d)
// DO NOT EDIT

use TreeIter;
use TreeModel;
use TreeSortable;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TreeStore(Object<ffi::GtkTreeStore>): TreeModel, TreeSortable;

    match fn {
        get_type => || ffi::gtk_tree_store_get_type(),
    }
}

impl TreeStore {
    //pub fn new(n_columns: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeStore {
    //    unsafe { TODO: call ffi::gtk_tree_store_new() }
    //}

    //pub fn newv(n_columns: i32, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) -> TreeStore {
    //    unsafe { TODO: call ffi::gtk_tree_store_newv() }
    //}

    pub fn append(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_append(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0));
            iter
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gtk_tree_store_clear(self.to_glib_none().0);
        }
    }

    pub fn insert(&self, parent: Option<&TreeIter>, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), position);
            iter
        }
    }

    pub fn insert_after(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_after(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    pub fn insert_before(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_before(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    //pub fn insert_with_values(&self, parent: Option<&mut TreeIter>, position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_tree_store_insert_with_values() }
    //}

    //pub fn insert_with_valuesv(&self, parent: Option<&mut TreeIter>, position: i32, columns: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, values: &[&glib::Value], n_values: i32) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_tree_store_insert_with_valuesv() }
    //}

    pub fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_is_ancestor(self.to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(descendant.to_glib_none().0)))
        }
    }

    pub fn iter_depth(&self, iter: &TreeIter) -> i32 {
        unsafe {
            ffi::gtk_tree_store_iter_depth(self.to_glib_none().0, mut_override(iter.to_glib_none().0))
        }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_iter_is_valid(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    pub fn move_after(&self, iter: &mut TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_after(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(position.to_glib_none().0));
        }
    }

    pub fn move_before(&self, iter: &mut TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_before(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(position.to_glib_none().0));
        }
    }

    pub fn prepend(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_prepend(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0));
            iter
        }
    }

    pub fn remove(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_remove(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    //pub fn reorder(&self, parent: Option<&mut TreeIter>, new_order: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }) {
    //    unsafe { TODO: call ffi::gtk_tree_store_reorder() }
    //}

    //pub fn set(&self, iter: &mut TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set() }
    //}

    //pub fn set_column_types(&self, n_columns: i32, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_column_types() }
    //}

    //pub fn set_valist(&self, iter: &mut TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_valist() }
    //}

    //pub fn set_valuesv(&self, iter: &mut TreeIter, columns: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, values: &[&glib::Value], n_values: i32) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_valuesv() }
    //}

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_tree_store_swap(self.to_glib_none().0, mut_override(a.to_glib_none().0), mut_override(b.to_glib_none().0));
        }
    }
}
