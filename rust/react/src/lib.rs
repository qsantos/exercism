use std::collections::{HashMap, HashSet};

// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
    dependants: Vec<ComputeCellId>,
}

struct ComputeCell<'a, T> {
    value: T,
    dependencies: Vec<CellId>,
    compute_func: fn(&[T]) -> T,
    dependants: Vec<ComputeCellId>,
    callback_mext_id: usize,
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T) + 'a>>,
}

pub struct Reactor<'a, T> {
    input_cells: Vec<InputCell<T>>,
    compute_cells: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(self.input_cells.len());
        self.input_cells.push(InputCell {
            value: initial,
            dependants: Vec::new(),
        });
        id
    }

    fn gather_dependencies(&self, dependencies: &[CellId]) -> Vec<T> {
        dependencies
            .iter()
            .map(|&id| match id {
                CellId::Input(InputCellId(id)) => self.input_cells[id].value,
                CellId::Compute(ComputeCellId(id)) => self.compute_cells[id].value,
            })
            .collect()
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute(
        &mut self,
        dependencies: &[CellId],
        compute_func: fn(&[T]) -> T,
    ) -> Result<ComputeCellId, CellId> {
        // check dependencies
        for &cell_id in dependencies {
            match cell_id {
                CellId::Input(InputCellId(id)) => {
                    if self.input_cells.get(id).is_none() {
                        return Err(cell_id);
                    }
                }
                CellId::Compute(ComputeCellId(id)) => {
                    if self.compute_cells.get(id).is_none() {
                        return Err(cell_id);
                    }
                }
            }
        }
        // register dependencies
        let new_cell_id = ComputeCellId(self.compute_cells.len());
        for &cell_id in dependencies {
            match cell_id {
                CellId::Input(InputCellId(id)) => self.input_cells[id].dependants.push(new_cell_id),

                CellId::Compute(ComputeCellId(id)) => {
                    self.compute_cells[id].dependants.push(new_cell_id)
                }
            }
        }
        // create cell
        let inputs = self.gather_dependencies(dependencies);
        self.compute_cells.push(ComputeCell {
            value: compute_func(&inputs),
            dependencies: dependencies.to_vec(),
            compute_func,
            dependants: Vec::new(),
            callback_mext_id: 0,
            callbacks: HashMap::new(),
        });
        Ok(new_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, cell_id: CellId) -> Option<T> {
        match cell_id {
            CellId::Input(InputCellId(id)) => self.input_cells.get(id).map(|cell| cell.value),
            CellId::Compute(ComputeCellId(id)) => self.compute_cells.get(id).map(|cell| cell.value),
        }
    }

    fn traverse_postorder(
        &self,
        postorder: &mut Vec<ComputeCellId>,
        visited: &mut HashSet<ComputeCellId>,
        cur: ComputeCellId,
    ) {
        let cell = &self.compute_cells[cur.0];
        for &dependant in &cell.dependants {
            self.traverse_postorder(postorder, visited, dependant);
        }
        if !visited.contains(&cur) {
            postorder.push(cur);
            visited.insert(cur);
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(cell) = self.input_cells.get_mut(id.0) {
            if new_value == cell.value {
                return true;
            }
            cell.value = new_value;
            let cell = &self.input_cells[id.0];

            // traverse the compute cells in reverse post-order of dependency
            let mut postorder = Vec::new();
            let mut visited = HashSet::new();
            for &dependant in &cell.dependants {
                self.traverse_postorder(&mut postorder, &mut visited, dependant);
            }
            postorder.reverse();
            for id in postorder {
                let cell = &self.compute_cells[id.0];
                let dependencies = self.gather_dependencies(&cell.dependencies);
                let cell = &mut self.compute_cells[id.0];
                let new_value = (cell.compute_func)(&dependencies);
                if new_value != cell.value {
                    cell.value = new_value;
                    for callback in cell.callbacks.values_mut() {
                        callback(new_value);
                    }
                }
            }

            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<'b: 'a, F: 'b + FnMut(T)>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if let Some(cell) = self.compute_cells.get_mut(id.0) {
            let id = CallbackId(cell.callback_mext_id);
            cell.callback_mext_id += 1;
            cell.callbacks.insert(id, Box::new(callback));
            Some(id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(cell) = self.compute_cells.get_mut(cell.0) {
            if cell.callbacks.remove_entry(&callback).is_some() {
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}

impl<'a, T: Copy + PartialEq> Default for Reactor<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
