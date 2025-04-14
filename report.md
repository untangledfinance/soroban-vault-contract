

<style>
.markdown-body table {min-width: 100%;width: 100%;display: table;}
thead {min-width: 100%;width: 100%;}
th {min-width: 60%;width: 60%;}
th:last-child {min-width: 20%;width: 20%;}
th:first-child {min-width: 20%;width: 20%;}
</style>



# Scout Report - Soroban Vault Contract - 2025-04-14

## Summary

| <span style="color:green">Crate</span> | <span style="color:green">Status</span> | <span style="color:green">Critical</span> | <span style="color:green">Medium</span> | <span style="color:green">Minor</span> | <span style="color:green">Enhancement</span> | 
| - | - | - | - | - | - | 
| untangled_vault | Analyzed | 0 | 0 | 0 | 2 | 


Issues found:



- [Soroban Version](#soroban-version) (1 results) (Enhancement)

- [Assert Violation](#assert-violation) (1 results) (Enhancement)



## Best Practices



### Soroban Version

**Impact:** Enhancement

**Issue:** Use the latest version of Soroban

**Description:** Using a older version of Soroban can be dangerous, as it may have bugs or security issues. Use the latest version available.

[**Learn More**](https://coinfabrik.github.io/scout-audit/docs/detectors/soroban/soroban-version)

#### Findings

| ID  | Package | File Location |
| --- | ------- | ------------- |
| 1 | contracts | [lib.rs:1:1 - 1:1](contracts/vault/src/lib.rs) |



## Error Handling



### Assert Violation

**Impact:** Enhancement

**Issue:** Assert causes panic. Instead, return a proper error.

**Description:** Using assert! macro in production code can cause unexpected panics. This violates best practices for smart contract error handling.

[**Learn More**](https://coinfabrik.github.io/scout-audit/docs/detectors/rust/assert-violation)

#### Findings

| ID  | Package | File Location |
| --- | ------- | ------------- |
| 0 | contracts | [test.rs:32:5 - 52:6](contracts/vault/src/test.rs) |


