/// Exports the following:
///
/// use_state associates a type T's state with a current topoolical context
///     returning a copy of that state as well as an accsssor method to update
///     that state
///
/// topo - re-export of topo crate. Needed to ensure a single version of topo
/// is used throughout so that user topo::CallIds match comp_state topo::CallIds.
///
/// do_once - a function to do a block once and once only
///
/// set_state - set the state of type T in the current topological context
///
/// clone_state - clone the state of a type T in the current topological context
///
/// get_state_with_topo_id - clone the state of type T in the given topological context
///
/// set_state_with_topo_id - set the state of type T in the given topological context
///
/// update_state_with_topo_id - update the state of type T in the given topological
///     context
///
/// purge_and_reset_unseed_ids - rudamentary gabrage collection, purgets any
///     topological context state that has not been accessed since the last time
///     this function was run
///
///  StateAccess - the access struct that enables a state to be udated or retrieved
pub use crate::marker::*;
pub use crate::reactive_state_access::{
    Atom, AtomUndo, CloneReactiveState, ObserveChangeReactiveState, Reaction,
};
pub use crate::reactive_state_functions::{
    atom, atom_undo, clone_reactive_state_with_id, reaction, reaction_start_suspended,
    reactive_state_exists_for_id, read_reactive_state_with_id, remove_reactive_state_with_id,
    return_key_for_type_and_insert_if_required, set_inert_atom_state_with_id,
    set_inert_atom_state_with_id_with_undo, try_read_reactive_state_with_id, unlink_dead_links,
    update_atom_state_with_id, UndoVec,
};
pub use crate::store::{ReactiveContext, RxFunc, TopoKey};
pub use crate::undo::{global_undo_queue, GlobalUndo};
pub use atomic_hooks_macros::{atom, reaction};
// pub use crate::local_update_el::{LocalUpdateEl2,Local,};
pub use illicit;
pub use topo;

// Re exports
pub use crate::helpers::{do_once, CallSite, Local};
pub use crate::hooks_state_functions::{
    clone_state_with_topo_id, execute_and_remove_unmounts, new_state, on_unmount,
    reset_unseen_id_list, set_state_with_topo_id, state_exists_for_topo_id, unseen_ids,
    update_state_with_topo_id, use_state, use_state_current,
};
pub use crate::state_access::{ChangedState, CloneState, StateAccess};
pub use crate::unmount::{StateAccessUnmount, Unmount};

pub use crate::observable::Observable;
