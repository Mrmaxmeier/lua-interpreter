assert(true)
assert(not false)
assert(true ~= false)
assert(false == false)
assert(2 > 1)
assert(3 <= 4)
assert(#"foobar" == 6)
assert(("foo" .. "bar" .. "baz") == "foobarbaz")
-- TODO: test tables, closures, branches, for-loops