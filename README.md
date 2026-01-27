### Project draft — ```dev``` branch 
### Rewritten format on the new architecture — ```rewrite/flatten-compiler```

The experimental project is under active development. 
As of January 2026, a DRAFT with a capability analysis had been implemented. 
Work is underway to migrate the compiler to a new architecture, involving a code rewrite and a new philosophy:
* Code generation itself is purely rule-based. 
* All optimizations and runtime integration are handled by the lowering stage (IR layer).
