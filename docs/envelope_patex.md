# Envelope Pattern Expression Syntax (_patex_)

This syntax is inspired by regular expressions but is specifically designed for Gordian Envelope.

The patex syntax is designed to be flexible and expressive. Patterns can be composed of *leaf patterns*, *structure patterns*, and combinators known as *meta-patterns*.

Keywords like `bool`, `array`, etc., are case-sensitive. Patterns can include specific values, ranges, or regexes to match against the corresponding parts of the envelope.

Spaces may used to separate different parts of the pattern.

Parentheses are used to group patterns or specify ranges. The syntax `(pattern)` is really just the repeat pattern with a repeat that matches the pattern exactly once.

The syntax integrates the dCBOR diagnostic notation for matching CBOR values, so we will use the `dcbor-parse` crate to parse these values. For patterns that don't match envelope-specific syntax, the parser will fall back to using dcbor-pattern syntax, providing seamless integration between the two pattern systems.

The result of successful parsing is a `Pattern` object, which can be used to match against Gordian Envelope.

White space is ignored between tokens, so you can use it to make patterns more readable. The syntax examples below includ white space both to show where it can be used and to show where it *cannot* be used (i.e., between characters of a token like `*?`)

# Leaf Patterns

All leaf patterns match Envelope leaves, which are CBOR values.

- Boolean
    - `bool`
        - Matches any boolean value.
    - `true`
        - Matches the boolean value `true`.
    - `false`
        - Matches the boolean value `false`.
- ByteString
    - `bstr`
        - Matches any byte string.
    - `h'hex'`
        - Matches a byte string with the specified hex value. Note that the `h'...'` syntax is used to denote hex strings in CBOR diagnostic notation, so we use it here for familiarity.
    - `h'/regex/'`
        - Matches a byte string that matches the specified binary regex.
- Date
    - `date`
        - Matches any date value.
    - `date'iso-8601'`
        - Matches a date value with the specified ISO 8601 format.
    - `date'iso-8601...iso-8601'`
        - Matches a date value within the specified range.
    - `date'iso-8601...'`
        - Matches a date value greater than or equal to the specified ISO 8601 date.
    - `date'...iso-8601'`
        - Matches a date value less than or equal to the specified ISO 8601 date.
    - `date'/regex/'`
        - Matches a date value that matches the specified regex.
- Known Value
    - `known`
        - Matches any known value. (See the `known-values` crate for more information.)
    - `'value'`
        - Matches the specified known value, which is a u64 value. dCBOR prints known values enclosed in single quotes, so we use that syntax here for familiarity.
    - `'name'`
        - Matches the known value with the specified name. Again we use single quotes here for familiarity.
    - `'/regex/'`
        - Matches a known value with a name that matches the specified regex.
- Null
    - `null`
        - Matches the null value.
- Number
    - `number`
        - Matches any number.
    - `value`
        - Matches the specified number.
    - `value...value`
        - Matches a number within the specified range.
    - `>=value`
        - Matches a number greater than or equal to the specified value.
    - `<=value`
        - Matches a number less than or equal to the specified value.
    - `>value`
        - Matches a number greater than the specified value.
    - `<value`
        - Matches a number less than the specified value.
    - `NaN`
        - Matches the NaN (Not a Number) value.
    - `Infinity`
        - Matches positive infinity.
    - `-Infinity`
        - Matches negative infinity.
- Text
    - `text`
        - Matches any text value.
    - `"string"`
        - Matches a text value with the specified string. Gordian Envelope and CBOR diagnostic notation use double quotes for text strings, so we use that syntax here for familiarity.
    - `/regex/`
        - Matches a text value that matches the specified regex. No double quotes are used here, as the regex is not a string but a pattern to match against the text value.
- Array
    - `[*]`
        - Matches any array.
    - `[{n}]`
        - Matches an array with exactly `n` elements.
    - `[{n,m}]`
        - Matches an array with between `n` and `m` elements, inclusive.
    - `[{n,}]`
        - Matches an array with at least `n` elements.
    - `[dcbor-patex, dcbor-patex, ...]`
        - Matches an array where the elements match the specified pattern. The pattern can be a simple pattern, a sequence of patterns, or patterns with repeat quantifiers.
        - Examples:
            - `[42]` - Array containing exactly one element: the number 42
            - `["a", "b", "c"]` - Array containing exactly ["a", "b", "c"] in sequence
            - `[(*)*, 42, (*)*]` - Array containing 42 anywhere within it
            - `[42, (*)*]` - Array starting with 42, followed by any elements
            - `[(*)*, 42]` - Array ending with 42, preceded by any elements
- Map
    - `{*}`
        - Matches any map.
    - `{{n}}`
        - Matches a map with exactly `n` entries.
    - `{{n,m}}`
        - Matches a map with between `n` and `m` entries, inclusive.
    - `{{n,}}`
        - Matches a map with at least `n` entries.
    - `{key-dcbor-patex: value-dcbor-patex, ...}`
        - Matches a map with the specified key-value patterns.
