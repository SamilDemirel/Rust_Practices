/* Key Questions to Systematically Define Test Scenarios



A. Happy Path
    What is the most common, expected usage of this function?
    These are standard input cases where everything works as intended.
    Example: add(2, 3) should return 5. 

B. Edge Cases
    What happens at the limits of valid input?
    Test inputs at the boundaries of what is allowed or meaningful.
    Examples:
        Empty inputs (e.g., [], "")
        Minimum and maximum values (e.g., i32::MIN, i32::MAX)
        Zero, negative values, etc.

C. Invalid or Unexpected Input
    How does the function behave with incorrect or invalid inputs?
    This includes values that are not supposed to work.
    Should it panic? Return None or an error? Fail gracefully?
    Examples:
        Division by zero
        Invalid string format
        Null/None values (where applicable)

D. Side Effects or State Changes

    Does the function have any observable effects beyond its return value?
    If the function writes to a file, modifies a global state, or interacts with an external system, test for those effects.
    Examples:
        File creation or content
        Console output
        Database updates

E. Performance / Stress (Advanced)
    What happens under high load or large inputs?
    These tests aren't always necessary for small functions, but for performance-sensitive code, you should consider them.
    Examples:
        Processing 1 million elements
        Long-running loops or recursive calls

 */
fn main(){



}