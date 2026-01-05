# Module cp_solver Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/intervals/mod.rs.html#20" class="src">Source</a>

Expand description

Constraint propagator/solver for custom [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") graphs.

The constraint propagator/solver in DataFusion uses interval arithmetic to perform mathematical operations on intervals, which represent a range of possible values rather than a single point value. This allows for the propagation of ranges through mathematical operations, and can be used to compute bounds for a complicated expression. The key idea is that by breaking down a complicated expression into simpler terms, and then combining the bounds for those simpler terms, one can obtain bounds for the overall expression.

This way of using interval arithmetic to compute bounds for a complex expression by combining the bounds for the constituent terms within the original expression allows us to reason about the range of possible values of the expression. This information later can be used in range pruning of the provably unnecessary parts of `RecordBatch`es.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html#example" class="doc-anchor">§</a>Example

For example, consider a mathematical expression such as `x^2 + y = 4` \[1\]. Since this expression would be a binary tree in [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") notation, this type of an hierarchical computation is well-suited for a graph based implementation. In such an implementation, an equation system `f(x) = 0` is represented by a directed acyclic expression graph (DAEG).

In order to use interval arithmetic to compute bounds for this expression, one would first determine intervals that represent the possible values of `x` and `y` Let’s say that the interval for `x` is `[1, 2]` and the interval for `y` is `[-3, 1]`. In the chart below, you can see how the computation takes place.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html#references" class="doc-anchor">§</a>References

1.  Kabak, Mehmet Ozan. Analog Circuit Start-Up Behavior Analysis: An Interval Arithmetic Based Approach, Chapter 4. Stanford University, 2015.
2.  Moore, Ramon E. Interval analysis. Vol. 4. Englewood Cliffs: Prentice-Hall, 1966.
3.  F. Messine, “Deterministic global optimization using interval constraint propagation techniques,” RAIRO-Operations Research, vol. 38, no. 04, pp. 277-293, 2004.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html#illustration" class="doc-anchor">§</a>Illustration

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html#computing-bounds-for-an-expression-using-interval-arithmetic" class="doc-anchor">§</a>Computing bounds for an expression using interval arithmetic

``` text
            +-----+                         +-----+
       +----|  +  |----+               +----|  +  |----+
       |    |     |    |               |    |     |    |
       |    +-----+    |               |    +-----+    |
       |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |           |  y  |       |   2 | [1, 4]    |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
      |                               |
      |                               |
    +---+                           +---+
    | x | [1, 2]                    | x | [1, 2]
    +---+                           +---+

 (a) Bottom-up evaluation: Step 1 (b) Bottom up evaluation: Step 2

                                     [1 - 3, 4 + 1] = [-2, 5]
            +-----+                         +-----+
       +----|  +  |----+               +----|  +  |----+
       |    |     |    |               |    |     |    |
       |    +-----+    |               |    +-----+    |
       |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |[1, 4]     |  y  |       |   2 |[1, 4]     |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
      |              [-3, 1]          |              [-3, 1]
      |                               |
    +---+                           +---+
    | x | [1, 2]                    | x | [1, 2]
    +---+                           +---+

 (c) Bottom-up evaluation: Step 3 (d) Bottom-up evaluation: Step 4
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html#top-down-constraint-propagation-using-inverse-semantics" class="doc-anchor">§</a>Top-down constraint propagation using inverse semantics

``` text
   [-2, 5] ∩ [4, 4] = [4, 4]               [4, 4]
           +-----+                         +-----+
      +----|  +  |----+               +----|  +  |----+
      |    |     |    |               |    |     |    |
      |    +-----+    |               |    +-----+    |
      |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 | [1, 4]    |  y  |       |   2 | [1, 4]    |  y  | [0, 1]*
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
     |              [-3, 1]          |
     |                               |
   +---+                           +---+
   | x | [1, 2]                    | x | [1, 2]
   +---+                           +---+

 (a) Top-down propagation: Step 1 (b) Top-down propagation: Step 2

                                    [1 - 3, 4 + 1] = [-2, 5]
           +-----+                         +-----+
      +----|  +  |----+               +----|  +  |----+
      |    |     |    |               |    |     |    |
      |    +-----+    |               |    +-----+    |
      |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |[3, 4]**   |  y  |       |   2 |[3, 4]     |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
     |              [0, 1]           |              [-3, 1]
     |                               |
   +---+                           +---+
   | x | [1, 2]                    | x | [sqrt(3), 2]***
   +---+                           +---+

 (c) Top-down propagation: Step 3  (d) Top-down propagation: Step 4

   * [-3, 1] ∩ ([4, 4] - [1, 4]) = [0, 1]
   ** [1, 4] ∩ ([4, 4] - [0, 1]) = [3, 4]
   *** [1, 2] ∩ [sqrt(3), sqrt(4)] = [sqrt(3), 2]
```

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/struct.ExprIntervalGraph.html" class="struct" title="struct datafusion::physical_expr::intervals::cp_solver::ExprIntervalGraph">ExprIntervalGraph</a>  
This object implements a directed acyclic expression graph (DAEG) that is used to compute ranges for expressions through interval arithmetic.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/struct.ExprIntervalGraphNode.html" class="struct" title="struct datafusion::physical_expr::intervals::cp_solver::ExprIntervalGraphNode">ExprIntervalGraphNode</a>  
This is a node in the DAEG; it encapsulates a reference to the actual [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") as well as an interval containing expression bounds.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/enum.PropagationResult.html" class="enum" title="enum datafusion::physical_expr::intervals::cp_solver::PropagationResult">PropagationResult</a>  
This object encapsulates all possible constraint propagation results.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/fn.propagate_arithmetic.html" class="fn" title="fn datafusion::physical_expr::intervals::cp_solver::propagate_arithmetic">propagate_arithmetic</a>  
This function refines intervals `left_child` and `right_child` by applying constraint propagation through `parent` via operation. The main idea is that we can shrink ranges of variables x and y using parent interval p.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/fn.propagate_comparison.html" class="fn" title="fn datafusion::physical_expr::intervals::cp_solver::propagate_comparison">propagate_comparison</a>  
This function refines intervals `left_child` and `right_child` by applying comparison propagation through `parent` via operation. The main idea is that we can shrink ranges of variables x and y using parent interval p. Two intervals can be ordered in 6 ways for a Gt `>` operator:
