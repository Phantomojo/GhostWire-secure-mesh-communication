Crate cassowary Copy item path

<span class="sub-heading"><a href="../src/cassowary/lib.rs.html#1-623" class="src">Source</a>
</span>

Expand description

This crate contains an implementation of the Cassowary constraint
solving algorithm, based upon the work by G.J. Badros et al. in 2001.
This algorithm is designed primarily for use constraining elements in
user interfaces. Constraints are linear combinations of the problem
variables. The notable features of Cassowary that make it ideal for user
interfaces are that it is incremental (i.e. you can add and remove
constraints at runtime and it will perform the minimum work to update
the result) and that the constraints can be violated if necessary, with
the order in which they are violated specified by setting a “strength”
for each constraint. This allows the solution to gracefully degrade,
which is useful for when a user interface needs to compromise on its
constraints in order to still be able to display something.

## <a href="#constraint-syntax" class="doc-anchor">§</a>Constraint syntax

This crate aims to provide syntax for describing linear constraints as
naturally as possible, within the limitations of Rust’s type system.
Generally you can write constraints as you would naturally, however the
operator symbol (for greater-than, less-than, equals) is replaced with
an instance of the `WeightedRelation` enum wrapped in “pipe brackets”.

For example, for the constraint `(a + b) * 2 + c >= d + 1` with strength
`s`, the code to use is

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
(a + b) * 2.0 + c |GE(s)| d + 1.0
```

# <a href="#a-simple-example" class="doc-anchor">§</a>A simple example

Imagine a layout consisting of two elements laid out horizontally. For
small window widths the elements should compress to fit, but if there is
enough space they should display at their preferred widths. The first
element will align to the left, and the second to the right. For this
example we will ignore vertical layout.

First we need to include the relevant parts of `cassowary`:

``` {.rust .rust-example-rendered}
use cassowary::{ Solver, Variable };
use cassowary::WeightedRelation::*;
use cassowary::strength::{ WEAK, MEDIUM, STRONG, REQUIRED };
```

And we’ll construct some conveniences for pretty printing (which should
hopefully be self-explanatory):

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use std::collections::HashMap;
let mut names = HashMap::new();
fn print_changes(names: &HashMap<Variable, &'static str>, changes: &[(Variable, f64)]) {
    println!("Changes:");
    for &(ref var, ref val) in changes {
        println!("{}: {}", names[var], val);
    }
}
```

Let’s define the variables required - the left and right edges of the
elements, and the width of the window.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
let window_width = Variable::new();
names.insert(window_width, "window_width");

struct Element {
    left: Variable,
    right: Variable
}
let box1 = Element {
    left: Variable::new(),
    right: Variable::new()
};
names.insert(box1.left, "box1.left");
names.insert(box1.right, "box1.right");

let box2 = Element {
    left: Variable::new(),
    right: Variable::new()
};
names.insert(box2.left, "box2.left");
names.insert(box2.right, "box2.right");
```

Now to set up the solver and constraints.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
let mut solver = Solver::new();
solver.add_constraints(&[window_width |GE(REQUIRED)| 0.0, // positive window width
                         box1.left |EQ(REQUIRED)| 0.0, // left align
                         box2.right |EQ(REQUIRED)| window_width, // right align
                         box2.left |GE(REQUIRED)| box1.right, // no overlap
                         // positive widths
                         box1.left |LE(REQUIRED)| box1.right,
                         box2.left |LE(REQUIRED)| box2.right,
                         // preferred widths:
                         box1.right - box1.left |EQ(WEAK)| 50.0,
                         box2.right - box2.left |EQ(WEAK)| 100.0]).unwrap();
```

The window width is currently free to take any positive value. Let’s
constrain it to a particular value. Since for this example we will
repeatedly change the window width, it is most efficient to use an “edit
variable”, instead of repeatedly removing and adding constraints (note
that for efficiency reasons we cannot edit a normal constraint that has
been added to the solver).

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
solver.add_edit_variable(window_width, STRONG).unwrap();
solver.suggest_value(window_width, 300.0).unwrap();
```

This value of 300 is enough to fit both boxes in with room to spare, so
let’s check that this is the case. We can fetch a list of changes to the
values of variables in the solver. Using the pretty printer defined
earlier we can see what values our variables now hold.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
print_changes(&names, solver.fetch_changes());
```

This should print (in a possibly different order):

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
Changes:
window_width: 300
box1.right: 50
box2.left: 200
box2.right: 300
```

Note that the value of `box1.left` is not mentioned. This is because
`solver.fetch_changes` only lists *changes* to variables, and since each
variable starts in the solver with a value of zero, any values that have
not changed from zero will not be reported.

Now let’s try compressing the window so that the boxes can’t take up
their preferred widths.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
solver.suggest_value(window_width, 75.0);
print_changes(&names, solver.fetch_changes);
```

