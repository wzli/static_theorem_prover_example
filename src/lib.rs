macro_rules! λ {
    ($($t:tt)*) => {
        Box::new(move $($t)*)
    };
}

pub trait Prop: 'static {
    const VALUE: bool;
}

pub struct Bool<const B: bool>;
pub type True = Bool<true>;
pub type False = Bool<false>;

impl<const B: bool> Prop for Bool<B> {
    const VALUE: bool = B;
}

pub struct And<A, B>(A, B);

impl<A: Prop, B: Prop> Prop for And<A, B> {
    const VALUE: bool = A::VALUE && B::VALUE;
}

pub enum Or<L, R> {
    L(L),
    R(R),
}

impl<L: Prop, R: Prop> Prop for Or<L, R> {
    const VALUE: bool = L::VALUE || R::VALUE;
}

pub type Imply<P, Q> = Box<dyn FnOnce(P) -> Q>;

impl<P: Prop, Q: Prop> Prop for Imply<P, Q> {
    const VALUE: bool = !P::VALUE || Q::VALUE;
}

pub type Not<P> = Imply<P, False>;

pub type Equal<P, Q> = And<Imply<P, Q>, Imply<Q, P>>;

// axioms
pub fn axiom<P: Prop>() -> P {
    unreachable!("axioms cannot be executed")
}

#[deprecated]
pub fn sorry<P: Prop>() -> P {
    axiom()
}

pub fn exfalso<P: Prop>(_: False) -> P {
    axiom()
}

pub fn excluded_middle<P: Prop>() -> Or<P, Not<P>> {
    axiom()
}

// theorems

pub fn and_comm<A: Prop, B: Prop>(h: And<A, B>) -> And<B, A> {
    And(h.1, h.0)
}

pub fn or_comm<L: Prop, R: Prop>(h: Or<L, R>) -> Or<R, L> {
    match h {
        Or::L(l) => Or::R(l),
        Or::R(r) => Or::L(r),
    }
}

pub fn double_negation_introduction<P: Prop>(p: P) -> Not<Not<P>> {
    λ!(|np| np(p))
}

pub fn double_negation_elimination<P: Prop>(nnp: Not<Not<P>>) -> P {
    match excluded_middle() {
        Or::L(p) => p,
        Or::R(np) => exfalso(nnp(np)),
    }
}

pub fn double_negation<P: Prop>() -> Equal<P, Not<Not<P>>> {
    And(
        λ!(|p| double_negation_introduction(p)),
        λ!(|nnp| double_negation_elimination(nnp)),
    )
}

pub fn contraposition_forward<P: Prop, Q: Prop>(h: Imply<P, Q>) -> Imply<Not<Q>, Not<P>> {
    λ!(|nq| λ!(|p| nq(h(p))))
}

pub fn contraposition_reverse<P: Prop, Q: Prop>(h: Imply<Not<Q>, Not<P>>) -> Imply<P, Q> {
    λ!(|p| match excluded_middle() {
        Or::L(q) => q,
        Or::R(nq) => exfalso(h(nq)(p)),
    })
}

pub fn contraposition<P: Prop, Q: Prop>() -> Equal<Imply<P, Q>, Imply<Not<Q>, Not<P>>> {
    And(
        λ!(|h| contraposition_forward(h)),
        λ!(|h| contraposition_reverse(h)),
    )
}

pub fn material_implication_forward<P: Prop, Q: Prop>(h: Imply<P, Q>) -> Or<Not<P>, Q> {
    match excluded_middle() {
        Or::L(p) => Or::R(h(p)),
        Or::R(np) => Or::L(np),
    }
}

pub fn material_implication_reverse<P: Prop, Q: Prop>(h: Or<Not<P>, Q>) -> Imply<P, Q> {
    λ!(|p| match h {
        Or::L(np) => exfalso(np(p)),
        Or::R(q) => q,
    })
}

pub fn material_implication<P: Prop, Q: Prop>() -> Equal<Imply<P, Q>, Or<Not<P>, Q>> {
    And(
        λ!(|h| material_implication_forward(h)),
        λ!(|h| material_implication_reverse(h)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        assert!(True::VALUE);
        assert!(!False::VALUE);
    }

    #[test]
    fn test_not() {
        assert!(!Not::<True>::VALUE);
        assert!(Not::<False>::VALUE);
    }

    #[test]
    fn test_and() {
        assert!(And::<True, True>::VALUE);
        assert!(!And::<True, False>::VALUE);
        assert!(!And::<False, True>::VALUE);
        assert!(!And::<False, False>::VALUE);
    }

    #[test]
    fn test_or() {
        assert!(Or::<True, True>::VALUE);
        assert!(Or::<True, False>::VALUE);
        assert!(Or::<False, True>::VALUE);
        assert!(!Or::<False, False>::VALUE);
    }

    #[test]
    fn test_imply() {
        assert!(Imply::<True, True>::VALUE);
        assert!(!Imply::<True, False>::VALUE);
        assert!(Imply::<False, True>::VALUE);
        assert!(Imply::<False, False>::VALUE);
    }

    #[test]
    fn test_equal() {
        assert!(Equal::<True, True>::VALUE);
        assert!(!Equal::<True, False>::VALUE);
        assert!(!Equal::<False, True>::VALUE);
        assert!(Equal::<False, False>::VALUE);
    }
}
