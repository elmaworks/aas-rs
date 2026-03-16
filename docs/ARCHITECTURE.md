## The project architecture

The project structure follows the structure of the project being translated.

- @aas-core3.0-typescript/src/common.ts -> @src/common/
- @aas-core3.0-typescript/src/constants.ts -> @src/constants/
- @aas-core3.0-typescript/src/jsonization.ts -> @src/jsonization/
- @aas-core3.0-typescript/src/stringification.ts -> @src/stringification/
- @aas-core3.0-typescript/src/types.ts -> @src/types/
- @aas-core3.0-typescript/src/verification.ts -> @src/verification/

## Test Requirements

All Rust tests must satisfy the corresponding TypeScript tests from `@aas-core3.0-typescript/test/`.

| TypeScript Test                                       | Rust Test                                                      |
| ----------------------------------------------------- | -------------------------------------------------------------- |
| `common.base64.spec.ts`                               | `tests/test_common_base64.rs`                                  |
| `common.base64url.spec.ts`                            | `tests/test_common_base64url.rs`                               |
| `examples.spec.ts`                                    | `tests/test_examples.rs`                                       |
| `jsonization.concreteClasses.spec.ts`                 | `tests/test_jsonization_concrete_classes.rs`                   |
| `jsonization.concreteClassesOutsideContainer.spec.ts` | `tests/test_jsonization_concrete_classes_outside_container.rs` |
| `jsonization.enums.spec.ts`                           | `tests/test_jsonization_enums.rs`                              |
| `jsonization.interfaces.spec.ts`                      | `tests/test_jsonization_interfaces.rs`                         |
| `types.casts.spec.ts`                                 | `tests/test_types_casts.rs`                                    |
| `types.descendAndPassThroughVisitor.spec.ts`          | `tests/test_types_descend_and_pass_through_visitor.rs`         |
| `types.descendOnce.spec.ts`                           | `tests/test_types_descend_once.rs`                             |
| `types.modelType.spec.ts`                             | `tests/test_types_model_type.rs`                               |
| `types.overEnum.spec.ts`                              | `tests/test_types_over_enum.rs`                                |
| `types.overXOrEmpty.spec.ts`                          | `tests/test_types_over_x_or_empty.rs`                          |
| `types.typeMatches.spec.ts`                           | `tests/test_types_type_matches.rs`                             |
| `types.xOrDefault.spec.ts`                            | `tests/test_types_x_or_default.rs`                             |
| `verification.date.spec.ts`                           | `tests/test_verification_date.rs`                              |

All tests are stored in @tests/, and test data is sourced from the following directory.

- @aas-core3.0-typescript/test_data -> @test_data/
