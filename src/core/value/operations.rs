use crate::core::value::BitState;

pub fn not(a: BitState) -> BitState {
    use crate::core::value::BitState::{F, T, X, E};

    match a {
        F => { T }
        T => { F }
        X => { X }
        E => { E }
    }
}

pub fn assign(a: BitState, b: BitState) -> BitState {
    use crate::core::value::BitState::{F, T, X, E};

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
    use crate::core::value::BitState::{F, T, X, E};

    match (a, b) {
        (F, F) => { F }
        (F, T) => { F }
        (F, X) => { F }
        (F, E) => { E }
        (T, T) => { T }
        (T, X) => { X }
        (T, E) => { E }
        (X, X) => { X }
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
    use crate::core::value::BitState::{F, T, X, E};

    match (a, b) {
        (F, F) => { F }
        (F, T) => { T }
        (F, X) => { X }
        (F, E) => { E }
        (T, T) => { T }
        (T, X) => { T }
        (T, E) => { E }
        (X, X) => { X }
        (X, E) => { E }
        (E, E) => { E }

        (l, r) => { or(r, l) }
    }
}