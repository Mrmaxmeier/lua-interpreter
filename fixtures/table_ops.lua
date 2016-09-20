local table1 = {}
local copied = table1
assert(table1 == copied)
assert(table1 ~= {})
table1["key"] = "value"
assert(table1["key"] == "value")
assert(copied["key"] == "value")


local t = {}
t[3] = "number_three"
t["3"] = "string_three"
assert(t["3"] == "string_three")
assert(t[3] == "number_three")

t[{}] = "lost memory"
assert(t[{}] == nil)

t[table1] = "accessable memory"
table1["modified"] = true
assert(t[copied] == "accessable memory")