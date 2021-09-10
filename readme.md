# Data Fission - as opposed to DataFusion

Just a learning project for now.

The idea is to understand most current abstractions within DataFusion, so that modifying them can become an easy task later on.
But since datafusion is such a big project, perhaps it is worth it to extract the _essential_ concepts into a separate project.

## DataFission Learning Path

1. Implementing the Type System: Schema, Field, FieldVector, ArrowType, ColumnVector, RecordBatch
2. Data Source Interface: Ability to parse data source to read its Schema and Data
3. Logical Plans and Expressions
4. Physical Plans and Expressions