Now the solver can’t satisfy all of the constraints. It will pick at
least one of the weakest constraints to violate. In this case it will be
one or both of the preferred widths. For efficiency reasons this is
picked nondeterministically, so there are two possible results. This
could be

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
Changes:
window_width: 75
box1.right: 0
box2.left: 0
box2.right: 75
```

or

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
Changes:
window_width: 75
box2.left: 50
box2.right: 75
```

Due to the nature of the algorithm, “in-between” solutions, although
just as valid, are not picked.

In a user interface this is not likely a result we would prefer. The
solution is to add another constraint to control the behaviour when the
preferred widths cannot both be satisfied. In this example we are going
to constrain the boxes to try to maintain a ratio between their widths.

``` {.rust .rust-example-rendered}
solver.add_constraint(
    (box1.right - box1.left) / 50.0 |EQ(MEDIUM)| (box2.right - box2.left) / 100.0
    ).unwrap();
print_changes(&names, solver.fetch_changes());
```

Now the result gives values that maintain the ratio between the sizes of
the two boxes:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
Changes:
box1.right: 25
box2.left: 25
```

This example may have appeared somewhat contrived, but hopefully it
shows the power of the cassowary algorithm for laying out user
interfaces.

One thing that this example exposes is that this crate is a rather low
level library. It does not have any inherent knowledge of user
interfaces, directions or boxes. Thus for use in a user interface this
crate should ideally be wrapped by a higher level API, which is outside
the scope of this crate.

# Modules<a href="#modules" class="anchor">§</a>

<a href="strength/index.html" class="mod"
title="mod cassowary::strength">strength</a>  
Contains useful constants and functions for producing strengths for use
in the constraint solver. Each constraint added to the solver has an
associated strength specifying the precedence the solver should impose
when choosing which constraints to enforce. It will try to enforce all
constraints, but if that is impossible the lowest strength constraints
are the first to be violated.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Constraint.html" class="struct"
title="struct cassowary::Constraint">Constraint</a>  
A constraint, consisting of an equation governed by an expression and a
relational operator, and an associated strength.

<a href="struct.Expression.html" class="struct"
title="struct cassowary::Expression">Expression</a>  
An expression that can be the left hand or right hand side of a
constraint equation. It is a linear combination of variables, i.e. a sum
of variables weighted by coefficients, plus an optional constant.

<a href="struct.PartialConstraint.html" class="struct"
title="struct cassowary::PartialConstraint">PartialConstraint</a>  
This is an intermediate type used in the syntactic sugar for specifying
constraints. You should not use it directly.

<a href="struct.Solver.html" class="struct"
title="struct cassowary::Solver">Solver</a>  
A constraint solver using the Cassowary algorithm. For proper usage
please see the top level crate documentation.

<a href="struct.Term.html" class="struct"
title="struct cassowary::Term">Term</a>  
A variable and a coefficient to multiply that variable by. This is a
sub-expression in a constraint equation.

<a href="struct.Variable.html" class="struct"
title="struct cassowary::Variable">Variable</a>  
Identifies a variable for the constraint solver. Each new variable is
unique in the view of the solver, but copying or cloning the variable
produces a copy of the same variable.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AddConstraintError.html" class="enum"
title="enum cassowary::AddConstraintError">AddConstraintError</a>  
The possible error conditions that `Solver::add_constraint` can fail
with.

<a href="enum.AddEditVariableError.html" class="enum"
title="enum cassowary::AddEditVariableError">AddEditVariableError</a>  
The possible error conditions that `Solver::add_edit_variable` can fail
with.

<a href="enum.RelationalOperator.html" class="enum"
title="enum cassowary::RelationalOperator">RelationalOperator</a>  
The possible relations that a constraint can specify.

<a href="enum.RemoveConstraintError.html" class="enum"
title="enum cassowary::RemoveConstraintError">RemoveConstraintError</a>  
The possible error conditions that `Solver::remove_constraint` can fail
with.

<a href="enum.RemoveEditVariableError.html" class="enum"
title="enum cassowary::RemoveEditVariableError">RemoveEditVariableError</a>  
The possible error conditions that `Solver::remove_edit_variable` can
fail with.

<a href="enum.SuggestValueError.html" class="enum"
title="enum cassowary::SuggestValueError">SuggestValueError</a>  
The possible error conditions that `Solver::suggest_value` can fail
with.

<a href="enum.WeightedRelation.html" class="enum"
title="enum cassowary::WeightedRelation">WeightedRelation</a>  
This is part of the syntactic sugar used for specifying constraints.
This enum should be used as part of a constraint expression. See the
module documentation for more information.
