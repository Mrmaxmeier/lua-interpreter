-- prevent luac constant evaluation
local zero = 1
local one = 1
local four = 4
local twenty = 20
local a, b

assert(true) -- boolean truthiness

assert(false == false) -- equals
assert(true ~= false) -- not equals

assert(5.0 == 5)
assert(5.3 ~= 5)

assert(1 < 2) -- lessthan
assert(3 <= 3) -- lessthenorequal
assert(2 > 1) -- inverted lessthenorequal
assert(2 >= 2) -- inverted lessthan

assert(assert(#assert("foobar")) == 6) -- assert returns arguments
assert(("foo" .. "bar" .. "baz") == "foobarbaz") -- concat

a, b = assert(four, twenty) -- multiple returns
assert(a * (101 - one) + b == b * b + a * (b / a)) -- add, sub, mul, div
assert(four ^ twenty % twenty == 16) -- pow, mod
assert((twenty - one) // four == 4) -- idiv

assert(~zero == -2) -- bnot
assert(-one == -1) -- unm
assert(512 >> one == 256) -- shr
assert(one << four == 16) -- shl
assert(four | twenty == 20) -- band
assert(one | four == 5) -- bor
assert(four ~ twenty == 16) -- bxor


-- TESTSET
assert(false or true == true)
assert(nil or "a" == "a")
assert({} or nil ~= nil)

-- TODO: test tables, closures, branches, loops