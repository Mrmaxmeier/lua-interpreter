local outside = 42

function a()
    local inside = 1337
    print("inside a")
    return inside, 9
end

print("outside a")
local from_inside, from_inside2 = a()
assert(from_inside == 1337)
assert(from_inside2 == 9)
assert(outside == 42)
print("after a")