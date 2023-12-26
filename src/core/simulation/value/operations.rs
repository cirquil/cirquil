use crate::core::simulation::value::BitState;
use crate::core::simulation::value::BitState::{E, F, T, X};

pub fn not(a: BitState) -> BitState {
    match a {
        F => { T }
        T => { F }
        X => { E }
        E => { E }
    }
}

pub fn assign(a: BitState, b: BitState) -> BitState {
    match (a, b) {
        (F, F) => { F }
        (F, T) => { E }
        (F, X) => { F }
        (F, E) => { E }
        (T, T) => { T }
        (T, X) => { T }
        (T, E) => { E }
        (X, X) => { X }
        (X, E) => { E }
        (E, E) => { E }

        (l, r) => { assign(r, l) }
    }
}

pub fn and(a: BitState, b: BitState) -> BitState {
    match (a, b) {
        (F, F) => { F }
        (F, T) => { F }
        (F, X) => { F }
        (F, E) => { F }
        (T, T) => { T }
        (T, X) => { E }
        (T, E) => { E }
        (X, X) => { E }
        (X, E) => { E }
        (E, E) => { E }

        (l, r) => { and(r, l) }
    }

    // match (a, b) {
    //     (E, _) => { E }
    //     (_, E) => { E }
    //     (F, _) => { F }
    //     (_, F) => { F }
    //     (T, T) => { T }
    //     (X, _) => { X }
    //     (_, X) => { X }
    // }
}

pub fn or(a: BitState, b: BitState) -> BitState {
    match (a, b) {
        (F, F) => { F }
        (F, T) => { T }
        (F, X) => { E }
        (F, E) => { E }
        (T, T) => { T }
        (T, X) => { T }
        (T, E) => { T }
        (X, X) => { E }
        (X, E) => { E }
        (E, E) => { E }

        (l, r) => { or(r, l) }
    }
}