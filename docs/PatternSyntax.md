# Pattern Syntax

This syntax is inspired by regular expressions but is specifically designed for Gordian Envelope.

The pattern syntax is designed to be flexible and expressive. Patterns can be composed of *leaf patterns*, *structure patterns*, and combinators known as *meta-patterns*.

Keywords like `BOOL`, `ARRAY`, `MAP`, etc., are case-sensitive and must be written in uppercase. Patterns can include specific values, ranges, or regexes to match against the corresponding parts of the envelope.

Spaces may used to separate different parts of the pattern.

Parentheses are used to group patterns or specify ranges. The syntax `(pattern)` is really just the repeat pattern with a repeat that matches the pattern exactly once.

The syntax integrates the dCBOR diagnostic notation for matching CBOR values, so we will use the `dcbor-parse` crate to parse these values.

The result of successful parsing is a `Pattern` object, which can be used to match against Gordian Envelope.

White space is ignored between tokens, so you can use it to make patterns more readable. The syntax examples below includ white space both to show where it can be used and to show where it *cannot* be used (i.e., between characters of a token like `*?`)

# Leaf Patterns

All leaf patterns match Envelope leaves, which are CBOR values.

- Leaf
    -  `LEAF`
        - Matches any leaf value.
- Array
    - `ARRAY`
        - Matches any array.
    - `ARRAY ( { n } )`
        - Matches an array with exactly `n` elements.
    - `ARRAY ( { n , m } )`
        - Matches an array with between `n` and `m` elements, inclusive.
    - `ARRAY ( { n , } )`
        - Matches an array with at least `n` elements.
- Boolean
    - `BOOL`
        - Matches any boolean value.
    - `BOOL ( true )`
        - Matches the boolean value `true`.
    - `BOOL ( false )`
        - Matches the boolean value `false`.
- ByteString
    - `BSTR`
        - Matches any byte string.
    - `BSTR ( h'hex' )`
        - Matches a byte string with the specified hex value. Note that the `h'...'` syntax is used to denote hex strings in CBOR diagnostic notation, so we use it here for familiarity.
    - `BSTR ( /regex/ )`
        - Matches a byte string that matches the specified binary regex.
- CBOR
    - `CBOR`
        - Matches any CBOR value.
    - `CBOR ( diagnostic-notation )`
        - Matches a CBOR value that matches the specified diagnostic notation, parsed using the `dcbor-parse` crate, which uses the `logos` crate for parsing.
    - `CBOR ( ur:type/value )`
        - Matches a CBOR value that matches the specified `ur`, parsed using the `bc-ur` crate.
- Date
    - `DATE`
        - Matches any date value.
    - `DATE ( iso-8601 )`
        - Matches a date value with the specified ISO 8601 format. This is a bare string with no delimiters apart from the enclosing parentheses.
    - `DATE ( iso-8601 ... iso-8601 )`
        - Matches a date value within the specified range.
    - `DATE ( iso-8601 ... )`
        - Matches a date value greater than or equal to the specified ISO 8601 date.
    - `DATE ( ... iso-8601 )`
        - Matches a date value less than or equal to the specified ISO 8601 date.
    - `DATE ( /regex/ )`
        - Matches a date value that matches the specified regex.
- Known Value
    - `KNOWN`
        - Matches any known value. (See the `known-values` crate for more information.)
    - `KNOWN ( 'value' )`
        - Matches the specified known value, which is a u64 value. Gordian Envelope prints known values enclosed in single quotes, so we use that syntax here for familiarity.
    - `KNOWN ( 'name' )`
        - Matches the known value with the specified name. Again we use single quotes here for familiarity.
    - `KNOWN ( /regex/ )`
        - Matches a known value with a name that matches the specified regex. We do not use the single quotes here.
- Map
    - `MAP`
        - Matches any map.
    - `MAP ( n )`
        - Matches a map with exactly `n` entries.
    - `MAP ( { n , m } )`
        - Matches a map with between `n` and `m` entries, inclusive.
- Null
    - `NULL`
        - Matches the null value.
- Number
    - `NUMBER`
        - Matches any number.
    - `NUMBER ( value )`
        - Matches the specified number.
    - `NUMBER ( value ... value )`
        - Matches a number within the specified range.
    - `NUMBER ( >= value )`
        - Matches a number greater than or equal to the specified value.
    - `NUMBER ( <= value )`
        - Matches a number less than or equal to the specified value.
    - `NUMBER ( > value )`
        - Matches a number greater than the specified value.
    - `NUMBER ( < value )`
        - Matches a number less than the specified value.
    - `NUMBER ( NaN )`
        - Matches the NaN (Not a Number) value.