- Tagged
    - `tagged`
        - Matches any CBOR tagged value.
    - `tagged ( value, dcbor-patex )`
        - Matches the specified CBOR tagged value with content that matches the given pattern. The tag value is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `tagged ( name, dcbor-patex )`
        - Matches the CBOR tagged value with the specified name and content that matches the given pattern. The tag name is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `tagged ( /regex/, dcbor-patex )`
        - Matches a CBOR tagged value with a name that matches the specified regex and content that matches the given pattern.
- CBOR
    - `cbor`
        - Matches any subject CBOR value.
    - `cbor ( dcbor-diagnostic-notation )`
        - Matches a subject CBOR value that matches the specified diagnostic notation, parsed using the `dcbor-parse` crate, which uses the `logos` crate for parsing.
    - `cbor ( ur:type/value )`
        - Matches a subject CBOR value that matches the specified `ur`, parsed using the `bc-ur` crate.
    - `cbor ( /dcbor-patex/ )`
        - Matches a subject CBOR value that matches the specified dcbor-pattern expression. This enables advanced pattern matching within CBOR structures including quantifiers, captures, and complex structural patterns. The pattern expression uses dcbor-pattern syntax.

## Structure Patterns

Structure patterns match parts of Gordian Envelope structures.

- Leaf
    - `leaf`
        - Matches any leaf envelope (terminal nodes in the envelope tree), a "bare subject". This is distinct from the `node` pattern, which matches a subject with one or more assertions.
- Assertions
    - `assert`
        - Matches any assertion.
    - `assertpred ( patex )`
        - Matches an assertion having a predicate that matches the specified pattern.
    - `assertobj ( patex )`
        - Matches an assertion having an object that matches the specified pattern.
- Digest
    - `digest ( hex )`
        - Matches a digest whose value starts with the specified hex prefix. Up to 32 bytes can be specified, which is the length of the full SHA-256 digest.
    - `digest ( ur:digest/value )`
        - Matches the specified `ur:digest` value, parsed using the `bc-ur` crate.
- Node
    - `node`
        - Matches any Gordian Envelope node, which is an envelope with at least one assertion.
    - `node ( { n, m } )`
        - Matches a Gordian Envelope node with between `n` and `m` assertions, inclusive. An `n` of zero will never match.
- Objects
    - `obj`
        - Matches any object.
    - `obj ( patex )`
        - Matches an object that matches the specified pattern.
- Obscured
    - `obscured`
        - Matches any obscured (elided, encrypted, or compressed) branch of the Envelope tree.
    - `elided`
        - Matches any elided branch of the Envelope tree.
    - `encrypted`
        - Matches any encrypted branch of the Envelope tree.
    - `compressed`
        - Matches any compressed branch of the Envelope tree.
- Predicates
    - `pred`
        - Matches any predicate.
    - `pred ( patex )`
        - Matches a predicate that matches the specified pattern.
- Subjects
    - `subj`
        - Matches any subject. If the envelope is not a NODE, then this is the identity function.
    - `subj ( patex )`
        - Matches a subject that matches the specified pattern.
- Wrapped
    - `wrapped`
        - Matches any wrapped Envelope.
    - `unwrap`
        - Matches on the content of a wrapped Envelope.

## Meta Patterns

The following meta patterns are available to combine or modify other patterns.

Precedence: Repeat has the highest precedence, followed by And, Not, Traversal, and then Or. Parentheses can be used to group patterns and change precedence.

- And
    - `patex & patex & patex...`
        - Matches if all specified patterns match.
- Any
    - `*`
        - Always matches.
- Capture
    - `@name ( patex )`
        - Matches the specified pattern and captures the match for later use with the given name.
- Not
    - `! patex`
        - Matches if the specified patex does not match.
        - A pattern that never matches can be represented as `!*`.
- Or
    - `patex | patex | pattern…`
        - Matches if any of the specified patterns match.
- Repeat
    - Greedy — grabs as many repetitions as possible, then backtracks if the rest of the patex cannot match.
        - `( patex )` (exactly once, this is used to group patterns)
        - `( patex )*` (0 or more)
        - `( patex )?` (0 or 1)
        - `( patex )+` (1 or more)
        - `( patex ){ n , m }` (`n` to `m` repeats, inclusive)
    - Lazy — starts with as few repetitions as possible, adding more only if the rest of the pattern cannot match.
        - `( patex )*?` (0 or more)
        - `( patex )??` (0 or 1)
        - `( patex )+?` (1 or more)
        - `( patex ){ n , m }?` (`n` to `m` repeats, inclusive)
    - Possessive — grabs as many repetitions as possible and never backtracks; if the rest of the pattern cannot match, the whole match fails.
        - `( patex )*+` (0 or more)
        - `( patex )?+` (0 or 1)
        - `( patex )++` (1 or more)
        - `( patex ){ n , m }+` (`n` to `m` repeats, inclusive)
- Search
    - `search ( patex )`
      - Visits every node in the Envelope tree, matching the specified pattern against each node.
- Traversal
    - `patex -> patex -> patex`
        - Matches if the specified patterns match a traversal path, with no other nodes in between.
