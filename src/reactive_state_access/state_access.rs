use crate::{
    clone_reactive_state_with_id, hooks_state_functions::*, read_reactive_state_with_id,
    store::TopoKey, Observable, ReactiveContext,
};
use std::marker::PhantomData;

///  Accessor struct that provides access to getting and setting the
///  state of the stored type
// #[derive(Debug)]
pub struct StateAccess<T> {
    pub id: TopoKey,
    _phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for StateAccess<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for StateAccess<T> {}
impl<T> Clone for StateAccess<T> {
    fn clone(&self) -> StateAccess<T> {
        StateAccess::<T> {
            id: self.id,
            _phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> StateAccess<T>
where
    T: 'static,
{
    pub fn new(id: TopoKey) -> StateAccess<T> {
        StateAccess {
            id,
            _phantom_data: PhantomData,
        }
    }

    // stores a value of type T in a backing Store
    pub fn set(self, value: T) {
        set_state_with_topo_id(value, self.id);
    }

    pub fn remove(self) -> Option<T> {
        remove_state_with_topo_id(self.id)
    }

    pub fn delete(self) {
        self.remove();
    }

    pub fn reset_on_unmount(self) -> Self {
        on_unmount(move || self.delete());
        self
    }

    /// updates the stored state in place
    /// using the provided function
    pub fn update<F: FnOnce(&mut T) -> ()>(self, func: F) {
        update_state_with_topo_id(self.id, func);
    }

    pub fn state_exists(self) -> bool {
        state_exists_for_topo_id::<T>(self.id)
    }

    pub fn get_with<F: FnOnce(&T) -> R, R>(self, func: F) -> R {
        read_state_with_topo_id(self.id, func)
    }
}

pub trait CloneState<T>
where
    T: Clone + 'static,
{
    fn get(&self) -> T;

    fn soft_get(&self) -> Option<T>;
}

impl<T> CloneState<T> for StateAccess<T>
where
    T: Clone + 'static,
{
    /// returns a clone of the stored state panics if not stored.
    fn get(&self) -> T {
        clone_state_with_topo_id::<T>(self.id).expect("state should be present")
    }

    fn soft_get(&self) -> Option<T> {
        clone_state_with_topo_id::<T>(self.id)
    }
}

#[derive(Clone)]
struct ChangedWrapper<T>(T);

pub trait ChangedState {
    fn changed(&self) -> bool;
}

impl<T> ChangedState for StateAccess<T>
where
    T: Clone + 'static + PartialEq,
{
    fn changed(&self) -> bool {
        if let Some(old_state) = clone_state_with_topo_id::<ChangedWrapper<T>>(self.id) {
            old_state.0 != self.get()
        } else {
            set_state_with_topo_id(ChangedWrapper(self.get()), self.id);
            true
        }
    }
}

impl<T> std::fmt::Display for StateAccess<T>
where
    T: std::fmt::Display + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_with(|t| format!("{}", t)))
    }
}

use crate::reactive_state_functions::STORE;
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};

impl<T> Add for StateAccess<T>
where
    T: Copy + Add<Output = T> + 'static,
{
    type Output = T;

    fn add(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o + *s))
    }
}

impl<T> Mul for StateAccess<T>
where
    T: Copy + Mul<Output = T> + 'static,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o * *s))
    }
}

impl<T> Div for StateAccess<T>
where
    T: Copy + Div<Output = T> + 'static,
{
    type Output = T;

    fn div(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o / *s))
    }
}

impl<T> Sub for StateAccess<T>
where
    T: Copy + Sub<Output = T> + 'static,
{
    type Output = T;

    fn sub(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o - *s))
    }
}

impl<T> Observable<T> for StateAccess<T>
where
    T: 'static,
{
    fn observe(&self) -> T
    where
        T: Clone,
    {
        let id = crate::store::StorageKey::TopoKey(self.id);
        let context = illicit::get::<RefCell<ReactiveContext>>().expect(
            "No #[reaction] context found, are you sure you are in one? I.e. does the current \
             function have a #[reaction] tag?",
        );
        context.borrow_mut().reactive_state_accessors.push(id);

        STORE.with(|store_refcell| {
            store_refcell
                .borrow_mut()
                .add_dependency(&id, &context.borrow().key);
        });

        clone_reactive_state_with_id::<T>(id).unwrap()
    }

    #[topo::nested]
    fn observe_update(&self) -> (Option<T>, T)
    where
        T: 'static + Clone,
    {
        let previous_value_access = crate::hooks_state_functions::use_state(|| None);
        let opt_previous_value = previous_value_access.get();
        let new_value = self.get();
        previous_value_access.set(Some(new_value.clone()));
        (opt_previous_value, new_value)
    }

    // <T: 'static, F: FnOnce(&T) -> R, R>(id: StorageKey, func: F) -> R {
    fn observe_with<F: FnOnce(&T) -> R, R>(&self, func: F) -> R {
        let id = crate::store::StorageKey::TopoKey(self.id);
        if let Ok(context) = illicit::get::<RefCell<ReactiveContext>>() {
            context
                .borrow_mut()
                .reactive_state_accessors
                .push(id.clone());

            STORE.with(|store_refcell| {
                store_refcell
                    .borrow_mut()
                    .add_dependency(&id, &context.borrow().key);
            });
        }
        read_reactive_state_with_id(id, func)
    }
}