- Tagged
    - `TAG`
        - Matches any CBOR tagged value.
    - `TAG ( value )`
        - Matches the specified CBOR tagged value. This is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `TAG ( name )`
        - Matches the CBOR tagged value with the specified name. It is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `TAG ( /regex/ )`
        - Matches a CBOR tagged value with a name that matches the specified regex.
- Text
    - `TEXT`
        - Matches any text value.
    - `TEXT ( "string" )`
        - Matches a text value with the specified string. Gordian Envelope and CBOR diagnostic notation use double quotes for text strings, so we use that syntax here for familiarity.
    - `TEXT ( /regex/ )`
        - Matches a text value that matches the specified regex. No double quotes are used here, as the regex is not a string but a pattern to match against the text value.

## Structure Patterns

Structure patterns match parts of Gordian Envelope structures.

- Assertions
    - `ASSERT`
        - Matches any assertion.
    - `ASSERTPRED ( pattern )`
        - Matches an assertion having a predicate that matches the specified pattern.
    - `ASSERTOBJ ( pattern )`
        - Matches an assertion having an object that matches the specified pattern.
- Digest
    - `DIGEST ( hex )`
        - Matches a digest whose value starts with the specified hex prefix. Up to 32 bytes can be specified, which is the length of the full SHA-256 digest.
    - `DIGEST ( ur:digest/value )`
        - Matches the specified `ur:digest` value, parsed using the `bc-ur` crate.
- Node
    - `NODE`
        - Matches any Gordian Envelope node, which is an envelope with at least one assertion.
    - `NODE ( { m, n } )`
        - Matches a Gordian Envelope node with between `m` and `n` assertions, inclusive.
- Objects
    - `OBJ`
        - Matches any object.
    - `OBJ ( pattern )`
        - Matches an object that matches the specified pattern.
- Obscured
    - `OBSCURED`
        - Matches any obscured (elided, encrypted, or compressed) branch of the Envelope tree.
    - `ELIDED`
        - Matches any elided branch of the Envelope tree.
    - `ENCRYPTED`
        - Matches any encrypted branch of the Envelope tree.
    - `COMPRESSED`
        - Matches any compressed branch of the Envelope tree.
- Predicates
    - `PRED`
        - Matches any predicate.
    - `PRED ( pattern )`
        - Matches a predicate that matches the specified pattern.
- Subjects
    - `SUBJECT`
        - Matches any subject. If the envelope is not a NODE, then this is the identity function.
    - `SUBJECT ( pattern )`
        - Matches a subject that matches the specified pattern.
- Wrapped
    - `WRAPPED`
        - Matches any wrapped Envelope.
    - `UNWRAP`
        - Matches on the content of a wrapped Envelope.

## Meta Patterns

The following meta patterns are available to combine or modify other patterns.

Precedence: Repeat has the highest precedence, followed by And, Not, Sequence, and then Or. Parentheses can be used to group patterns and change precedence.

- And
    - `pattern & pattern & pattern`…
        - Matches if all specified patterns match.
- Any
    - `ANY`
        - Always matches.
- Capture
    - `@name ( pattern )`
        - Matches the specified pattern and captures the match for later use with the given name.
- None
    - `NONE`
        - Never matches.
- Not
    - `! pattern`
        - Matches if the specified pattern does not match.
- Or
    - `pattern | pattern | pattern…`
        - Matches if any of the specified patterns match.
- Repeat
    - Greedy — grabs as many repetitions as possible, then backtracks if the rest of the pattern cannot match.
        - `( pattern )` (exactly once, this is used to group patterns)
        - `( pattern )*` (0 or more)
        - `( pattern )?` (0 or 1)
        - `( pattern )+` (1 or more)
        - `( pattern ){ n , m }` (`n` to `m` repeats, inclusive)
    - Lazy — starts with as few repetitions as possible, adding more only if the rest of the pattern cannot match.
        - `( pattern )*?` (0 or more)
        - `( pattern )??` (0 or 1)
        - `( pattern )+?` (1 or more)
        - `( pattern ){ n , m }?` (`n` to `m` repeats, inclusive)
    - Possessive — grabs as many repetitions as possible and never backtracks; if the rest of the pattern cannot match, the whole match fails.
        - `( pattern )*+` (0 or more)
        - `( pattern )?+` (0 or 1)
        - `( pattern )++` (1 or more)
        - `( pattern ){ n , m }+` (`n` to `m` repeats, inclusive)
- Search
    - `SEARCH ( pattern )`
      - Visits every node in the Envelope tree, matching the specified pattern against each node.
- Sequence
    - `pattern > pattern > pattern`
        - Matches if the specified patterns match in sequence, with no other nodes in between.
