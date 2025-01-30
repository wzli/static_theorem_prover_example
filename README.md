# Theorem Prover in Rust: Static Proofs via Curry-Howard Correspondence

A Rust crate demonstrating compile-time theorem proving by mapping logical propositions to types and proofs to programs. Inspired by the Curry-Howard correspondence, this crate models classical logic constructs using Rust’s type system, enabling static verification of theorems.

---

## Core Concepts

### Propositions as Types
- **Prop Trait**:  
  A proposition is represented by a type implementing the `Prop` trait. The associated constant `VALUE` is `true` if the proposition is provable.  
  Example:
  ```rust
  pub trait Prop: 'static {
      const VALUE: bool;
  }
  ```

- **Atomic Propositions**:  
  `True` (`Bool<true>`) and `False` (`Bool<false>`) are basic truth values.  
  ```rust
  pub struct Bool<const B: bool>;
  pub type True = Bool<true>;
  pub type False = Bool<false>;
  ```

### Proofs as Programs
- Proofs are functions or closures that transform evidence (values) of one proposition into another.  
  Example: A proof of `And<A, B>` requires values of both `A` and `B`.

---

## Logical Connectives

### Conjunction (`And<A, B>`)
- **True if both `A` and `B` are true**.  
  Struct holding proofs of `A` and `B`.  
  ```rust
  pub struct And<A, B>(A, B);
  impl<A: Prop, B: Prop> Prop for And<A, B> {
      const VALUE: bool = A::VALUE && B::VALUE;
  }
  ```

### Disjunction (`Or<L, R>`)
- **True if either `L` or `R` is true**.  
  Enum with variants `L(L)` and `R(R)`.  
  ```rust
  pub enum Or<L, R> {
      L(L),
      R(R),
  }
  impl<L: Prop, R: Prop> Prop for Or<L, R> {
      const VALUE: bool = L::VALUE || R::VALUE;
  }
  ```

### Implication (`Imply<P, Q>`)
- **A function from `P` to `Q`**. True if `P` is false or `Q` is true.  
  Implemented as a boxed closure for runtime flexibility.  
  ```rust
  pub type Imply<P, Q> = Box<dyn FnOnce(P) -> Q>;
  impl<P: Prop, Q: Prop> Prop for Imply<P, Q> {
      const VALUE: bool = !P::VALUE || Q::VALUE;
  }
  ```

### Negation (`Not<P>`)
- **Equivalent to `Imply<P, False>`**:  
  ```rust
  pub type Not<P> = Imply<P, False>;
  ```

### Equivalence (`Equal<P, Q>`)
- **Bi-directional implication**: Proves `P → Q` and `Q → P`.  
  ```rust
  pub type Equal<P, Q> = And<Imply<P, Q>, Imply<Q, P>>;
  ```

---

## Axioms and Theorems

### Axioms
- **Law of Excluded Middle**:  
  `P ∨ ¬P` (Classical logic axiom).  
  ```rust
  pub fn excluded_middle<P: Prop>() -> Or<P, Not<P>> {
      axiom() // Constructed at compile time
  }
  ```

- **Ex Falso Quodlibet**:  
  A false proposition implies anything.  
  ```rust
  pub fn exfalso<P: Prop>(_: False) -> P {
      axiom()
  }
  ```

### Theorems
- **Commutativity of Conjunction**:  
  Swaps the order of `And<A, B>`.  
  ```rust
  pub fn and_comm<A: Prop, B: Prop>(h: And<A, B>) -> And<B, A> {
      And(h.1, h.0) // Swap the terms
  }
  ```

- **Double Negation**:  
  Proves `P` is equivalent to `¬¬P`.  
  ```rust
  pub fn double_negation<P: Prop>() -> Equal<P, Not<Not<P>>> {
      And(
          λ!(|p| double_negation_introduction(p)), // P → ¬¬P
          λ!(|nnp| double_negation_elimination(nnp)), // ¬¬P → P
      )
  }
  ```

- **Contraposition**:  
  Proves `P → Q` is equivalent to `¬Q → ¬P`.  
  ```rust
  pub fn contraposition<P: Prop, Q: Prop>() -> Equal<Imply<P, Q>, Imply<Not<Q>, Not<P>>> {
      And(
          λ!(|h| contraposition_forward(h)), // Forward direction
          λ!(|h| contraposition_reverse(h)), // Reverse direction
      )
  }
  ```

---

## Examples

### Proving Commutativity
```rust
use your_crate::{And, and_comm};

// Given a proof of `And<True, False>`, derive `And<False, True>`
let proof: And<True, False> = And(True, False);
let swapped = and_comm(proof); // Returns `And<False, True>`
```

### Material Implication
```rust
use your_crate::{material_implication, Or, Not};

// Convert an implication `Imply<True, False>` to a disjunction `Or<Not<True>, False>`
let imply: Imply<True, False> = Box::new(|_| unreachable!());
let disjunction = material_implication(imply); // Returns `Or::L(Not<True>)`
```

---

## Limitations
1. **Static Verification**: Proofs are checked at compile time and cannot depend on runtime values.
2. **Classical Logic**: Uses `excluded_middle`, making it incompatible with intuitionistic logic.
3. **No Dependent Types**: Cannot express proofs that depend on runtime data.

---


**Note**: This is an educational tool, not a production-grade proof assistant. For advanced theorem proving, consider languages like Lean or Coq.
