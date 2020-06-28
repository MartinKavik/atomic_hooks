/// Exports the following:
///
/// use_state associates a type T's state with a current topoolical context
///     returning a copy of that state as well as an accsssor method to update
///     that state
///
/// topo - re-export of topo crate. Needed to ensure a single version of topo
/// is used throughout so that user topo::Ids match comp_state topo::Ids.
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
pub use atom_macros::{atom, reaction,set_reaction};
pub use crate::atom_store::{Getter,Reaction};
pub use crate::atom_state_access::{ChangedAtomState, CloneAtomState, IsAnAtomState,IsAReactionState, AllowUndo, NoUndo, OverloadedUpdateStateAccess,AtomStateAccess};
pub use crate::atom_state_functions::{ATOM_STORE,atom,UndoVec,reaction,observe,observe_with,
    clone_atom_state_with_id, 
    set_inert_atom_state_with_id,
    atom_state_exists_for_id, update_atom_state_with_id, atom_with_undo,remove_atom_state_with_id,unlink_dead_links
};


// pub use crate::atom_state::{OverloadedGeneralLookUp,AtomStore, atom, reaction};

