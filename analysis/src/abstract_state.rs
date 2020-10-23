// © 2020, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rustc_middle::mir;
use std::vec::Vec;
use crate::AnalysisError;


pub trait AbstractState: Sized {   // Sized needed for apply_terminator_effect's return type
    //fn make_top(&mut self) -> Self;
    //fn make_bottom(&mut self) -> Self;
    fn new_bottom() -> Self;
    //fn new_top() -> Self;
    fn new_initial(args: Vec<&mir::LocalDecl>) -> Self;

    fn need_to_widen(counter: &u32) -> bool;

    //fn is_top(&self) -> bool;
    //fn is_bottom(&self) -> bool;
    //fn less_equal(&self, other: &Self) -> bool;
    fn join(&mut self, other: &Self);
    //fn meet(&mut self, other: &Self) -> Self;
    fn widen(&mut self, previous: &Self);

    fn apply_statement_effect(&mut self, location: &mir::Location, stmt: &mir::Statement)
        -> Result<(), AnalysisError>;
    fn apply_terminator_effect(&self, location: &mir::Location, terminator: &mir::terminator::Terminator)
        -> Result<Vec<(mir::BasicBlock, Self)>, AnalysisError>;
}